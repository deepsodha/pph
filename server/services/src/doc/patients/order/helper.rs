use crate::doc::syncs::DataTable;
use anyhow::Result;
use cosmo_store::{common::i64_event_version::EventVersion, types::event_read::EventRead};
use cosmo_store_util::aggregate::Aggregate;
use db_services::services::order::upsert_order;
use log::{error, info};
use patient::domain::immunizations::order::{
    order_aggregate::ORDER_AGGREGATE, order_domain::OrderState, order_events::OrderEvent,
};
use sqlx::{Pool, Sqlite};

pub async fn process_order_events(
    read_pool: Pool<Sqlite>,
    order_id: String,
    stream_id: String,
    read_events: Vec<EventRead<OrderEvent, OrderEvent, EventVersion>>,
) -> Result<()> {
    info!("Processing order events...");

    let order_db =
        sqlx::query_as::<_, DataTable>("SELECT * from  order_table_state WHERE id = ? LIMIT 1")
            .bind(order_id.clone())
            .fetch_optional(&read_pool)
            .await?;

    let order_state: Option<OrderState> =
        order_db.and_then(|data_table| serde_json::from_value(data_table.data).ok());

    let order_updated_state = read_events
        .iter()
        .fold(order_state, |a, b| ORDER_AGGREGATE.apply(a, &b.data));

    match order_updated_state {
        Some(p) => {
            info!("Updating order state...");
            upsert_order(
                read_pool,
                p,
                read_events
                    .last()
                    .map_or_else(|| 0, |event| event.version.0),
                stream_id,
            )
            .await?;
            info!("Order state updated successfully.");
        }
        None => {
            error!("Order with ID: {} not found", order_id);
        }
    }
    Ok(())
}
