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
use patient::domain::vital::{
    vital_aggregate::VITALS_AGGREGATE,
    vital_commands::{CreateVitals, DeleteVitals, UpdateVitals, VitalsCommand},
};
use patient::dto::vital::{
    vital_add::VitalsAdd, vital_delete::VitalsDelete, vital_update::VitalsUpdate,
};
use serde_json::Value;
use sqlx::types::chrono::Utc;
use utils::store_helper::patient_store;
use uuid::Uuid;

use super::helper::process_vitals_events;
pub async fn add_vitals(
    params: VitalsAdd,
    app_state: AppState,
    organization_id: String,
    user_id: String,
) -> Result<Value, ErrorData> {
    let patient_id = patient_id_fetch(app_state.clone(), params.patient_id.clone()).await?;
    if patient_id {
        info!("Adding vitals data");

        let store = patient_store(&app_state.write_pool, &organization_id).await?;
        let stream_id = format!("Vitals::{}", Uuid::new_v4().as_simple());
        let command = CreateVitals {
            id: Uuid::new_v4().as_simple().to_string(),
            org_id: organization_id.clone(),
            patient_id: params.patient_id,
            doctor_id: user_id.clone(),
            created_by: user_id.clone(),
            updated_by: user_id.clone(),
            created_at: Utc::now(),
            last_updated: Utc::now(),
            date_time: params.date_time,
            blood_pressure: params.blood_pressure,
            heart_rate: params.heart_rate,
            height: params.height,
            weight: params.weight,
            bmi: params.bmi,
            temperature: params.temperature,
            comments: params.comments,
        };

        info!("Creating command for adding vitals");

        let events = make_handler(
            &VITALS_AGGREGATE,
            &store,
            &VitalsCommand::CreateVitals(command.clone()),
            &stream_id,
            &EventsReadRange::AllEvents,
            &ExpectedVersion::Any,
        )
        .await?;

        info!("Handling events for adding vitals");

        let res =
            process_vitals_events(app_state.read_pool.clone(), command.id, stream_id, events).await;

        info!("Processing vitals events");

        let row = sync(
            params.last_updated_input,
            organization_id.clone(),
            app_state.read_pool.clone(),
        )
        .await?;

        match res {
            anyhow::Result::Ok(_) => {
                info!("Vitals added successfully");
                serde_json::to_value(row).map_err(ErrorData::from)
            }
            Err(err) => {
                error!("Error adding vitals: {:#?}", err);
                Err(ErrorData {
                    message: String::from("VITALS_NOT_ADDED"),
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
pub async fn update_vitals(
    params: VitalsUpdate,
    app_state: AppState,
    organization_id: String,
    user_id: String,
) -> Result<Value, ErrorData> {
    let patient_id = patient_id_fetch(app_state.clone(), params.input.patient_id.clone()).await?;
    if patient_id {
        info!("Updating vitals data");

        let store = patient_store(&app_state.write_pool, &organization_id).await?;
        let vitals_db = sqlx::query_as::<_, Stream>(
        "SELECT id,stream_id,version,(data->>'created_by') AS created_by, (data->>'created_at') AS created_at,data FROM vitals_table_state WHERE id = ?",
    )
    .bind(params.id.clone())
    .fetch_optional(&app_state.read_pool.clone())
    .await?;

        let created_by = vitals_db
            .as_ref()
            .map(|org| org.created_by.clone())
            .unwrap_or_default();
        let stream_id = vitals_db
            .as_ref()
            .map(|org| org.stream_id.clone())
            .unwrap_or_default();
        let created_at = vitals_db
            .as_ref()
            .map(|org| org.created_at)
            .unwrap_or_default();

        let command = UpdateVitals {
            id: params.id,
            doctor_id: user_id.clone(),
            created_by,
            updated_by: user_id,
            created_at,
            last_updated: Utc::now(),
            org_id: organization_id.clone(),
            patient_id: params.input.patient_id,
            date_time: params.input.date_time,
            blood_pressure: params.input.blood_pressure,
            heart_rate: params.input.heart_rate,
            height: params.input.height,
            weight: params.input.weight,
            bmi: params.input.bmi,
            temperature: params.input.temperature,
            comments: params.input.comments,
        };

        info!("Creating command for updating vitals");

        let events = make_handler(
            &VITALS_AGGREGATE,
            &store,
            &VitalsCommand::UpdateVitals(command.clone()),
            &stream_id,
            &EventsReadRange::AllEvents,
            &ExpectedVersion::Any,
        )
        .await?;

        info!("Handling events for updating vitals");

        let res =
            process_vitals_events(app_state.read_pool.clone(), command.id, stream_id, events).await;

        info!("Processing vitals events");

        let row = sync(
            params.input.last_updated_input,
            organization_id.clone(),
            app_state.read_pool.clone(),
        )
        .await?;

        match res {
            anyhow::Result::Ok(_) => {
                info!("Vitals updated successfully");
                serde_json::to_value(row).map_err(ErrorData::from)
            }
            Err(err) => {
                error!("Error updating vitals: {:#?}", err);
                Err(ErrorData {
                    message: String::from("VITALS_NOT_UPDATED"),
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

pub async fn delete_vitals(
    params: VitalsDelete,
    app_state: AppState,
    organization_id: String,
    user_id: String,
) -> Result<Value, ErrorData> {
    let patient_id = patient_id_fetch(app_state.clone(), params.patient_id.clone()).await?;
    if patient_id {
        info!("Deleting vitals data");

        let store = patient_store(&app_state.write_pool, &organization_id).await?;
        let vitals_db = sqlx::query_as::<_, Stream>(
        "SELECT id,stream_id,version,(data->>'created_by') AS created_by, (data->>'created_at') AS created_at,data FROM vitals_table_state WHERE id = ?",
    )
    .bind(params.id.clone())
    .fetch_optional(&app_state.read_pool.clone())
    .await?;

        let created_by = vitals_db
            .as_ref()
            .map(|org| org.created_by.clone())
            .unwrap_or_default();
        let stream_id = vitals_db
            .as_ref()
            .map(|org| org.stream_id.clone())
            .unwrap_or_default();
        let created_at = vitals_db
            .as_ref()
            .map(|org| org.created_at)
            .unwrap_or_default();
        let command = DeleteVitals {
            id: params.id,
            doctor_id: user_id.clone(),
            org_id: organization_id.clone(),
            patient_id: params.patient_id,
            created_by,
            updated_by: user_id,
            created_at,
            last_updated: Utc::now(),
        };

        info!("Creating command for deleting vitals");

        let events = make_handler(
            &VITALS_AGGREGATE,
            &store,
            &VitalsCommand::DeleteVitals(command.clone()),
            &stream_id,
            &EventsReadRange::AllEvents,
            &ExpectedVersion::Any,
        )
        .await?;

        info!("Handling events for deleting vitals");

        let res =
            process_vitals_events(app_state.read_pool.clone(), command.id, stream_id, events).await;

        info!("Processing vitals events");

        let row = sync(
            params.last_updated_input,
            organization_id.clone(),
            app_state.read_pool.clone(),
        )
        .await?;

        match res {
            anyhow::Result::Ok(_) => {
                info!("Vitals deleted successfully");
                serde_json::to_value(row).map_err(ErrorData::from)
            }
            Err(err) => {
                error!("Error deleting vitals: {:#?}", err);
                Err(ErrorData {
                    message: String::from("VITALS_NOT_DELETED"),
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
