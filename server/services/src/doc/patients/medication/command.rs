use crate::{
    app_state::AppState,
    doc::{patients::get_patient_id::patient_id_fetch, syncs::sync},
    method::convention::ErrorData,
};
use anyhow::Result;
use common::stream::Stream;
use cosmo_store::types::{event_read_range::EventsReadRange, expected_version::ExpectedVersion};
use cosmo_store_util::aggregate::make_handler;
use log::{error, info};
use patient::domain::medication::{
    medication_aggregate::MEDICATIONS_AGGREGATE,
    medication_commands::{
        CreateMedication, DeleteMedication, MedicationsCommand, UpdateMedication,
    },
};
use patient::dto::medication::{
    medication_add::MedicationsAdd, medication_delete::MedicationDelete,
    medication_update::MedicationUpdate,
};
use serde_json::Value;
use sqlx::types::chrono::Utc;
use utils::store_helper::patient_store;
use uuid::Uuid;

use super::helper::process_medication_events;
pub async fn add_medication(
    params: MedicationsAdd,
    app_state: AppState,
    organization_id: String,
    user_id: String,
) -> Result<Value, ErrorData> {
    let patient_id = patient_id_fetch(app_state.clone(), params.patient_id.clone()).await?;

    if patient_id {
        info!("Adding medication...");

        let store = patient_store(&app_state.write_pool, &organization_id).await?;
        let stream_id = format!("Medications::{}", Uuid::new_v4().as_simple());
        let command = CreateMedication {
            id: Uuid::new_v4().as_simple().to_string(),
            org_id: organization_id.clone(),
            patient_id: params.patient_id,
            created_by: user_id.clone(),
            updated_by: user_id.clone(),
            created_at: Utc::now(),
            last_updated: Utc::now(),
            status: params.status,
            drug: params.drug,
            instruction: params.instruction,
            comments: params.comments,
        };
        let events = make_handler(
            &MEDICATIONS_AGGREGATE,
            &store,
            &MedicationsCommand::CreateMedication(command.clone()),
            &stream_id,
            &EventsReadRange::AllEvents,
            &ExpectedVersion::Any,
        )
        .await?;
        let res =
            process_medication_events(app_state.read_pool.clone(), command.id, stream_id, events)
                .await;

        let row = sync(
            params.last_updated_input,
            organization_id.clone(),
            app_state.read_pool.clone(),
        )
        .await?;

        match res {
            anyhow::Result::Ok(_) => {
                info!("Medication added successfully.");
                serde_json::to_value(row).map_err(ErrorData::from)
            }
            Err(err) => {
                error!("{:#?}", err);
                error!("Failed to add medication.");
                Err(ErrorData {
                    message: String::from("MEDICATION_NOT_ADDED"),
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

pub async fn update_medication(
    params: MedicationUpdate,
    app_state: AppState,
    organization_id: String,
    user_id: String,
) -> Result<Value, ErrorData> {
    let patient_id = patient_id_fetch(app_state.clone(), params.input.patient_id.clone()).await?;

    if patient_id {
        info!("Updating medication...");

        let store = patient_store(&app_state.write_pool, &organization_id).await?;
        let medication_db = sqlx::query_as::<_, Stream>(
            "SELECT id,stream_id,version,(data->>'created_by') AS created_by, (data->>'created_at') AS created_at,data FROM medication_table_state WHERE id = ?",
        )
        .bind(params.id.clone())
        .fetch_optional(&app_state.read_pool.clone())
        .await?;

        let created_by = medication_db
            .as_ref()
            .map(|org| org.created_by.clone())
            .unwrap_or_default();
        let stream_id = medication_db
            .as_ref()
            .map(|org| org.stream_id.clone())
            .unwrap_or_default();
        let created_at = medication_db
            .as_ref()
            .map(|org| org.created_at)
            .unwrap_or_default();
        let command = UpdateMedication {
            id: params.id,
            org_id: organization_id.clone(),
            patient_id: params.input.patient_id,
            created_by,
            updated_by: user_id,
            created_at,
            last_updated: Utc::now(),
            status: params.input.status,
            drug: params.input.drug,
            instruction: params.input.instruction,
            comments: params.input.comments,
        };
        let events = make_handler(
            &MEDICATIONS_AGGREGATE,
            &store,
            &MedicationsCommand::UpdateMedication(command.clone()),
            &stream_id,
            &EventsReadRange::AllEvents,
            &ExpectedVersion::Any,
        )
        .await?;
        let res =
            process_medication_events(app_state.read_pool.clone(), command.id, stream_id, events)
                .await;

        let row = sync(
            params.input.last_updated_input,
            organization_id.clone(),
            app_state.read_pool.clone(),
        )
        .await?;

        match res {
            anyhow::Result::Ok(_) => {
                info!("Medication updated successfully.");
                serde_json::to_value(row).map_err(ErrorData::from)
            }
            Err(err) => {
                error!("{:#?}", err);
                error!("Failed to update medication.");
                Err(ErrorData {
                    message: String::from("MEDICATION_NOT_UPDATED"),
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
pub async fn delete_medication(
    params: MedicationDelete,
    app_state: AppState,
    organization_id: String,
    user_id: String,
) -> Result<Value, ErrorData> {
    let patient_id = patient_id_fetch(app_state.clone(), params.patient_id.clone()).await?;

    if patient_id {
        info!("Deleting medication...");

        let store = patient_store(&app_state.write_pool, &organization_id).await?;
        let medication_db = sqlx::query_as::<_, Stream>(
        "SELECT id,stream_id,version,(data->>'created_by') AS created_by, (data->>'created_at') AS created_at,data FROM medication_table_state WHERE id = ?",
    )
    .bind(params.id.clone())
    .fetch_optional(&app_state.read_pool.clone())
    .await?;

        let created_by = medication_db
            .as_ref()
            .map(|org| org.created_by.clone())
            .unwrap_or_default();
        let stream_id = medication_db
            .as_ref()
            .map(|org| org.stream_id.clone())
            .unwrap_or_default();
        let created_at = medication_db
            .as_ref()
            .map(|org| org.created_at)
            .unwrap_or_default();
        let command = DeleteMedication {
            id: params.id,
            org_id: organization_id.clone(),
            patient_id: params.patient_id,
            created_by,
            updated_by: user_id,
            created_at,
            last_updated: Utc::now(),
        };
        let events = make_handler(
            &MEDICATIONS_AGGREGATE,
            &store,
            &MedicationsCommand::DeleteMedication(command.clone()),
            &stream_id,
            &EventsReadRange::AllEvents,
            &ExpectedVersion::Any,
        )
        .await?;
        let res =
            process_medication_events(app_state.read_pool.clone(), command.id, stream_id, events)
                .await;

        let row = sync(
            params.last_updated_input,
            organization_id.clone(),
            app_state.read_pool.clone(),
        )
        .await?;

        match res {
            anyhow::Result::Ok(_) => {
                info!("Medication deleted successfully.");
                serde_json::to_value(row).map_err(ErrorData::from)
            }
            Err(err) => {
                error!("{:#?}", err);
                error!("Failed to delete medication.");
                Err(ErrorData {
                    message: String::from("MEDICATION_NOT_DELETED"),
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
