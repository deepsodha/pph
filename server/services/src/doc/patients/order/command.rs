use super::helper::process_order_events;
use crate::app_state::AppState;
use crate::doc::patients::get_patient_id::patient_id_fetch;
use crate::doc::syncs::sync;
use crate::method::convention::ErrorData;
use anyhow::Result;
use common::stream::Stream;
use cosmo_store::types::{event_read_range::EventsReadRange, expected_version::ExpectedVersion};
use cosmo_store_util::aggregate::make_handler;
use log::{error, info};
use patient::domain::immunizations::order::order_commands::DeleteOrder;
use patient::domain::immunizations::order::{
    order_aggregate::ORDER_AGGREGATE,
    order_commands::{CreateOrder, OrderCommand, UpdateOrder},
};
use patient::dto::immunization::order_types::{OrderAdd, OrderDelete, OrderUpdate};
use serde_json::Value;
use sqlx::types::chrono::Utc;
use utils::store_helper::patient_store;
use uuid::Uuid;

pub async fn add_order(
    params: OrderAdd,
    app_state: AppState,
    organization_id: String,
    user_id: String,
) -> Result<Value, ErrorData> {
    let patient_id = patient_id_fetch(app_state.clone(), params.patient_id.clone()).await?;
    if patient_id {
        info!("Adding order...");

        let store = patient_store(&app_state.write_pool, &organization_id).await?;
        let stream_id = format!("Order::{}", Uuid::new_v4().as_simple());
        let command = CreateOrder {
            id: Uuid::new_v4().as_simple().to_string(),
            created_by: user_id.clone(),
            updated_by: user_id.clone(),
            created_at: Utc::now(),
            last_updated: Utc::now(),
            org_id: organization_id.clone(),
            patient_id: params.patient_id,
            vaccine: params.vaccine,
            types: params.types,
            ordered: params.ordered,
            provider: params.provider,
            comments: params.comments,
        };

        info!("Making handler to process order creation command...");
        let events = make_handler(
            &ORDER_AGGREGATE,
            &store,
            &OrderCommand::CreateOrder(command.clone()),
            &stream_id,
            &EventsReadRange::AllEvents,
            &ExpectedVersion::Any,
        )
        .await?;

        info!("Processing order events...");
        let res =
            process_order_events(app_state.read_pool.clone(), command.id, stream_id, events).await;

        info!("Synchronizing data...");
        let row = sync(
            params.last_updated_input,
            organization_id.clone(),
            app_state.read_pool.clone(),
        )
        .await?;

        match res {
            anyhow::Result::Ok(_) => {
                info!("Order added successfully.");
                serde_json::to_value(row).map_err(ErrorData::from)
            }
            Err(err) => {
                error!("Error occurred while adding order: {:#?}", err);
                Err(ErrorData {
                    message: String::from("ORDER_NOT_ADDED"),
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

pub async fn update_order(
    params: OrderUpdate,
    app_state: AppState,
    organization_id: String,
    user_id: String,
) -> Result<Value, ErrorData> {
    let patient_id = patient_id_fetch(app_state.clone(), params.input.patient_id.clone()).await?;
    if patient_id {
        let store = patient_store(&app_state.write_pool, &organization_id).await?;
        let order_db = sqlx::query_as::<_, Stream>(
            "SELECT id,stream_id,version,(data->>'created_by') AS created_by, (data->>'created_at') AS created_at,data FROM order_table_state WHERE id = ?",
        )
        .bind(params.id.clone())
        .fetch_optional(&app_state.read_pool.clone())
        .await?;

        let created_by = order_db
            .as_ref()
            .map(|org| org.created_by.clone())
            .unwrap_or_default();
        let stream_id = order_db
            .as_ref()
            .map(|org| org.stream_id.clone())
            .unwrap_or_default();
        let created_at = order_db
            .as_ref()
            .map(|org| org.created_at)
            .unwrap_or_default();
        let command = UpdateOrder {
            id: params.id,
            created_by,
            updated_by: user_id,
            created_at,
            last_updated: Utc::now(),
            org_id: organization_id.clone(),
            patient_id: params.input.patient_id,
            vaccine: params.input.vaccine,
            types: params.input.types,
            ordered: params.input.ordered,
            provider: params.input.provider,
            comments: params.input.comments,
        };
        let events = make_handler(
            &ORDER_AGGREGATE,
            &store,
            &OrderCommand::UpdateOrder(command.clone()),
            &stream_id,
            &EventsReadRange::AllEvents,
            &ExpectedVersion::Any,
        )
        .await?;
        let res =
            process_order_events(app_state.read_pool.clone(), command.id, stream_id, events).await;

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
                    message: String::from("ORDER_NOT_UPDATED"),
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

pub async fn delete_order(
    params: OrderDelete,
    app_state: AppState,
    organization_id: String,
    user_id: String,
) -> Result<Value, ErrorData> {
    let patient_id = patient_id_fetch(app_state.clone(), params.patient_id.clone()).await?;
    if patient_id {
        info!("Deleting order...");

        let store = patient_store(&app_state.write_pool, &organization_id).await?;

        // Querying order record from the database
        let order_db = sqlx::query_as::<_, Stream>(
            "SELECT id,stream_id,version,(data->>'created_by') AS created_by, (data->>'created_at') AS created_at,data FROM order_table_state WHERE id = ?",
        )
        .bind(params.id.clone())
        .fetch_optional(&app_state.read_pool.clone())
        .await?;

        let created_by = order_db
            .as_ref()
            .map(|org| org.created_by.clone())
            .unwrap_or_default();
        let stream_id = order_db
            .as_ref()
            .map(|org| org.stream_id.clone())
            .unwrap_or_default();
        let created_at = order_db
            .as_ref()
            .map(|org| org.created_at)
            .unwrap_or_default();

        let command = DeleteOrder {
            id: params.id,
            created_by,
            updated_by: user_id,
            created_at,
            last_updated: Utc::now(),
            org_id: organization_id.clone(),
            patient_id: params.patient_id.clone(),
        };

        info!("Making handler to process order deletion command...");
        let events = make_handler(
            &ORDER_AGGREGATE,
            &store,
            &OrderCommand::DeleteOrder(command.clone()),
            &stream_id,
            &EventsReadRange::AllEvents,
            &ExpectedVersion::Any,
        )
        .await?;

        info!("Processing order events...");
        let res =
            process_order_events(app_state.read_pool.clone(), command.id, stream_id, events).await;

        info!("Synchronizing data...");
        let row = sync(
            params.last_updated_input,
            organization_id.clone(),
            app_state.read_pool.clone(),
        )
        .await?;

        match res {
            anyhow::Result::Ok(_) => {
                info!("Order deleted successfully.");
                serde_json::to_value(row).map_err(ErrorData::from)
            }
            Err(err) => {
                error!("Error occurred while deleting order: {:#?}", err);
                Err(ErrorData {
                    message: String::from("ORDER_NOT_DELETED"),
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
