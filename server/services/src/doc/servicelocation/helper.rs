use anyhow::Result;
use cosmo_store::{common::i64_event_version::EventVersion, types::event_read::EventRead};
use cosmo_store_util::aggregate::Aggregate;
use db_services::services::servicelocation::upsert_servicelocation;
use log::error;
use servicelocation::domain::{
    service_location_aggregate::SERVICELOCATION_AGGREGATE,
    service_location_domain::ServiceLocationState, service_location_events::ServiceLocationEvent,
};
use sqlx::{Pool, Sqlite};

use crate::doc::syncs::DataTable;

pub async fn process_servicelocation_events(
    read_pool: Pool<Sqlite>,
    servicelocation_id: String,
    stream_id: String,
    read_events: Vec<EventRead<ServiceLocationEvent, ServiceLocationEvent, EventVersion>>,
) -> Result<()> {
    log::info!(
        "Processing service location events for servicelocation_id: {} and stream_id: {}",
        servicelocation_id,
        stream_id
    );

    let servicelocation_db = sqlx::query_as::<_, DataTable>(
        "SELECT * from  servicelocation_table_state WHERE id = ? LIMIT 1",
    )
    .bind(servicelocation_id.clone())
    .fetch_optional(&read_pool)
    .await?;

    let servicelocation_state: Option<ServiceLocationState> =
        servicelocation_db.and_then(|data_table| serde_json::from_value(data_table.data).ok());

    let servicelocation_updated_state = read_events.iter().fold(servicelocation_state, |a, b| {
        SERVICELOCATION_AGGREGATE.apply(a, &b.data)
    });

    match servicelocation_updated_state {
        Some(p) => {
            upsert_servicelocation(
                read_pool.clone(),
                p,
                read_events
                    .last()
                    .map_or_else(|| 0, |event| event.version.0),
                stream_id.clone(),
            )
            .await?;
            log::info!("Service location events processed successfully for servicelocation_id: {} and stream_id: {}", servicelocation_id, stream_id);
        }
        None => {
            error!("ServiceLocation with ID: {} not found", servicelocation_id);
        }
    }
    Ok(())
}
