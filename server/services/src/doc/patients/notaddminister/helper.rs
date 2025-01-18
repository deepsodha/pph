use crate::doc::syncs::DataTable;
use anyhow::Result;
use cosmo_store::{common::i64_event_version::EventVersion, types::event_read::EventRead};
use cosmo_store_util::aggregate::Aggregate;
use db_services::services::notadminister::upsert_notadminister;
use log::{error, info};
use patient::domain::immunizations::notadministered::{
    notadministered_aggregate::NOT_ADMINISTER_AGGREGATE,
    notadministered_domain::NotAdministeredState, notadministered_events::NotAdministerEvent,
};
use sqlx::{Pool, Sqlite};

pub async fn process_not_administer_events(
    read_pool: Pool<Sqlite>,
    not_administered_id: String,
    stream_id: String,
    read_events: Vec<EventRead<NotAdministerEvent, NotAdministerEvent, EventVersion>>,
) -> Result<()> {
    info!("Processing not administered events...");

    // Querying not administered record from the database
    let not_administer_db = sqlx::query_as::<_, DataTable>(
        "SELECT * from  notadminister_table_state WHERE id = ? LIMIT 1",
    )
    .bind(not_administered_id.clone())
    .fetch_optional(&read_pool)
    .await?;

    let not_administer_state: Option<NotAdministeredState> =
        not_administer_db.and_then(|data_table| serde_json::from_value(data_table.data).ok());

    let not_administer_updated_state = read_events.iter().fold(not_administer_state, |a, b| {
        NOT_ADMINISTER_AGGREGATE.apply(a, &b.data)
    });

    match not_administer_updated_state {
        Some(p) => {
            info!("Upserting not administered...");
            upsert_notadminister(
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
            error!("Not Administer with ID: {} not found", not_administered_id);
        }
    }

    info!("Not administered events processed successfully.");
    Ok(())
}
