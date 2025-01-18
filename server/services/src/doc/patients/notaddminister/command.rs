use super::helper::process_not_administer_events;
use crate::app_state::AppState;
use crate::doc::patients::get_patient_id::patient_id_fetch;
use crate::doc::syncs::sync;
use crate::method::convention::ErrorData;
use anyhow::Result;
use common::stream::Stream;
use cosmo_store::types::{event_read_range::EventsReadRange, expected_version::ExpectedVersion};
use cosmo_store_util::aggregate::make_handler;
use log::{error, info};
use patient::domain::immunizations::notadministered::notadministered_commands::DeleteNotAdministered;
use patient::domain::immunizations::notadministered::{
    notadministered_aggregate::NOT_ADMINISTER_AGGREGATE,
    notadministered_commands::{
        CreateNotAdministered, NotAdministerCommand, UpdateNotAdministered,
    },
};
use patient::dto::immunization::not_administered_types::{
    NotAdministeredAdd, NotAdministeredDelete, NotAdministeredUpdate,
};
use serde_json::Value;
use sqlx::types::chrono::Utc;
use utils::store_helper::patient_store;
use uuid::Uuid;

pub async fn add_not_administer(
    params: NotAdministeredAdd,
    app_state: AppState,
    organization_id: String,
    user_id: String,
) -> Result<Value, ErrorData> {
    let patient_id = patient_id_fetch(app_state.clone(), params.patient_id.clone()).await?;
    if patient_id {
        info!("Adding not administered record...");

        let store = patient_store(&app_state.write_pool, &organization_id).await?;
        let stream_id = format!("NotAdminister::{}", Uuid::new_v4().as_simple());
        let command = CreateNotAdministered {
            id: Uuid::new_v4().as_simple().to_string(),
            org_id: organization_id.clone(),
            created_by: user_id.clone(),
            updated_by: user_id.clone(),
            created_at: Utc::now(),
            last_updated: Utc::now(),
            patient_id: params.patient_id,
            vaccine: params.vaccine,
            types: params.types,
            recorded: params.recorded,
            reason_for_non_administration: params.reason_for_non_administration,
            comments: params.comments,
        };

        info!("Making handler to process not administered creation command...");
        let events = make_handler(
            &NOT_ADMINISTER_AGGREGATE,
            &store,
            &NotAdministerCommand::CreateNotAdministered(command.clone()),
            &stream_id,
            &EventsReadRange::AllEvents,
            &ExpectedVersion::Any,
        )
        .await?;

        info!("Processing not administered events...");
        let res = process_not_administer_events(
            app_state.read_pool.clone(),
            command.id,
            stream_id,
            events,
        )
        .await;

        info!("Synchronizing data...");
        let row = sync(
            params.last_updated_input,
            organization_id.clone(),
            app_state.read_pool.clone(),
        )
        .await?;

        match res {
            anyhow::Result::Ok(_) => {
                info!("Not administered record added successfully.");
                serde_json::to_value(row).map_err(ErrorData::from)
            }
            Err(err) => {
                error!(
                    "Error occurred while adding not administered record: {:#?}",
                    err
                );
                Err(ErrorData {
                    message: String::from("NOTADMINISTER_NOT_ADDED"),
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

pub async fn update_not_administer(
    params: NotAdministeredUpdate,
    app_state: AppState,
    organization_id: String,
    user_id: String,
) -> Result<Value, ErrorData> {
    let patient_id = patient_id_fetch(app_state.clone(), params.input.patient_id.clone()).await?;
    if patient_id {
        info!("Updating not administered record...");

        let store = patient_store(&app_state.write_pool, &organization_id).await?;

        // Querying not administered record from the database
        let notadminister_db = sqlx::query_as::<_, Stream>(
            "SELECT id,stream_id,version,(data->>'created_by') AS created_by, (data->>'created_at') AS created_at,data FROM notadminister_table_state WHERE id = ?",
        )
        .bind(params.id.clone())
        .fetch_optional(&app_state.read_pool.clone())
        .await?;

        let created_by = notadminister_db
            .as_ref()
            .map(|org| org.created_by.clone())
            .unwrap_or_default();
        let stream_id = notadminister_db
            .as_ref()
            .map(|org| org.stream_id.clone())
            .unwrap_or_default();
        let created_at = notadminister_db
            .as_ref()
            .map(|org| org.created_at)
            .unwrap_or_default();

        let command = UpdateNotAdministered {
            id: params.id,
            created_by,
            updated_by: user_id,
            created_at,
            last_updated: Utc::now(),
            org_id: organization_id.clone(),
            patient_id: params.input.patient_id,
            vaccine: params.input.vaccine,
            types: params.input.types,
            recorded: params.input.recorded,
            reason_for_non_administration: params.input.reason_for_non_administration,
            comments: params.input.comments,
        };

        info!("Making handler to process not administered update command...");
        let events = make_handler(
            &NOT_ADMINISTER_AGGREGATE,
            &store,
            &NotAdministerCommand::UpdateNotAdministered(command.clone()),
            &stream_id,
            &EventsReadRange::AllEvents,
            &ExpectedVersion::Any,
        )
        .await?;

        info!("Processing not administered events...");
        let res = process_not_administer_events(
            app_state.read_pool.clone(),
            command.id,
            stream_id,
            events,
        )
        .await;

        info!("Synchronizing data...");
        let row = sync(
            params.input.last_updated_input,
            organization_id.clone(),
            app_state.read_pool.clone(),
        )
        .await?;

        match res {
            anyhow::Result::Ok(_) => {
                info!("Not administered record updated successfully.");
                serde_json::to_value(row).map_err(ErrorData::from)
            }
            Err(err) => {
                error!(
                    "Error occurred while updating not administered record: {:#?}",
                    err
                );
                Err(ErrorData {
                    message: String::from("NOTADMINISTER_NOT_UPDATED"),
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

pub async fn delete_not_administer(
    params: NotAdministeredDelete,
    app_state: AppState,
    organization_id: String,
    user_id: String,
) -> Result<Value, ErrorData> {
    let patient_id = patient_id_fetch(app_state.clone(), params.patient_id.clone()).await?;
    if patient_id {
        info!("Deleting not administered record...");

        let store = patient_store(&app_state.write_pool, &organization_id).await?;

        // Querying not administered record from the database
        let notadminister_db = sqlx::query_as::<_, Stream>(
            "SELECT id,stream_id,version,(data->>'created_by') AS created_by, (data->>'created_at') AS created_at,data FROM notadminister_table_state WHERE id = ?",
        )
        .bind(params.id.clone())
        .fetch_optional(&app_state.read_pool.clone())
        .await?;

        let created_by = notadminister_db
            .as_ref()
            .map(|org| org.created_by.clone())
            .unwrap_or_default();
        let stream_id = notadminister_db
            .as_ref()
            .map(|org| org.stream_id.clone())
            .unwrap_or_default();
        let created_at = notadminister_db
            .as_ref()
            .map(|org| org.created_at)
            .unwrap_or_default();

        let command = DeleteNotAdministered {
            id: params.id,
            created_by,
            updated_by: user_id,
            created_at,
            last_updated: Utc::now(),
            org_id: organization_id.clone(),
            patient_id: params.patient_id,
        };

        info!("Making handler to process not administered deletion command...");
        let events = make_handler(
            &NOT_ADMINISTER_AGGREGATE,
            &store,
            &NotAdministerCommand::DeleteNotAdministered(command.clone()),
            &stream_id,
            &EventsReadRange::AllEvents,
            &ExpectedVersion::Any,
        )
        .await?;

        info!("Processing not administered events...");
        let res = process_not_administer_events(
            app_state.read_pool.clone(),
            command.id,
            stream_id,
            events,
        )
        .await;

        info!("Synchronizing data...");
        let row = sync(
            params.last_updated_input,
            organization_id.clone(),
            app_state.read_pool.clone(),
        )
        .await?;

        match res {
            anyhow::Result::Ok(_) => {
                info!("Not administered record deleted successfully.");
                serde_json::to_value(row).map_err(ErrorData::from)
            }
            Err(err) => {
                error!(
                    "Error occurred while deleting not administered record: {:#?}",
                    err
                );
                Err(ErrorData {
                    message: String::from("NOTADMINISTER_NOT_DELETED"),
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
