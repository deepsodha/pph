use anyhow::Result;
use cosmo_store::{common::i64_event_version::EventVersion, types::event_read::EventRead};
use cosmo_store_util::aggregate::Aggregate;
use db_services::services::administer::upsert_administer;
use patient::domain::immunizations::administer::{
    administer_aggregate::ADMINISTER_AGGREGATE, administer_domain::AdministerState,
    administer_events::AdministerEvent,
};
use sqlx::{Pool, Sqlite};

use crate::doc::syncs::DataTable;

use log::{error, info};

pub async fn process_administer_events(
    read_pool: Pool<Sqlite>,
    administerd_id: String,
    stream_id: String,
    read_events: Vec<EventRead<AdministerEvent, AdministerEvent, EventVersion>>,
) -> Result<()> {
    info!(
        "Start processing administer events for ID: {}",
        administerd_id
    );

    let administer_db =
        sqlx::query_as::<_, DataTable>("SELECT * FROM administer_table_state WHERE id = ? LIMIT 1")
            .bind(administerd_id.clone())
            .fetch_optional(&read_pool)
            .await?;

    let administer_state: Option<AdministerState> =
        administer_db.and_then(|data_table| serde_json::from_value(data_table.data).ok());

    let administer_updated_state = read_events.iter().fold(
        administer_state,
        |a, b: &EventRead<AdministerEvent, AdministerEvent, EventVersion>| {
            ADMINISTER_AGGREGATE.apply(a, &b.data)
        },
    );

    match administer_updated_state {
        Some(p) => {
            upsert_administer(
                read_pool.clone(),
                p,
                read_events
                    .last()
                    .map_or_else(|| 0, |event| event.version.0),
                stream_id.clone(),
            )
            .await?;
            info!(
                "Administer events processed successfully for ID: {}",
                administerd_id
            );
        }
        None => {
            error!("Administer with ID: {} not found", administerd_id);
        }
    }

    info!(
        "Processing administer events completed for ID: {}",
        administerd_id
    );

    Ok(())
}
