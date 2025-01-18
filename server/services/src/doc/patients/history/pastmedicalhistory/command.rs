use super::helper::process_pastmedicalhistory_events;
use crate::{
    app_state::AppState,
    doc::{patients::get_patient_id::patient_id_fetch, syncs::sync},
    method::convention::ErrorData,
};
use chrono::Utc;
use common::stream::Stream;
use cosmo_store::types::{event_read_range::EventsReadRange, expected_version::ExpectedVersion};
use cosmo_store_util::aggregate::make_handler;
use log::{error, info};
use patient::{
    domain::history::pastmedical_history::{
        pastmedical_history_aggregate::PASTMEDICALHISTORY_AGGREGATE,
        pastmedical_history_commands::{
            CreatePastMedicalHistory, DeletePastMedicalHistory, PastMedicalHistoryCommand,
            UpdatePastMedicalHistory,
        },
    },
    dto::history::pastmedical_history::{
        pastmedical_add::PastMedicalHistoryAdd, pastmedical_delete::PastMedicalHistoryDelete,
        pastmedical_update::PastMedicalHistoryUpdate,
    },
};
use serde_json::Value;
use utils::store_helper::pastmedicalhistory_store;
use uuid::Uuid;
pub async fn add_past_medical_history(
    params: PastMedicalHistoryAdd,
    app_state: AppState,
    organization_id: String,
    user_id: String,
) -> Result<Value, ErrorData> {
    let patient_id = patient_id_fetch(app_state.clone(), params.patient_id.clone()).await?;
    if patient_id {
        info!("Adding Past Medical History...");

        let store = pastmedicalhistory_store(&app_state.write_pool, &organization_id).await?;
        let stream_id = format!("PastMedicalHistory::{}", uuid::Uuid::new_v4().as_simple());
        let command = CreatePastMedicalHistory {
            id: Uuid::new_v4().as_simple().to_string(),
            org_id: organization_id.clone(),
            patient_id: params.patient_id,
            created_by: user_id.clone(),
            updated_by: user_id.clone(),
            created_at: Utc::now(),
            last_updated: Utc::now(),
            blood_type: params.blood_type,
            head: params.head,
            respiratory: params.respiratory,
            musculoskeletal: params.musculoskeletal,
            endocrine: params.endocrine,
            eyes: params.eyes,
            gastrointestinal: params.gastrointestinal,
            skin: params.skin,
            ears: params.ears,
            noses: params.noses,
            neurological: params.neurological,
            heme: params.heme,
            mouth: params.mouth,
            infectious: params.infectious,
            cardiovascular: params.cardiovascular,
            genitourinary: params.genitourinary,
            psychiatric: params.psychiatric,
            comments: params.comments,
        };
        let events = make_handler(
            &PASTMEDICALHISTORY_AGGREGATE,
            &store,
            &PastMedicalHistoryCommand::CreatePastMedicalHistory(command.clone()),
            &stream_id,
            &EventsReadRange::AllEvents,
            &ExpectedVersion::Any,
        )
        .await?;

        let res = process_pastmedicalhistory_events(
            app_state.read_pool.clone(),
            command.id.clone(),
            stream_id.clone(),
            events,
        )
        .await;

        let row = sync(
            params.last_updated_input,
            organization_id.clone(),
            app_state.read_pool.clone(),
        )
        .await?;

        match res {
            Ok(_) => {
                info!("Past Medical History added successfully.");
                serde_json::to_value(row).map_err(ErrorData::from)
            }
            Err(err) => {
                error!("{:#?}", err);
                Err(ErrorData {
                    message: String::from("PASTMEDICALHISTORY_NOT_ADDED"),
                    data: Value::Null,
                    code: -32600,
                })
            }
        }
    } else {
        Err(ErrorData {
            message: String::from("PATIENT_NOT_FOUND"),
            data: Value::Null,
            code: -32600,
        })
    }
}

pub async fn update_past_medical_history(
    params: PastMedicalHistoryUpdate,
    app_state: AppState,
    organization_id: String,
    user_id: String,
) -> Result<Value, ErrorData> {
    let patient_id = patient_id_fetch(app_state.clone(), params.input.patient_id.clone()).await?;
    if patient_id {
        info!("Updating Past Medical History...");

        let store = pastmedicalhistory_store(&app_state.write_pool, &organization_id).await?;
        let pastmedicalhistory_db = sqlx::query_as::<_, Stream>(
        "SELECT id, stream_id, version, (data->>'created_by') AS created_by, (data->>'created_at') AS created_at, data FROM pastmedicalhistory_table_state WHERE id = ?",
    )
    .bind(params.id.clone())
    .fetch_optional(&app_state.read_pool.clone())
    .await?;

        let created_by = pastmedicalhistory_db
            .as_ref()
            .map(|org| org.created_by.clone())
            .unwrap_or_default();
        let stream_id = pastmedicalhistory_db
            .as_ref()
            .map(|org| org.stream_id.clone())
            .unwrap_or_default();
        let created_at = pastmedicalhistory_db
            .as_ref()
            .map(|org| org.created_at)
            .unwrap_or_default();
        let command = UpdatePastMedicalHistory {
            id: params.id.clone(),
            created_by,
            updated_by: user_id.clone(),
            created_at,
            last_updated: Utc::now(),
            org_id: organization_id.clone(),
            patient_id: params.input.patient_id,
            blood_type: params.input.blood_type,
            head: params.input.head,
            respiratory: params.input.respiratory,
            musculoskeletal: params.input.musculoskeletal,
            endocrine: params.input.endocrine,
            eyes: params.input.eyes,
            gastrointestinal: params.input.gastrointestinal,
            skin: params.input.skin,
            ears: params.input.ears,
            noses: params.input.noses,
            neurological: params.input.neurological,
            heme: params.input.heme,
            mouth: params.input.mouth,
            infectious: params.input.infectious,
            cardiovascular: params.input.cardiovascular,
            genitourinary: params.input.genitourinary,
            psychiatric: params.input.psychiatric,
            comments: params.input.comments,
        };
        let events = make_handler(
            &PASTMEDICALHISTORY_AGGREGATE,
            &store,
            &PastMedicalHistoryCommand::UpdatePastMedicalHistory(command.clone()),
            &stream_id,
            &EventsReadRange::AllEvents,
            &ExpectedVersion::Any,
        )
        .await?;

        let res = process_pastmedicalhistory_events(
            app_state.read_pool.clone(),
            command.id.clone(),
            stream_id.clone(),
            events,
        )
        .await;

        let row = sync(
            params.input.last_updated_input,
            organization_id.clone(),
            app_state.read_pool.clone(),
        )
        .await?;

        match res {
            Ok(_) => {
                info!("Past Medical History updated successfully.");
                serde_json::to_value(row).map_err(ErrorData::from)
            }
            Err(err) => {
                error!("{:#?}", err);
                Err(ErrorData {
                    message: String::from("PASTMEDICALHISTORY_NOT_UPDATED"),
                    data: Value::Null,
                    code: -32600,
                })
            }
        }
    } else {
        Err(ErrorData {
            message: String::from("PATIENT_NOT_FOUND"),
            data: Value::Null,
            code: -32600,
        })
    }
}

pub async fn delete_past_medical_history(
    params: PastMedicalHistoryDelete,
    app_state: AppState,
    organization_id: String,
    user_id: String,
) -> Result<Value, ErrorData> {
    let patient_id = patient_id_fetch(app_state.clone(), params.patient_id.clone()).await?;
    if patient_id {
        info!("Deleting Past Medical History...");

        let store = pastmedicalhistory_store(&app_state.write_pool, &organization_id).await?;
        let pastmedicalhistory_db = sqlx::query_as::<_, Stream>(
        "SELECT id, stream_id, version, (data->>'created_by') AS created_by, (data->>'created_at') AS created_at, data FROM pastmedicalhistory_table_state WHERE id = ?",
    )
    .bind(params.id.clone())
    .fetch_optional(&app_state.read_pool.clone())
    .await?;

        let created_by = pastmedicalhistory_db
            .as_ref()
            .map(|org| org.created_by.clone())
            .unwrap_or_default();
        let stream_id = pastmedicalhistory_db
            .as_ref()
            .map(|org| org.stream_id.clone())
            .unwrap_or_default();
        let created_at = pastmedicalhistory_db
            .as_ref()
            .map(|org| org.created_at)
            .unwrap_or_default();

        let command = DeletePastMedicalHistory {
            id: params.id.clone(),
            patient_id: params.patient_id.clone(),
            org_id: organization_id.clone(),
            created_by,
            updated_by: user_id.clone(),
            created_at,
            last_updated: Utc::now(),
        };
        let events = make_handler(
            &PASTMEDICALHISTORY_AGGREGATE,
            &store,
            &PastMedicalHistoryCommand::DeletePastMedicalHistory(command.clone()),
            &stream_id,
            &EventsReadRange::AllEvents,
            &ExpectedVersion::Any,
        )
        .await?;

        let res = process_pastmedicalhistory_events(
            app_state.read_pool.clone(),
            command.id.clone(),
            stream_id.clone(),
            events,
        )
        .await;

        let row = sync(
            params.last_updated_input,
            organization_id.clone(),
            app_state.read_pool.clone(),
        )
        .await?;

        match res {
            Ok(_) => {
                info!("Past Medical History deleted successfully.");
                serde_json::to_value(row).map_err(ErrorData::from)
            }
            Err(err) => {
                error!("{:#?}", err);
                Err(ErrorData {
                    message: String::from("PASTMEDICALHISTORY_NOT_DELETED"),
                    data: Value::Null,
                    code: -32600,
                })
            }
        }
    } else {
        Err(ErrorData {
            message: String::from("PATIENT_NOT_FOUND"),
            data: Value::Null,
            code: -32600,
        })
    }
}
