use crate::doc::syncs::DataTable;
use anyhow::Result;
use cosmo_store::{common::i64_event_version::EventVersion, types::event_read::EventRead};
use cosmo_store_util::aggregate::Aggregate;
use db_services::services::systemadmin::upsert_systemadmin;
use log::{error, info};
use sqlx::{Pool, Sqlite};
use system_admin::domain::{
    systemadmin_aggregate::SYSTEMADMIN_AGGREGATE, systemadmin_domain::SystemAdminState,
    systemadmin_events::SystemAdminEvent,
};

pub async fn process_systemadmin_events(
    read_pool: Pool<Sqlite>,
    systemadmin_id: String,
    org_id: String,
    org_name: String,
    stream_id: String,
    read_events: Vec<EventRead<SystemAdminEvent, SystemAdminEvent, EventVersion>>,
) -> Result<()> {
    info!("Start processing systemadmin events");

    let systemadmin_db = sqlx::query_as::<_, DataTable>(
        "SELECT * from systemadmin_table_state WHERE id = ? LIMIT 1",
    )
    .bind(systemadmin_id.clone())
    .fetch_optional(&read_pool)
    .await?;

    let systemadmin_state: Option<SystemAdminState> =
        systemadmin_db.and_then(|data_table| serde_json::from_value(data_table.data).ok());

    let systemadmin_updated_state = read_events.iter().fold(systemadmin_state, |a, b| {
        SYSTEMADMIN_AGGREGATE.apply(a, &b.data)
    });

    match systemadmin_updated_state {
        Some(p) => {
            upsert_systemadmin(
                read_pool.clone(),
                p,
                org_id,
                org_name,
                read_events
                    .last()
                    .map_or_else(|| 0, |event| event.version.0),
                stream_id.clone(),
            )
            .await?;
            info!("Systemadmin events processed successfully");
        }
        None => {
            error!("SystemAdmin with ID: {} not found", systemadmin_id);
        }
    }

    info!("End processing systemadmin events");

    Ok(())
}
