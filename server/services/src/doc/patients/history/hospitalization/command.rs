use super::helper::process_hospitalization_events;
use crate::{
    app_state::AppState,
    doc::{patients::get_patient_id::patient_id_fetch, syncs::sync},
    method::convention::ErrorData,
};
use chrono::Utc;
use common::stream::Stream;
use cosmo_store::types::{event_read_range::EventsReadRange, expected_version::ExpectedVersion};
use cosmo_store_util::aggregate::make_handler;
use log::{debug, error, info};
use patient::{
    domain::history::hospitalization::{
        hospitalization_aggregate::HOSPITALIZATION_AGGREGATE,
        hospitalization_commands::{
            CreateHospitalization, DeleteHospitalization, HospitalizationCommand,
            UpdateHospitalization,
        },
    },
    dto::history::hospitalization::{
        hospitalization_add::HospitalizationAdd, hospitalization_delete::HospitalizationDelete,
        hospitalization_update::HospitalizationUpdate,
    },
};
use serde_json::Value;
use utils::store_helper::hospitalization_store;
use uuid::Uuid;

pub async fn add_hospitalization(
    params: HospitalizationAdd,
    app_state: AppState,
    organization_id: String,
    user_id: String,
) -> Result<Value, ErrorData> {
    let patient_id = patient_id_fetch(app_state.clone(), params.patient_id.clone()).await?;
    if patient_id {
        info!("Adding hospitalization...");

        let store = hospitalization_store(&app_state.write_pool, &organization_id).await?;
        let stream_id = format!("Hospitalization::{}", uuid::Uuid::new_v4().as_simple());
        let command = CreateHospitalization {
            id: Uuid::new_v4().as_simple().to_string(),
            org_id: organization_id.clone(),
            patient_id: params.patient_id,
            created_by: user_id.clone(),
            updated_by: user_id.clone(),
            created_at: Utc::now(),
            last_updated: Utc::now(),
            admission_date: params.admission_date,
            related_to: params.related_to,
            status: params.status,
            length_of_stay: params.length_of_stay,
            procedure: params.procedure,
            comments: params.comments,
        };

        debug!("Creating hospitalization command: {:?}", &command);

        let events = make_handler(
            &HOSPITALIZATION_AGGREGATE,
            &store,
            &HospitalizationCommand::CreateHospitalization(command.clone()),
            &stream_id,
            &EventsReadRange::AllEvents,
            &ExpectedVersion::Any,
        )
        .await?;

        debug!("Events created: {:?}", &events);

        let res = process_hospitalization_events(
            app_state.read_pool.clone(),
            command.id,
            stream_id,
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
            anyhow::Result::Ok(_) => {
                info!("Hospitalization added successfully.");
                serde_json::to_value(row).map_err(ErrorData::from)
            }
            Err(err) => {
                error!("Error adding hospitalization: {:#?}", err);
                Err(ErrorData {
                    message: String::from("HOSPITALIZATION_NOT_ADDED"),
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

pub async fn update_hospitalization(
    params: HospitalizationUpdate,
    app_state: AppState,
    organization_id: String,
    user_id: String,
) -> Result<Value, ErrorData> {
    let patient_id = patient_id_fetch(app_state.clone(), params.input.patient_id.clone()).await?;
    if patient_id {
        info!("Updating hospitalization...");

        let store = hospitalization_store(&app_state.write_pool, &organization_id).await?;
        let hospitalization_db = sqlx::query_as::<_, Stream>(
        "SELECT id,stream_id,version,(data->>'created_by') AS created_by, (data->>'created_at') AS created_at,data FROM hospitalization_table_state WHERE id = ?",
    )
    .bind(params.id.clone())
    .fetch_optional(&app_state.read_pool.clone())
    .await?;

        let created_by = hospitalization_db
            .as_ref()
            .map(|org| org.created_by.clone())
            .unwrap_or_default();
        let stream_id = hospitalization_db
            .as_ref()
            .map(|org| org.stream_id.clone())
            .unwrap_or_default();
        let created_at = hospitalization_db
            .as_ref()
            .map(|org| org.created_at)
            .unwrap_or_default();
        let command = UpdateHospitalization {
            id: params.id,
            created_by,
            updated_by: user_id.clone(),
            created_at,
            last_updated: Utc::now(),
            org_id: organization_id.clone(),
            patient_id: params.input.patient_id,
            comments: params.input.comments,
            admission_date: params.input.admission_date,
            related_to: params.input.related_to,
            status: params.input.status,
            length_of_stay: params.input.length_of_stay,
            procedure: params.input.procedure,
        };

        debug!("Updating hospitalization command: {:?}", &command);

        let events = make_handler(
            &HOSPITALIZATION_AGGREGATE,
            &store,
            &HospitalizationCommand::UpdateHospitalization(command.clone()),
            &stream_id,
            &EventsReadRange::AllEvents,
            &ExpectedVersion::Any,
        )
        .await?;

        debug!("Events created: {:?}", &events);

        let res = process_hospitalization_events(
            app_state.read_pool.clone(),
            command.id,
            stream_id,
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
            anyhow::Result::Ok(_) => {
                info!("Hospitalization updated successfully.");
                serde_json::to_value(row).map_err(ErrorData::from)
            }
            Err(err) => {
                error!("Error updating hospitalization: {:#?}", err);
                Err(ErrorData {
                    message: String::from("HOSPITALIZATION_NOT_UPDATED"),
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

pub async fn delete_hospitalization(
    params: HospitalizationDelete,
    app_state: AppState,
    organization_id: String,
    user_id: String,
) -> Result<Value, ErrorData> {
    let patient_id = patient_id_fetch(app_state.clone(), params.patient_id.clone()).await?;
    if patient_id {
        info!("Deleting hospitalization...");

        let store = hospitalization_store(&app_state.write_pool, &organization_id).await?;
        let hospitalization_db = sqlx::query_as::<_, Stream>(
        "SELECT id,stream_id,version,(data->>'created_by') AS created_by, (data->>'created_at') AS created_at,data FROM hospitalization_table_state WHERE id = ?",
    )
    .bind(params.id.clone())
    .fetch_optional(&app_state.read_pool.clone())
    .await?;

        let created_by = hospitalization_db
            .as_ref()
            .map(|org| org.created_by.clone())
            .unwrap_or_default();
        let stream_id = hospitalization_db
            .as_ref()
            .map(|org| org.stream_id.clone())
            .unwrap_or_default();
        let created_at = hospitalization_db
            .as_ref()
            .map(|org| org.created_at)
            .unwrap_or_default();

        let command = DeleteHospitalization {
            id: params.id,
            patient_id: params.patient_id,
            org_id: organization_id.clone(),
            created_by,
            updated_by: user_id.clone(),
            created_at,
            last_updated: Utc::now(),
        };

        debug!("Deleting hospitalization command: {:?}", &command);

        let events = make_handler(
            &HOSPITALIZATION_AGGREGATE,
            &store,
            &HospitalizationCommand::DeleteHospitalization(command.clone()),
            &stream_id,
            &EventsReadRange::AllEvents,
            &ExpectedVersion::Any,
        )
        .await?;

        debug!("Events created: {:?}", &events);

        let res = process_hospitalization_events(
            app_state.read_pool.clone(),
            command.id,
            stream_id,
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
            anyhow::Result::Ok(_) => {
                info!("Hospitalization deleted successfully.");
                serde_json::to_value(row).map_err(ErrorData::from)
            }
            Err(err) => {
                error!("Error deleting hospitalization: {:#?}", err);
                Err(ErrorData {
                    message: String::from("HOSPITALIZATION_NOT_DELETED"),
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
