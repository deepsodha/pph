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
use patient::dto::notes::history_and_physical::history_and_physical_add::HistoryAndPhysicalAdd;
use patient::dto::notes::notes_types::HistoryAndPhysical;
use patient::dto::notes::notes_types::NoteType;
use serde_json::Value;
use utils::store_helper::note_store;
use uuid::Uuid;

pub async fn add_historyandphysical(
    params: HistoryAndPhysicalAdd,
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
        note: NoteType::HistoryAndPhysical(Box::new(HistoryAndPhysical {
            chief_complaint: params.chief_complaint.clone(),
            history_of_present_illness: params.history_of_present_illness.clone(),
            past_medical_history: params.past_medical_history.clone(),
            past_surgical_history: params.past_surgical_history.clone(),
            family_history: params.family_history.clone(),
            social_history: params.social_history.clone(),
            obstetric_and_pregnancy_history: params.obstetric_and_pregnancy_history.clone(),
            hospitalizations: params.hospitalizations.clone(),
            implantable_devices: params.implantable_devices.clone(),
            review_of_systems: params.review_of_systems.clone(),
            medications: medication_state,
            allergies: allergies_state,
            mental_or_functional: params.mental_or_functional.clone(),
            vitals: vitals_state,
            exam: params.exam.clone(),
            assessment: params.assessment.clone(),
            plan: params.plan.clone(),
            minor_procedures: params.minor_procedures.clone(),
            goals: params.goals.clone(),
            health_concerns: params.health_concerns.clone(),
        })),
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

    info!("Processing HistoryAndPhysical events...");
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
            info!("HistoryAndPhysical added successfully.");
            serde_json::to_value(row).map_err(ErrorData::from)
        }
        Err(err) => {
            error!("Error occurred while adding HistoryAndPhysical: {:#?}", err);
            Err(ErrorData {
                message: String::from("HISTORYANDPHYSICAL_NOT_ADDED"),
                data: Value::Null,
                code: -32600,
            })
        }
    }
}
