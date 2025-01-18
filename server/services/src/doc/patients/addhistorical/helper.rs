use anyhow::Result;
use cosmo_store::{common::i64_event_version::EventVersion, types::event_read::EventRead};
use cosmo_store_util::aggregate::Aggregate;
use db_services::services::addhistorical::upsert_historical;
use patient::domain::immunizations::addhistorical::{
    addhistorical_aggregate::HISTORICAL_AGGREGATE, addhistorical_domain::HistoricalState,
    addhistorical_events::HistoricalEvent,
};
use sqlx::{Pool, Sqlite};

use crate::doc::syncs::DataTable;

use log::{error, info};

pub async fn process_historical_events(
    read_pool: Pool<Sqlite>,
    historical_id: String,
    stream_id: String,
    read_events: Vec<EventRead<HistoricalEvent, HistoricalEvent, EventVersion>>,
) -> Result<()> {
    info!(
        "Start processing historical events for ID: {}",
        historical_id
    );

    let historical_db = sqlx::query_as::<_, DataTable>(
        "SELECT * FROM addhistorical_table_state WHERE id = ? LIMIT 1",
    )
    .bind(historical_id.clone())
    .fetch_optional(&read_pool)
    .await?;

    let historical_state: Option<HistoricalState> =
        historical_db.and_then(|data_table| serde_json::from_value(data_table.data).ok());

    let historical_updated_state = read_events.iter().fold(historical_state, |a, b| {
        HISTORICAL_AGGREGATE.apply(a, &b.data)
    });

    match historical_updated_state {
        Some(p) => {
            upsert_historical(
                read_pool.clone(),
                p,
                read_events
                    .last()
                    .map_or_else(|| 0, |event| event.version.0),
                stream_id.clone(),
            )
            .await?;
            info!(
                "Historical events processed successfully for ID: {}",
                historical_id
            );
        }
        None => {
            error!("Historical with ID: {} not found", historical_id);
        }
    }

    info!(
        "Processing historical events completed for ID: {}",
        historical_id
    );

    Ok(())
}
