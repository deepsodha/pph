use super::helper::process_socialhistory_events;
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
    domain::history::social_history::{
        social_history_aggregate::SOCIALHISTORY_AGGREGATE,
        social_history_commands::{
            CreateSocialHistory, DeleteSocialHistory, SocialHistoryCommand, UpdateSocialHistory,
        },
    },
    dto::history::social_history::{
        socialhistory_add::SocialHistoryAdd, socialhistory_delete::SocialHistoryDelete,
        socialhistory_update::SocialHistoryUpdate,
    },
};
use serde_json::Value;
use utils::store_helper::socialhistory_store;
use uuid::Uuid;

pub async fn add_social_history(
    params: SocialHistoryAdd,
    app_state: AppState,
    organization_id: String,
    user_id: String,
) -> Result<Value, ErrorData> {
    let patient_id = patient_id_fetch(app_state.clone(), params.patient_id.clone()).await?;
    if patient_id {
        info!("Adding social history...");

        let store = socialhistory_store(&app_state.write_pool, &organization_id).await?;
        let stream_id = format!("SocialHistory::{}", uuid::Uuid::new_v4().as_simple());
        let command = CreateSocialHistory {
            id: Uuid::new_v4().as_simple().to_string(),
            org_id: organization_id.clone(),
            patient_id: params.patient_id,
            created_by: user_id.clone(),
            updated_by: user_id.clone(),
            created_at: Utc::now(),
            last_updated: Utc::now(),
            comments: params.comments,
            birth_gender: params.birth_gender,
            tobacco: params.tobacco,
            alcohol: params.alcohol,
            cardiovascular: params.cardiovascular,
            sexual_activity: params.sexual_activity,
            drug_abuse: params.drug_abuse,
            safety: params.safety,
        };

        info!("Making handler to process social history creation command...");
        let events = make_handler(
            &SOCIALHISTORY_AGGREGATE,
            &store,
            &SocialHistoryCommand::CreateSocialHistory(command.clone()),
            &stream_id,
            &EventsReadRange::AllEvents,
            &ExpectedVersion::Any,
        )
        .await?;

        info!("Processing social history events...");
        let res = process_socialhistory_events(
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
                info!("Social history added successfully.");
                serde_json::to_value(row).map_err(ErrorData::from)
            }
            Err(err) => {
                error!("Error occurred while adding social history: {:#?}", err);
                Err(ErrorData {
                    message: String::from("SOCIALHISTORY_NOT_ADDED"),
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

pub async fn update_social_history(
    params: SocialHistoryUpdate,
    app_state: AppState,
    organization_id: String,
    user_id: String,
) -> Result<Value, ErrorData> {
    let patient_id = patient_id_fetch(app_state.clone(), params.input.patient_id.clone()).await?;
    if patient_id {
        let store = socialhistory_store(&app_state.write_pool, &organization_id).await?;
        let socialhistory_db = sqlx::query_as::<_, Stream>(
        "SELECT id,stream_id,version,(data->>'created_by') AS created_by, (data->>'created_at') AS created_at,data FROM socialhistory_table_state WHERE id = ?",
    )
    .bind(params.id.clone())
    .fetch_optional(&app_state.read_pool.clone())
    .await?;

        let created_by = socialhistory_db
            .as_ref()
            .map(|org| org.created_by.clone())
            .unwrap_or_default();
        let stream_id = socialhistory_db
            .as_ref()
            .map(|org| org.stream_id.clone())
            .unwrap_or_default();
        let created_at = socialhistory_db
            .as_ref()
            .map(|org| org.created_at)
            .unwrap_or_default();
        let command = UpdateSocialHistory {
            id: params.id,
            created_by,
            updated_by: user_id,
            created_at,
            last_updated: Utc::now(),
            org_id: organization_id.clone(),
            patient_id: params.input.patient_id,
            comments: params.input.comments,
            birth_gender: params.input.birth_gender,
            tobacco: params.input.tobacco,
            alcohol: params.input.alcohol,
            cardiovascular: params.input.cardiovascular,
            sexual_activity: params.input.sexual_activity,
            drug_abuse: params.input.drug_abuse,
            safety: params.input.safety,
        };
        let events = make_handler(
            &SOCIALHISTORY_AGGREGATE,
            &store,
            &SocialHistoryCommand::UpdateSocialHistory(command.clone()),
            &stream_id,
            &EventsReadRange::AllEvents,
            &ExpectedVersion::Any,
        )
        .await?;
        let res = process_socialhistory_events(
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
            anyhow::Result::Ok(_) => serde_json::to_value(row).map_err(ErrorData::from),
            Err(err) => {
                error!("{:#?}", err);
                Err(ErrorData {
                    message: String::from("SOCIALHISTORY_NOT_UPDATED"),
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

pub async fn delete_social_history(
    params: SocialHistoryDelete,
    app_state: AppState,
    organization_id: String,
    user_id: String,
) -> Result<Value, ErrorData> {
    let patient_id = patient_id_fetch(app_state.clone(), params.patient_id.clone()).await?;
    if patient_id {
        let store = socialhistory_store(&app_state.write_pool, &organization_id).await?;
        let socialhistory_db = sqlx::query_as::<_, Stream>(
        "SELECT id,stream_id,version,(data->>'created_by') AS created_by, (data->>'created_at') AS created_at,data FROM socialhistory_table_state WHERE id = ?",
    )
    .bind(params.id.clone())
    .fetch_optional(&app_state.read_pool.clone())
    .await?;

        let created_by = socialhistory_db
            .as_ref()
            .map(|org| org.created_by.clone())
            .unwrap_or_default();
        let stream_id = socialhistory_db
            .as_ref()
            .map(|org| org.stream_id.clone())
            .unwrap_or_default();
        let created_at = socialhistory_db
            .as_ref()
            .map(|org| org.created_at)
            .unwrap_or_default();

        let command = DeleteSocialHistory {
            id: params.id,
            patient_id: params.patient_id,
            org_id: organization_id.clone(),
            created_by,
            updated_by: user_id,
            created_at,
            last_updated: Utc::now(),
        };
        let events = make_handler(
            &SOCIALHISTORY_AGGREGATE,
            &store,
            &SocialHistoryCommand::DeleteSocialHistory(command.clone()),
            &stream_id,
            &EventsReadRange::AllEvents,
            &ExpectedVersion::Any,
        )
        .await?;

        let res = process_socialhistory_events(
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
            anyhow::Result::Ok(_) => serde_json::to_value(row).map_err(ErrorData::from),
            Err(err) => {
                error!("{:#?}", err);
                Err(ErrorData {
                    message: String::from("SOCIALHISTORY_NOT_DELETED"),
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
