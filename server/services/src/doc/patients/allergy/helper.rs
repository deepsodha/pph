use anyhow::Result;
use cosmo_store::{common::i64_event_version::EventVersion, types::event_read::EventRead};
use cosmo_store_util::aggregate::Aggregate;
use db_services::services::allergies::upsert_allergies;
use patient::domain::allergies::{
    allergies_aggregate::ALLERGIES_AGGREGATE, allergies_domain::AllergiesState,
    allergies_events::AllergiesEvent,
};
use sqlx::{Pool, Sqlite};

use crate::doc::syncs::DataTable;
use log::{error, info};

pub async fn process_allergies_events(
    read_pool: Pool<Sqlite>,
    allergy_id: String,
    stream_id: String,
    read_events: Vec<EventRead<AllergiesEvent, AllergiesEvent, EventVersion>>,
) -> Result<()> {
    info!("Start processing allergies events");

    let allergies_db =
        sqlx::query_as::<_, DataTable>("SELECT * from  allergies_table_state WHERE id = ? LIMIT 1")
            .bind(allergy_id.clone())
            .fetch_optional(&read_pool)
            .await?;

    let allergies_state: Option<AllergiesState> =
        allergies_db.and_then(|data_table| serde_json::from_value(data_table.data).ok());

    let allergies_updated_state = read_events.iter().fold(allergies_state, |a, b| {
        ALLERGIES_AGGREGATE.apply(a, &b.data)
    });

    match allergies_updated_state {
        Some(p) => {
            upsert_allergies(
                read_pool.clone(),
                p,
                read_events
                    .last()
                    .map_or_else(|| 0, |event| event.version.0),
                stream_id.clone(),
            )
            .await?;
            info!("Allergies events processed successfully");
        }
        None => {
            error!("Allergies with ID: {} not found", allergy_id);
        }
    }
    Ok(())
}
