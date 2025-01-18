use crate::doc::syncs::DataTable;
use anyhow::Result;
use cosmo_store::{common::i64_event_version::EventVersion, types::event_read::EventRead};
use cosmo_store_util::aggregate::Aggregate;
use db_services::services::staff::upsert_staff;
use log::{error, info};
use sqlx::{Pool, Sqlite};
use staff::domain::{
    staff_aggregate::STAFF_AGGREGATE, staff_domain::StaffState, staff_events::StaffEvent,
};

pub async fn process_staff_events(
    read_pool: Pool<Sqlite>,
    staff_id: String,
    stream_id: String,
    read_events: Vec<EventRead<StaffEvent, StaffEvent, EventVersion>>,
) -> Result<()> {
    info!("Start processing staff events");

    let staff_db =
        sqlx::query_as::<_, DataTable>("SELECT * from staff_table_state WHERE id = ? LIMIT 1")
            .bind(staff_id.clone())
            .fetch_optional(&read_pool)
            .await?;

    let staff_state: Option<StaffState> =
        staff_db.and_then(|data_table| serde_json::from_value(data_table.data).ok());

    let staff_updated_state = read_events
        .iter()
        .fold(staff_state, |a, b| STAFF_AGGREGATE.apply(a, &b.data));

    match staff_updated_state {
        Some(p) => {
            upsert_staff(
                read_pool.clone(),
                p,
                read_events
                    .last()
                    .map_or_else(|| 0, |event| event.version.0),
                stream_id.clone(),
            )
            .await?;
            info!("Staff events processed successfully");
        }
        None => {
            error!("Staff with ID: {} not found", staff_id);
        }
    }

    info!("End processing staff events");

    Ok(())
}
