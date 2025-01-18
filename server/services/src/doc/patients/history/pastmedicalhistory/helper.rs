use crate::doc::syncs::DataTable;
use anyhow::Result;
use cosmo_store::{common::i64_event_version::EventVersion, types::event_read::EventRead};
use cosmo_store_util::aggregate::Aggregate;
use db_services::services::pastmedicalhistory::upsert_pastmedicalhistory;
use log::{error, info};
use patient::domain::history::pastmedical_history::{
    pastmedical_history_aggregate::PASTMEDICALHISTORY_AGGREGATE,
    pastmedical_history_domain::PastMedicalHistoryState,
    pastmedical_history_events::PastMedicalHistoryEvent,
};
use sqlx::{Pool, Sqlite};

pub async fn process_pastmedicalhistory_events(
    read_pool: Pool<Sqlite>,
    pastmedicalhistory_id: String,
    stream_id: String,
    read_events: Vec<EventRead<PastMedicalHistoryEvent, PastMedicalHistoryEvent, EventVersion>>,
) -> Result<()> {
    info!("Processing Past Medical History Events...");

    let pastmedicalhistory_db = sqlx::query_as::<_, DataTable>(
        "SELECT * FROM pastmedicalhistory_table_state WHERE id = ? LIMIT 1",
    )
    .bind(pastmedicalhistory_id.clone())
    .fetch_optional(&read_pool)
    .await?;

    let pastmedicalhistory_state: Option<PastMedicalHistoryState> =
        pastmedicalhistory_db.and_then(|data_table| serde_json::from_value(data_table.data).ok());

    let pastmedicalhistory_updated_state =
        read_events.iter().fold(pastmedicalhistory_state, |a, b| {
            PASTMEDICALHISTORY_AGGREGATE.apply(a, &b.data)
        });

    match pastmedicalhistory_updated_state {
        Some(p) => {
            upsert_pastmedicalhistory(
                read_pool.clone(),
                p,
                read_events
                    .last()
                    .map_or_else(|| 0, |event| event.version.0),
                stream_id.clone(),
            )
            .await?;
        }
        None => {
            error!("Past Medical with ID: {} not found", pastmedicalhistory_id);
        }
    }

    info!("Processed Past Medical History Events.");
    Ok(())
}
