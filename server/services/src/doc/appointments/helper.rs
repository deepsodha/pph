use crate::doc::syncs::DataTable;
use anyhow::Result;
use appointment::domain::{
    appointment_aggregate::APPOINTMENTS_AGGREGATE, appointment_domain::AppointmentState,
    appointment_events::AppointmentEvent,
};
use cosmo_store::{common::i64_event_version::EventVersion, types::event_read::EventRead};
use cosmo_store_util::aggregate::Aggregate;
use db_services::services::appointment::upsert_appointment;
use log::{error, info};
use sqlx::sqlite::SqlitePool;

pub async fn process_appointment_events(
    read_pool: SqlitePool,
    appointment_id: String,
    stream_id: String,
    read_events: Vec<EventRead<AppointmentEvent, AppointmentEvent, EventVersion>>,
) -> Result<()> {
    info!("Starting process_appointment_events function...");

    let appointment_db = sqlx::query_as::<_, DataTable>(
        "SELECT * from  appointment_table_state WHERE id = ? LIMIT 1",
    )
    .bind(appointment_id.clone())
    .fetch_optional(&read_pool)
    .await?;

    let appointment_state: Option<AppointmentState> =
        appointment_db.and_then(|data_table| serde_json::from_value(data_table.data).ok());

    let appointment_updated_state = read_events.iter().fold(appointment_state, |a, b| {
        APPOINTMENTS_AGGREGATE.apply(a, &b.data)
    });

    match appointment_updated_state {
        Some(p) => {
            info!("Appointment state updated successfully.");

            let last_event_version = read_events
                .last()
                .map_or_else(|| 0, |event| event.version.0);

            upsert_appointment(read_pool, p, last_event_version, stream_id).await?;
        }
        None => {
            error!("Appointment with ID: {} not found", appointment_id);
        }
    }

    info!("process_appointment_events function completed successfully");
    Ok(())
}
