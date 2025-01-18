use crate::doc::syncs::DataTable;
use anyhow::Result;
use cosmo_store::{common::i64_event_version::EventVersion, types::event_read::EventRead};
use cosmo_store_util::aggregate::Aggregate;
use db_services::services::socialhistory::upsert_socialhistory;
use log::{error, info};
use patient::domain::history::social_history::{
    social_history_aggregate::SOCIALHISTORY_AGGREGATE, social_history_domain::SocialHistoryState,
    social_history_events::SocialHistoryEvent,
};
use sqlx::{Pool, Sqlite};

pub async fn process_socialhistory_events(
    read_pool: Pool<Sqlite>,
    socialhistory_id: String,
    stream_id: String,
    read_events: Vec<EventRead<SocialHistoryEvent, SocialHistoryEvent, EventVersion>>,
) -> Result<()> {
    info!("Processing social history events...");

    let socialhistory_db = sqlx::query_as::<_, DataTable>(
        "SELECT * from  socialhistory_table_state WHERE id = ? LIMIT 1",
    )
    .bind(socialhistory_id.clone())
    .fetch_optional(&read_pool)
    .await?;

    let socialhistory_state: Option<SocialHistoryState> =
        socialhistory_db.and_then(|data_table| serde_json::from_value(data_table.data).ok());

    let socialhistory_updated_state = read_events.iter().fold(socialhistory_state, |a, b| {
        SOCIALHISTORY_AGGREGATE.apply(a, &b.data)
    });

    match socialhistory_updated_state {
        Some(p) => {
            info!("Upserting social history...");
            upsert_socialhistory(
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
            error!("Social history with ID: {} not found", socialhistory_id);
        }
    }

    info!("Social history events processed successfully.");
    Ok(())
}
