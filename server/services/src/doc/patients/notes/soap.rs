use crate::app_state::AppState;
use crate::doc::patients::notes::helper::process_notes_events;
use crate::doc::syncs::sync;
use crate::method::convention::ErrorData;
use chrono::Utc;
use cosmo_store::types::event_read_range::EventsReadRange;
use cosmo_store::types::expected_version::ExpectedVersion;
use cosmo_store_util::aggregate::make_handler;
use log::error;
use log::info;
use patient::domain::allergies::allergies_domain::AllergiesState;
use patient::domain::medication::medication_domain::MedicationsState;
use patient::domain::notes::note_aggregate::NOTE_AGGREGATE;
use patient::domain::notes::note_command::CreateNote;
use patient::domain::notes::note_command::NoteCommand;
use patient::domain::vital::vital_domain::VitalsState;
use patient::dto::notes::notes_types::NoteType;
use patient::dto::notes::notes_types::SOAPNote;
use patient::dto::notes::soap::soap_add::SoapAdd;
use serde_json::Value;
use utils::store_helper::note_store;
use uuid::Uuid;

pub async fn add_soap(
    params: SoapAdd,
    app_state: AppState,
    organization_id: String,
    user_id: String,
) -> Result<Value, ErrorData> {
    let store = note_store(&app_state.write_pool, &organization_id).await?;
    let stream_id = format!("Note::{}", Uuid::new_v4().as_simple());

    let mut medication_state: Vec<MedicationsState> = Vec::new();
    let mut allergies_state: Vec<AllergiesState> = Vec::new();

    let json_data_result: Option<Value> =
        sqlx::query_scalar::<_, Option<Value>>("SELECT data FROM vitals_table_state WHERE id = ?")
            .bind(params.vitals_id)
            .fetch_one(&app_state.read_pool.clone())
            .await?;

    let json_data = match json_data_result {
        Some(data) => data,
        None => serde_json::Value::Null,
    };

    let vitals_state: VitalsState = serde_json::from_value(json_data)
        .map_err(|e| anyhow::anyhow!("Failed to parse JSON: {}", e))?;

    for id in params.medications_id {
        let json_data_result: Option<Value> = sqlx::query_scalar::<_, Option<Value>>(
            "SELECT data FROM medication_table_state WHERE id = ?",
        )
        .bind(id)
        .fetch_one(&app_state.read_pool.clone())
        .await?;

        let json_data = match json_data_result {
            Some(data) => data,
            None => serde_json::Value::Null,
        };

        medication_state.push(
            serde_json::from_value(json_data)
                .map_err(|e| anyhow::anyhow!("Failed to parse JSON: {}", e))?,
        );
    }

    for id in params.allergies_id {
        let json_data_result: Option<Value> = sqlx::query_scalar::<_, Option<Value>>(
            "SELECT data FROM allergies_table_state WHERE id = ?",
        )
        .bind(id)
        .fetch_one(&app_state.read_pool.clone())
        .await?;

        let json_data = match json_data_result {
            Some(data) => data,
            None => serde_json::Value::Null,
        };

        allergies_state.push(
            serde_json::from_value(json_data)
                .map_err(|e| anyhow::anyhow!("Failed to parse JSON: {}", e))?,
        );
    }

    let command = CreateNote {
        id: Uuid::new_v4().as_simple().to_string(),
        created_by: user_id.clone(),
        updated_by: user_id.clone(),
        created_at: Utc::now(),
        last_updated: Utc::now(),
        org_id: organization_id.clone(),
        patient_id: params.patient_id.clone(),
        note: NoteType::SOAPNote(SOAPNote {
            chief_complaint: params.chief_complaint.clone(),
            subjective: params.subjective.clone(),
            medications: medication_state,
            allergies: allergies_state,
            mental_or_functional: params.mental_or_functional.clone(),
            vitals: vitals_state,
            objective: params.objective.clone(),
            assessment: params.assessment.clone(),
            plan: params.plan.clone(),
        }),
        note_state: params.note_state.clone(),
    };

    info!("Making handler to process note creation command...");
    let events = make_handler(
        &NOTE_AGGREGATE,
        &store,
        &NoteCommand::CreateNote(command.clone()),
        &stream_id,
        &EventsReadRange::AllEvents,
        &ExpectedVersion::Any,
    )
    .await?;

    info!("Processing note events...");
    let res =
        process_notes_events(app_state.read_pool.clone(), command.id, stream_id, events).await;

    info!("Synchronizing data...");
    let row = sync(
        params.last_updated_input,
        organization_id.clone(),
        app_state.read_pool.clone(),
    )
    .await?;

    match res {
        anyhow::Result::Ok(_) => {
            info!("Soap added successfully.");
            serde_json::to_value(row).map_err(ErrorData::from)
        }
        Err(err) => {
            error!("Error occurred while adding soap: {:#?}", err);
            Err(ErrorData {
                message: String::from("SOAP_NOT_ADDED"),
                data: Value::Null,
                code: -32600,
            })
        }
    }
}
