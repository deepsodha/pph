use crate::doc::syncs::DataTable;
use anyhow::Result;
use cosmo_store::{common::i64_event_version::EventVersion, types::event_read::EventRead};
use cosmo_store_util::aggregate::Aggregate;
use db_services::services::medication::upsert_medication;
use log::{error, info};
use patient::domain::medication::{
    medication_aggregate::MEDICATIONS_AGGREGATE, medication_domain::MedicationsState,
    medication_events::MedicationsEvent,
};
use sqlx::{Pool, Sqlite};
pub async fn process_medication_events(
    read_pool: Pool<Sqlite>,
    medication_id: String,
    stream_id: String,
    read_events: Vec<EventRead<MedicationsEvent, MedicationsEvent, EventVersion>>,
) -> Result<()> {
    info!("Processing medication events...");

    let medication_db = sqlx::query_as::<_, DataTable>(
        "SELECT * from  medication_table_state WHERE id = ? LIMIT 1",
    )
    .bind(medication_id.clone())
    .fetch_optional(&read_pool)
    .await?;

    let medication_state: Option<MedicationsState> =
        medication_db.and_then(|data_table| serde_json::from_value(data_table.data).ok());

    let medication_updated_state = read_events.iter().fold(medication_state, |a, b| {
        MEDICATIONS_AGGREGATE.apply(a, &b.data)
    });

    match medication_updated_state {
        Some(p) => {
            info!("Medication state updated successfully.");
            upsert_medication(
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
            error!("Medication with ID: {} not found", medication_id);
        }
    }

    info!("Medication events processed successfully.");
    Ok(())
}
