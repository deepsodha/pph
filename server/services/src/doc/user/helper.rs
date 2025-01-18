use crate::doc::syncs::DataTable;
use anyhow::Result;
use cosmo_store::{common::i64_event_version::EventVersion, types::event_read::EventRead};
use cosmo_store_util::aggregate::Aggregate;
use db_services::services::user::upsert_user;
use log::{error, info};
use sqlx::{Pool, Sqlite};
use user::domain::{
    user_aggregate::USER_AGGREGATE, user_domain::UserState, user_events::UserEvent,
};

pub async fn process_user_events(
    read_pool: Pool<Sqlite>,
    user_id: String,
    stream_id: String,
    read_events: Vec<EventRead<UserEvent, UserEvent, EventVersion>>,
) -> Result<()> {
    info!("Start processing user events");

    let user_db =
        sqlx::query_as::<_, DataTable>("SELECT * from user_table_state WHERE id = ? LIMIT 1")
            .bind(user_id.clone())
            .fetch_optional(&read_pool)
            .await?;

    let user_state: Option<UserState> =
        user_db.and_then(|data_table| serde_json::from_value(data_table.data).ok());

    println!("user state ............{:?}", user_state);
    let user_updated_state = read_events
        .iter()
        .fold(user_state, |a, b| USER_AGGREGATE.apply(a, &b.data));

    println!("user updated state ............{:?}", user_updated_state);
    match user_updated_state {
        Some(p) => {
            upsert_user(
                read_pool.clone(),
                p,
                read_events
                    .last()
                    .map_or_else(|| 0, |event| event.version.0),
                stream_id.clone(),
            )
            .await?;
            info!("User events processed successfully");
        }
        None => {
            error!("User with ID: {} not found", user_id);
        }
    }

    info!("End processing user events");

    Ok(())
}
