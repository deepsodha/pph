use super::helper::process_pastsurgicalhistory_events;
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
    domain::history::pastsurgical_history::{
        pastsurgical_history_aggregate::PASTSURGICALHISTORY_AGGREGATE,
        pastsurgical_history_commands::{
            CreatePastSurgicalHistory, DeletePastSurgicalHistory, PastSurgicalCommand,
            UpdatePastSurgicalHistory,
        },
    },
    dto::history::pastsurgical_history::{
        pastsurgicalhistory_add::PastSurgicalHistoryAdd,
        pastsurgicalhistory_delete::PastSurgicalHistoryDelete,
        pastsurgicalhistory_update::PastSurgicalHistoryUpdate,
    },
};
use serde_json::Value;
use utils::store_helper::pastsurgicalhistory_store;
use uuid::Uuid;

pub async fn add_past_surgical_history(
    params: PastSurgicalHistoryAdd,
    app_state: AppState,
    organization_id: String,
    user_id: String,
) -> Result<Value, ErrorData> {
    let patient_id = patient_id_fetch(app_state.clone(), params.patient_id.clone()).await?;
    if patient_id {
        info!("Adding past surgical history...");

        info!("Fetching past surgical history store...");
        let store = pastsurgicalhistory_store(&app_state.write_pool, &organization_id).await?;

        let stream_id = format!("PastSurgicalHistory::{}", uuid::Uuid::new_v4().as_simple());

        let command = CreatePastSurgicalHistory {
            id: Uuid::new_v4().as_simple().to_string(),
            org_id: organization_id.clone(),
            patient_id: params.patient_id,
            created_by: user_id.clone(),
            updated_by: user_id.clone(),
            created_at: Utc::now(),
            last_updated: Utc::now(),
            comments: params.comments,
            common_surgeries: params.common_surgeries,
        };

        info!("Making handler to process past surgical history creation command...");
        let events = make_handler(
            &PASTSURGICALHISTORY_AGGREGATE,
            &store,
            &PastSurgicalCommand::CreatePastSurgicalHistory(command.clone()),
            &stream_id,
            &EventsReadRange::AllEvents,
            &ExpectedVersion::Any,
        )
        .await?;

        info!("Processing past surgical history events...");
        let res = process_pastsurgicalhistory_events(
            app_state.read_pool.clone(),
            command.id,
            stream_id.clone(),
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
                info!("Past surgical history added successfully.");
                serde_json::to_value(row).map_err(ErrorData::from)
            }
            Err(err) => {
                error!(
                    "Error occurred while adding past surgical history: {:#?}",
                    err
                );
                Err(ErrorData {
                    message: String::from("PASTSURGICALHISTORY_NOT_ADDED"),
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

pub async fn update_past_surgical_history(
    params: PastSurgicalHistoryUpdate,
    app_state: AppState,
    organization_id: String,
    user_id: String,
) -> Result<Value, ErrorData> {
    let patient_id = patient_id_fetch(app_state.clone(), params.input.patient_id.clone()).await?;
    if patient_id {
        info!("Updating past surgical history...");

        info!("Fetching past surgical history store...");
        let store = pastsurgicalhistory_store(&app_state.write_pool, &organization_id).await?;

        info!("Querying past surgical history from the database...");
        let pastsurgicalhistory_db = sqlx::query_as::<_, Stream>(
            "SELECT id,stream_id,version,(data->>'created_by') AS created_by, (data->>'created_at') AS created_at,data FROM pastsurgicalhistory_table_state WHERE id = ?",
        )
        .bind(params.id.clone())
        .fetch_optional(&app_state.read_pool.clone())
        .await?;

        let created_by = pastsurgicalhistory_db
            .as_ref()
            .map(|org| org.created_by.clone())
            .unwrap_or_default();
        let stream_id = pastsurgicalhistory_db
            .as_ref()
            .map(|org| org.stream_id.clone())
            .unwrap_or_default();
        let created_at = pastsurgicalhistory_db
            .as_ref()
            .map(|org| org.created_at)
            .unwrap_or_default();

        let command = UpdatePastSurgicalHistory {
            id: params.id,
            created_by,
            updated_by: user_id,
            created_at,
            last_updated: Utc::now(),
            org_id: organization_id.clone(),
            patient_id: params.input.patient_id,
            comments: params.input.comments,
            common_surgeries: params.input.common_surgeries,
        };

        info!("Making handler to process past surgical history update command...");
        let events = make_handler(
            &PASTSURGICALHISTORY_AGGREGATE,
            &store,
            &PastSurgicalCommand::UpdatePastSurgicalHistory(command.clone()),
            &stream_id,
            &EventsReadRange::AllEvents,
            &ExpectedVersion::Any,
        )
        .await?;

        info!("Processing past surgical history events...");
        let res = process_pastsurgicalhistory_events(
            app_state.read_pool.clone(),
            command.id,
            stream_id.clone(),
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
                info!("Past surgical history updated successfully.");
                serde_json::to_value(row).map_err(ErrorData::from)
            }
            Err(err) => {
                error!(
                    "Error occurred while updating past surgical history: {:#?}",
                    err
                );
                Err(ErrorData {
                    message: String::from("PASTSURGICALHISTORY_NOT_UPDATED"),
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

pub async fn delete_past_surgical_history(
    params: PastSurgicalHistoryDelete,
    app_state: AppState,
    organization_id: String,
    user_id: String,
) -> Result<Value, ErrorData> {
    let patient_id = patient_id_fetch(app_state.clone(), params.patient_id.clone()).await?;
    if patient_id {
        info!("Deleting past surgical history...");

        info!("Fetching past surgical history store...");
        let store = pastsurgicalhistory_store(&app_state.write_pool, &organization_id).await?;

        info!("Querying past surgical history from the database...");
        let pastsurgicalhistory_db = sqlx::query_as::<_, Stream>(
        "SELECT id,stream_id,version,(data->>'created_by') AS created_by, (data->>'created_at') AS created_at,data FROM pastsurgicalhistory_table_state WHERE id = ?",
    )
    .bind(params.id.clone())
    .fetch_optional(&app_state.read_pool.clone())
    .await?;

        let created_by = pastsurgicalhistory_db
            .as_ref()
            .map(|org| org.created_by.clone())
            .unwrap_or_default();
        let stream_id = pastsurgicalhistory_db
            .as_ref()
            .map(|org| org.stream_id.clone())
            .unwrap_or_default();
        let created_at = pastsurgicalhistory_db
            .as_ref()
            .map(|org| org.created_at)
            .unwrap_or_default();

        let command = DeletePastSurgicalHistory {
            id: params.id,
            patient_id: params.patient_id,
            org_id: organization_id.clone(),
            created_by,
            updated_by: user_id,
            created_at,
            last_updated: Utc::now(),
        };

        info!("Making handler to process past surgical history delete command...");
        let events = make_handler(
            &PASTSURGICALHISTORY_AGGREGATE,
            &store,
            &PastSurgicalCommand::DeletePastSurgicalHistory(command.clone()),
            &stream_id,
            &EventsReadRange::AllEvents,
            &ExpectedVersion::Any,
        )
        .await?;

        info!("Processing past surgical history events...");
        let res = process_pastsurgicalhistory_events(
            app_state.read_pool.clone(),
            command.id,
            stream_id.clone(),
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
                info!("Past surgical history deleted successfully.");
                serde_json::to_value(row).map_err(ErrorData::from)
            }
            Err(err) => {
                error!(
                    "Error occurred while deleting past surgical history: {:#?}",
                    err
                );
                Err(ErrorData {
                    message: String::from("PASTSURGICALHISTORY_NOT_DELETED"),
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
