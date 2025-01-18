use crate::doc::syncs::DataTable;
use anyhow::Result;
use cosmo_store::{common::i64_event_version::EventVersion, types::event_read::EventRead};
use cosmo_store_util::aggregate::Aggregate;
use db_services::services::hospitalization::upsert_hospitalization;
use log::{debug, error, info};
use patient::domain::history::hospitalization::{
    hospitalization_aggregate::HOSPITALIZATION_AGGREGATE,
    hospitalization_domain::HospitalizationState, hospitalization_events::HospitalizationEvent,
};
use sqlx::{Pool, Sqlite};

pub async fn process_hospitalization_events(
    read_pool: Pool<Sqlite>,
    hospitalization_id: String,
    stream_id: String,
    read_events: Vec<EventRead<HospitalizationEvent, HospitalizationEvent, EventVersion>>,
) -> Result<()> {
    info!("Processing hospitalization events...");

    let hospitalization_db = sqlx::query_as::<_, DataTable>(
        "SELECT * from  hospitalization_table_state WHERE id = ? LIMIT 1",
    )
    .bind(hospitalization_id.clone())
    .fetch_optional(&read_pool)
    .await?;

    let hospitalization_state: Option<HospitalizationState> =
        hospitalization_db.and_then(|data_table| serde_json::from_value(data_table.data).ok());

    debug!(
        "Hospitalization state retrieved: {:?}",
        hospitalization_state
    );

    let hospitalization_updated_state = read_events.iter().fold(hospitalization_state, |a, b| {
        HOSPITALIZATION_AGGREGATE.apply(a, &b.data)
    });

    debug!(
        "Updated hospitalization state: {:?}",
        hospitalization_updated_state
    );

    match hospitalization_updated_state {
        Some(p) => {
            debug!("Upserting hospitalization state...");
            upsert_hospitalization(
                read_pool,
                p,
                read_events
                    .last()
                    .map_or_else(|| 0, |event| event.version.0),
                stream_id,
            )
            .await?;
            info!("Hospitalization events processed successfully.");
        }
        None => {
            error!("Hospitalization with ID: {} not found", hospitalization_id);
        }
    }
    Ok(())
}
