use crate::doc::syncs::DataTable;
use anyhow::Result;
use cosmo_store::{common::i64_event_version::EventVersion, types::event_read::EventRead};
use cosmo_store_util::aggregate::Aggregate;
use db_services::services::familyhistory::upsert_familyhistory;
use log::{error, info};
use patient::domain::history::familyhistory::{
    familyhistory_aggregate::FAMILYHISTORY_AGGREGATE, familyhistory_domain::FamilyHistoryState,
    familyhistory_events::FamilyHistoryEvent,
};
use sqlx::{Pool, Sqlite};
pub async fn process_familyhistory_events(
    read_pool: Pool<Sqlite>,
    familyhistory_id: String,
    stream_id: String,
    read_events: Vec<EventRead<FamilyHistoryEvent, FamilyHistoryEvent, EventVersion>>,
) -> Result<()> {
    let familyhistory_db = sqlx::query_as::<_, DataTable>(
        "SELECT * FROM familyhistory_table_state WHERE id = ? LIMIT 1",
    )
    .bind(&familyhistory_id)
    .fetch_optional(&read_pool)
    .await?;

    let familyhistory_state: Option<FamilyHistoryState> =
        familyhistory_db.and_then(|data_table| serde_json::from_value(data_table.data).ok());

    let familyhistory_updated_state = read_events.iter().fold(familyhistory_state, |a, b| {
        FAMILYHISTORY_AGGREGATE.apply(a, &b.data)
    });

    match familyhistory_updated_state {
        Some(p) => {
            upsert_familyhistory(
                read_pool.clone(),
                p,
                read_events.last().map_or(0, |event| event.version.0),
                stream_id.clone(),
            )
            .await?;
            info!("Family history event processed successfully");
        }
        None => {
            error!("Family history with ID: {} not found", familyhistory_id);
        }
    }
    Ok(())
}
