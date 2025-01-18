use super::helper::process_historical_events;
use crate::{
    app_state::AppState,
    doc::{patients::get_patient_id::patient_id_fetch, syncs::sync},
    method::convention::ErrorData,
};
use anyhow::Result;
use common::stream::Stream;
use cosmo_store::types::{event_read_range::EventsReadRange, expected_version::ExpectedVersion};
use cosmo_store_util::aggregate::make_handler;
use patient::{
    domain::immunizations::addhistorical::{
        addhistorical_aggregate::HISTORICAL_AGGREGATE,
        addhistorical_commands::{
            CreateHistorical, DeleteHistorical, HistoricalCommand, UpdateHistorical,
        },
    },
    dto::immunization::add_historical_types::{HistoricalAdd, HistoricalDelete, HistoricalUpdate},
};
use serde_json::Value;
use sqlx::types::chrono::Utc;
use utils::store_helper::patient_store;
use uuid::Uuid;

use log::{error, info};

pub async fn add_historical(
    params: HistoricalAdd,
    app_state: AppState,
    organization_id: String,
    user_id: String,
) -> Result<Value, ErrorData> {
    info!("Start adding historical entry");

    let patient_id = patient_id_fetch(app_state.clone(), params.patient_id.clone()).await?;

    if patient_id {
        let store = patient_store(&app_state.write_pool, &organization_id).await?;
        let stream_id = format!("Historical::{}", Uuid::new_v4().as_simple());
        let command = CreateHistorical {
            id: Uuid::new_v4().as_simple().to_string(),
            org_id: organization_id.clone(),
            patient_id: params.patient_id,
            created_by: user_id.clone(),
            updated_by: user_id.clone(),
            created_at: Utc::now(),
            last_updated: Utc::now(),
            vaccine: params.vaccine,
            types: params.types,
            date: params.date,
            number_in_series: params.number_in_series,
            provider: params.provider,
            source_of_information: params.source_of_information,
            comments: params.comments,
        };
        let events: Vec<cosmo_store::types::event_read::EventRead<_, _, _>> = make_handler(
            &HISTORICAL_AGGREGATE,
            &store,
            &HistoricalCommand::CreateHistorical(command.clone()),
            &stream_id,
            &EventsReadRange::AllEvents,
            &ExpectedVersion::Any,
        )
        .await?;

        let res =
            process_historical_events(app_state.read_pool.clone(), command.id, stream_id, events)
                .await;

        let row = sync(
            params.last_updated_input,
            organization_id.clone(),
            app_state.read_pool.clone(),
        )
        .await?;

        match res {
            Ok(_) => {
                info!("Historical entry added successfully");
                serde_json::to_value(row).map_err(ErrorData::from)
            }
            Err(err) => {
                error!("{:#?}", err);
                Err(ErrorData {
                    message: String::from("ADDHISTORICAL_NOT_ADDED"),
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

pub async fn update_historical(
    params: HistoricalUpdate,
    app_state: AppState,
    organization_id: String,
    user_id: String,
) -> Result<Value, ErrorData> {
    let patient_id = patient_id_fetch(app_state.clone(), params.input.patient_id.clone()).await?;

    if patient_id {
        info!("Start updating historical entry");

        let store = patient_store(&app_state.write_pool, &organization_id).await?;
        let historical_db = sqlx::query_as::<_, Stream>(
            "SELECT id,stream_id,version,(data->>'created_by') AS created_by, (data->>'created_at') AS created_at,data FROM addhistorical_table_state WHERE id = ?",
        )
        .bind(params.id.clone())
        .fetch_optional(&app_state.read_pool.clone())
        .await?;

        let created_by = historical_db
            .as_ref()
            .map(|org| org.created_by.clone())
            .unwrap_or_default();
        let stream_id = historical_db
            .as_ref()
            .map(|org| org.stream_id.clone())
            .unwrap_or_default();
        let created_at = historical_db
            .as_ref()
            .map(|org| org.created_at)
            .unwrap_or_default();

        let command = UpdateHistorical {
            id: params.id,
            created_by,
            updated_by: user_id,
            created_at,
            last_updated: Utc::now(),
            org_id: organization_id.clone(),
            patient_id: params.input.patient_id,
            vaccine: params.input.vaccine,
            types: params.input.types,
            date: params.input.date,
            number_in_series: params.input.number_in_series,
            provider: params.input.provider,
            source_of_information: params.input.source_of_information,
            comments: params.input.comments,
        };
        let events: Vec<cosmo_store::types::event_read::EventRead<_, _, _>> = make_handler(
            &HISTORICAL_AGGREGATE,
            &store,
            &HistoricalCommand::UpdateHistorical(command.clone()),
            &stream_id,
            &EventsReadRange::AllEvents,
            &ExpectedVersion::Any,
        )
        .await?;

        let res =
            process_historical_events(app_state.read_pool.clone(), command.id, stream_id, events)
                .await;

        let row = sync(
            params.input.last_updated_input,
            organization_id.clone(),
            app_state.read_pool.clone(),
        )
        .await?;

        match res {
            Ok(_) => {
                info!("Historical entry updated successfully");
                serde_json::to_value(row).map_err(ErrorData::from)
            }
            Err(err) => {
                error!("{:#?}", err);
                Err(ErrorData {
                    message: String::from("UPDATEHISTORICAL_NOT_UPDATED"),
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

pub async fn delete_historical(
    params: HistoricalDelete,
    app_state: AppState,
    organization_id: String,
    user_id: String,
) -> Result<Value, ErrorData> {
    let patient_id = patient_id_fetch(app_state.clone(), params.patient_id.clone()).await?;

    if patient_id {
        info!("Start deleting historical entry");

        let store = patient_store(&app_state.write_pool, &organization_id).await?;
        let historical_db = sqlx::query_as::<_, Stream>(
            "SELECT id,stream_id,version,(data->>'created_by') AS created_by, (data->>'created_at') AS created_at,data FROM addhistorical_table_state WHERE id = ?",
        )
        .bind(params.id.clone())
        .fetch_optional(&app_state.read_pool.clone())
        .await?;
        let created_by = historical_db
            .as_ref()
            .map(|org| org.created_by.clone())
            .unwrap_or_default();
        let stream_id = historical_db
            .as_ref()
            .map(|org| org.stream_id.clone())
            .unwrap_or_default();
        let created_at = historical_db
            .as_ref()
            .map(|org| org.created_at)
            .unwrap_or_default();
        let command = DeleteHistorical {
            id: params.id,
            created_by,
            updated_by: user_id,
            created_at,
            last_updated: Utc::now(),
            org_id: organization_id.clone(),
            patient_id: params.patient_id,
        };
        let events: Vec<cosmo_store::types::event_read::EventRead<_, _, _>> = make_handler(
            &HISTORICAL_AGGREGATE,
            &store,
            &HistoricalCommand::DeleteHistorical(command.clone()),
            &stream_id,
            &EventsReadRange::AllEvents,
            &ExpectedVersion::Any,
        )
        .await?;

        let res =
            process_historical_events(app_state.read_pool.clone(), command.id, stream_id, events)
                .await;

        let row = sync(
            params.last_updated_input,
            organization_id.clone(),
            app_state.read_pool.clone(),
        )
        .await?;

        match res {
            Ok(_) => {
                info!("Historical entry deleted successfully");
                serde_json::to_value(row).map_err(ErrorData::from)
            }
            Err(err) => {
                error!("{:#?}", err);
                Err(ErrorData {
                    message: String::from("DELETEHISTORICAL_NOT_DELETED"),
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
