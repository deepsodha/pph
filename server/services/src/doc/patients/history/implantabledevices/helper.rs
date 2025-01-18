use crate::doc::syncs::DataTable;
use anyhow::Result;
use cosmo_store::{common::i64_event_version::EventVersion, types::event_read::EventRead};
use cosmo_store_util::aggregate::Aggregate;
use db_services::services::implantabledevices::upsert_implantabledevices;
use log::{debug, error, info};
use patient::domain::history::implantabledevices::{
    implantable_devices_aggregate::IMPLANTABLEDEVICES_AGGREGATE,
    implantable_devices_domain::ImplantableDevicesState,
    implantable_devices_events::ImplantableDevicesEvent,
};
use sqlx::{Pool, Sqlite};
pub async fn process_implantabledevices_events(
    read_pool: Pool<Sqlite>,
    implantabledevices_id: String,
    stream_id: String,
    read_events: Vec<EventRead<ImplantableDevicesEvent, ImplantableDevicesEvent, EventVersion>>,
) -> Result<()> {
    info!("Processing implantable devices events...");

    let implantabledevices_db = sqlx::query_as::<_, DataTable>(
        "SELECT * FROM implantabledevices_table_state WHERE id = ? LIMIT 1",
    )
    .bind(implantabledevices_id.clone())
    .fetch_optional(&read_pool)
    .await?;

    let implantabledevices_state: Option<ImplantableDevicesState> =
        implantabledevices_db.and_then(|data_table| serde_json::from_value(data_table.data).ok());

    debug!(
        "Implantable devices state retrieved: {:?}",
        implantabledevices_state
    );

    let implantabledeivces_updated_state =
        read_events.iter().fold(implantabledevices_state, |a, b| {
            IMPLANTABLEDEVICES_AGGREGATE.apply(a, &b.data)
        });

    debug!(
        "Updated implantable devices state: {:?}",
        implantabledeivces_updated_state
    );

    match implantabledeivces_updated_state {
        Some(p) => {
            debug!("Upserting implantable devices state...");
            upsert_implantabledevices(
                read_pool,
                p,
                read_events
                    .last()
                    .map_or_else(|| 0, |event| event.version.0),
                stream_id,
            )
            .await?;
            info!("Implantable devices events processed successfully.");
        }
        None => {
            error!(
                "Implantable devices with ID: {} not found",
                implantabledevices_id
            );
        }
    }
    Ok(())
}
