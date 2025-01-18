use crate::doc::syncs::DataTable;
use anyhow::Result;
use cosmo_store::{common::i64_event_version::EventVersion, types::event_read::EventRead};
use cosmo_store_util::aggregate::Aggregate;
use db_services::services::pastsurgicalhistory::upsert_pastsurgicalhistory;
use log::{error, info};
use patient::domain::history::pastsurgical_history::{
    pastsurgical_history_aggregate::PASTSURGICALHISTORY_AGGREGATE,
    pastsurgical_history_domain::PastSurgicalHistoryState,
    pastsurgical_history_events::PastSurgicalEvent,
};
use sqlx::{Pool, Sqlite};

pub async fn process_pastsurgicalhistory_events(
    read_pool: Pool<Sqlite>,
    pastsurgicalhistory_id: String,
    stream_id: String,
    read_events: Vec<EventRead<PastSurgicalEvent, PastSurgicalEvent, EventVersion>>,
) -> Result<()> {
    info!("Processing past surgical history events...");

    let pastsurgicalhistory_db = sqlx::query_as::<_, DataTable>(
        "SELECT * from  pastsurgicalhistory_table_state WHERE id = ? LIMIT 1",
    )
    .bind(pastsurgicalhistory_id.clone())
    .fetch_optional(&read_pool)
    .await?;

    let pastsurgicalhistory_state: Option<PastSurgicalHistoryState> =
        pastsurgicalhistory_db.and_then(|data_table| serde_json::from_value(data_table.data).ok());

    let pastsurgicalhistory_updated_state =
        read_events.iter().fold(pastsurgicalhistory_state, |a, b| {
            PASTSURGICALHISTORY_AGGREGATE.apply(a, &b.data)
        });

    match pastsurgicalhistory_updated_state {
        Some(p) => {
            upsert_pastsurgicalhistory(
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
            error!(
                "Past surgical with ID: {} not found",
                pastsurgicalhistory_id
            );
        }
    }

    info!("Past surgical history events processed successfully.");
    Ok(())
}
