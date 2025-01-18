use super::helper::process_familyhistory_events;
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
    domain::history::familyhistory::{
        familyhistory_aggregate::FAMILYHISTORY_AGGREGATE,
        familyhistory_commands::{
            CreateFamilyHistory, DeleteFamilyHistory, FamilyHistoryCommand, UpdateFamilyHistory,
        },
    },
    dto::history::familyhistory::{
        familyhistory_add::FamilyHistoryAdd, familyhistory_delete::FamilyHistoryDelete,
        familyhistory_update::FamilyHistoryUpdate,
    },
};
use serde_json::Value;
use utils::store_helper::familyhistory_store;
use uuid::Uuid;
pub async fn add_familyhistory(
    params: FamilyHistoryAdd,
    app_state: AppState,
    organization_id: String,
    user_id: String,
) -> Result<Value, ErrorData> {
    let patient_id = patient_id_fetch(app_state.clone(), params.patient_id.clone()).await?;
    if patient_id {
        let store = familyhistory_store(&app_state.write_pool, &organization_id).await?;
        let stream_id = format!("FamilyHistory::{}", uuid::Uuid::new_v4().as_simple());
        let command = CreateFamilyHistory {
            id: Uuid::new_v4().as_simple().to_string(),
            family_member: params.family_member,
            health_status: params.health_status,
            general: params.general,
            cancer: params.cancer,
            comments: params.comments,
            org_id: organization_id.clone(),
            patient_id: params.patient_id,
            created_by: user_id.clone(),
            updated_by: user_id.clone(),
            created_at: Utc::now(),
            last_updated: Utc::now(),
        };
        let events = make_handler(
            &FAMILYHISTORY_AGGREGATE,
            &store,
            &FamilyHistoryCommand::CreateFamilyHistory(command.clone()),
            &stream_id,
            &EventsReadRange::AllEvents,
            &ExpectedVersion::Any,
        )
        .await?;

        let res = process_familyhistory_events(
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
                info!("Family history added successfully");
                serde_json::to_value(row).map_err(ErrorData::from)
            }
            Err(err) => {
                error!("{:#?}", err);
                Err(ErrorData {
                    message: String::from("FAMILYHISTORY_NOT_ADDED"),
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

pub async fn update_familyhistory(
    params: FamilyHistoryUpdate,
    app_state: AppState,
    organization_id: String,
    user_id: String,
) -> Result<Value, ErrorData> {
    let patient_id = patient_id_fetch(app_state.clone(), params.input.patient_id.clone()).await?;
    if patient_id {
        let store = familyhistory_store(&app_state.write_pool, &organization_id).await?;
        let familyhistory_db = sqlx::query_as::<_, Stream>(
        "SELECT id,stream_id,version,(data->>'created_by') AS created_by, (data->>'created_at') AS created_at,data FROM familyhistory_table_state WHERE id = ?",
    )
    .bind(params.id.clone())
    .fetch_optional(&app_state.read_pool.clone())
    .await?;

        let created_by = familyhistory_db
            .as_ref()
            .map(|org| org.created_by.clone())
            .unwrap_or_default();
        let stream_id = familyhistory_db
            .as_ref()
            .map(|org| org.stream_id.clone())
            .unwrap_or_default();
        let created_at = familyhistory_db
            .as_ref()
            .map(|org| org.created_at)
            .unwrap_or_default();
        let command = UpdateFamilyHistory {
            id: params.id,
            created_by,
            updated_by: user_id,
            created_at,
            last_updated: Utc::now(),
            org_id: organization_id.clone(),
            patient_id: params.input.patient_id,
            family_member: params.input.family_member,
            health_status: params.input.health_status,
            general: params.input.general,
            cancer: params.input.cancer,
            comments: params.input.comments,
        };
        let events = make_handler(
            &FAMILYHISTORY_AGGREGATE,
            &store,
            &FamilyHistoryCommand::UpdateFamilyHistory(command.clone()),
            &stream_id,
            &EventsReadRange::AllEvents,
            &ExpectedVersion::Any,
        )
        .await?;
        let res = process_familyhistory_events(
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
                info!("Family history updated successfully");
                serde_json::to_value(row).map_err(ErrorData::from)
            }
            Err(err) => {
                error!("{:#?}", err);
                Err(ErrorData {
                    message: String::from("FAMILYHISTORY_NOT_UPDATED"),
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

pub async fn delete_familyhistory(
    params: FamilyHistoryDelete,
    app_state: AppState,
    organization_id: String,
    user_id: String,
) -> Result<Value, ErrorData> {
    let patient_id = patient_id_fetch(app_state.clone(), params.patient_id.clone()).await?;
    if patient_id {
        let store = familyhistory_store(&app_state.write_pool, &organization_id).await?;
        let familyhistory_db = sqlx::query_as::<_, Stream>(
            "SELECT id,stream_id,version,(data->>'created_by') AS created_by, (data->>'created_at') AS created_at,data FROM familyhistory_table_state WHERE id = ?",
        )
        .bind(params.id.clone())
        .fetch_optional(&app_state.read_pool.clone())
        .await?;

        let created_by = familyhistory_db
            .as_ref()
            .map(|org| org.created_by.clone())
            .unwrap_or_default();
        let stream_id = familyhistory_db
            .as_ref()
            .map(|org| org.stream_id.clone())
            .unwrap_or_default();
        let created_at = familyhistory_db
            .as_ref()
            .map(|org| org.created_at)
            .unwrap_or_default();

        let command = DeleteFamilyHistory {
            id: params.id,
            patient_id: params.patient_id,
            org_id: organization_id.clone(),
            created_by,
            updated_by: user_id,
            created_at,
            last_updated: Utc::now(),
        };
        let events = make_handler(
            &FAMILYHISTORY_AGGREGATE,
            &store,
            &FamilyHistoryCommand::DeleteFamilyHistory(command.clone()),
            &stream_id,
            &EventsReadRange::AllEvents,
            &ExpectedVersion::Any,
        )
        .await?;

        let res = process_familyhistory_events(
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
                info!("Family history deleted successfully");
                serde_json::to_value(row).map_err(ErrorData::from)
            }
            Err(err) => {
                error!("{:#?}", err);
                Err(ErrorData {
                    message: String::from("FAMILYHISTORY_NOT_DELETED"),
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
