use crate::doc::syncs::DataTable;
use anyhow::Result;
use cosmo_store::{common::i64_event_version::EventVersion, types::event_read::EventRead};
use cosmo_store_util::aggregate::Aggregate;
use db_services::services::doctor::upsert_doctor;
use doctor::domain::doctor_aggregate::DOCTOR_AGGREGATE;
use doctor::domain::doctor_domain::DoctorState;
use doctor::domain::doctor_events::DoctorEvent;
use log::error;
use sqlx::{Pool, Sqlite};

pub async fn process_doctor_events(
    read_pool: Pool<Sqlite>,
    doctor_id: String,
    stream_id: String,
    read_events: Vec<EventRead<DoctorEvent, DoctorEvent, EventVersion>>,
) -> Result<()> {
    log::info!("Processing doctor events...");

    let doctor_db =
        sqlx::query_as::<_, DataTable>("SELECT * from doctor_table_state WHERE id = ? LIMIT 1")
            .bind(doctor_id.clone())
            .fetch_optional(&read_pool)
            .await?;

    let doctor_state: Option<DoctorState> =
        doctor_db.and_then(|data_table| serde_json::from_value(data_table.data).ok());

    let doctor_updated_state = read_events
        .iter()
        .fold(doctor_state, |a, b| DOCTOR_AGGREGATE.apply(a, &b.data));

    match doctor_updated_state {
        Some(p) => {
            log::info!("Doctor state updated successfully.");
            upsert_doctor(
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
            error!("Doctor with ID: {} not found", doctor_id);
        }
    }
    log::info!("Processing doctor events completed.");
    Ok(())
}
