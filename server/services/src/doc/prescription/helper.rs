use anyhow::Result;
use cosmo_store::{common::i64_event_version::EventVersion, types::event_read::EventRead};
use cosmo_store_util::aggregate::Aggregate;
use db_services::services::prescription::upsert_prescription;
use log::error;
use prescription::domain::{
    prescription_aggregate::PRESCRIPTION_AGGREGATE, prescription_domain::PrescriptionState,
    prescription_events::PrescriptionEvent,
};
use sqlx::{Pool, Sqlite};

use crate::doc::syncs::DataTable;

pub async fn process_prescription_events(
    read_pool: Pool<Sqlite>,
    prescription_id: String,
    stream_id: String,
    read_events: Vec<EventRead<PrescriptionEvent, PrescriptionEvent, EventVersion>>,
) -> Result<()> {
    log::info!(
        "Processing prescription events for prescription_id: {} and stream_id: {}",
        prescription_id,
        stream_id
    );

    let prescription_db = sqlx::query_as::<_, DataTable>(
        "SELECT * from  prescription_table_state WHERE id = ? LIMIT 1",
    )
    .bind(prescription_id.clone())
    .fetch_optional(&read_pool)
    .await?;

    let prescription_state: Option<PrescriptionState> =
        prescription_db.and_then(|data_table| serde_json::from_value(data_table.data).ok());

    let prescription_updated_state = read_events.iter().fold(prescription_state, |a, b| {
        PRESCRIPTION_AGGREGATE.apply(a, &b.data)
    });

    match prescription_updated_state {
        Some(p) => {
            upsert_prescription(
                read_pool,
                p,
                read_events
                    .last()
                    .map_or_else(|| 0, |event| event.version.0),
                stream_id.clone(), // Added clone here for logging purposes
            )
            .await?;

            log::info!("Prescription events processed successfully for prescription_id: {} and stream_id: {}", prescription_id, stream_id);
        }
        None => {
            error!("Prescription with ID: {} not found", prescription_id);
        }
    }
    Ok(())
}
