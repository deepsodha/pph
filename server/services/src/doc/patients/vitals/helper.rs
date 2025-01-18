use crate::doc::syncs::DataTable;
use anyhow::Result;
use cosmo_store::{common::i64_event_version::EventVersion, types::event_read::EventRead};
use cosmo_store_util::aggregate::Aggregate;
use db_services::services::vitals::upsert_vitals;
use log::{error, info};
use patient::domain::vital::{
    vital_aggregate::VITALS_AGGREGATE, vital_domain::VitalsState, vital_events::VitalsEvent,
};
use sqlx::{Pool, Sqlite};
pub async fn process_vitals_events(
    read_pool: Pool<Sqlite>,
    vitals_id: String,
    stream_id: String,
    read_events: Vec<EventRead<VitalsEvent, VitalsEvent, EventVersion>>,
) -> Result<()> {
    info!("Processing vitals events");

    let vitals_db =
        sqlx::query_as::<_, DataTable>("SELECT * from  vitals_table_state WHERE id = ? LIMIT 1")
            .bind(vitals_id.clone())
            .fetch_optional(&read_pool)
            .await?;

    let vitals_state: Option<VitalsState> =
        vitals_db.and_then(|data_table| serde_json::from_value(data_table.data).ok());
    let vitals_updated_state = read_events
        .iter()
        .fold(vitals_state, |a, b| VITALS_AGGREGATE.apply(a, &b.data));

    match vitals_updated_state {
        Some(p) => {
            info!("Vitals state updated successfully");
            upsert_vitals(
                read_pool,
                p,
                read_events
                    .last()
                    .map_or_else(|| 0, |event| event.version.0),
                stream_id,
            )
            .await?;
        }
        None => {
            error!("Vitals with ID: {} not found", vitals_id);
        }
    }
    Ok(())
}
