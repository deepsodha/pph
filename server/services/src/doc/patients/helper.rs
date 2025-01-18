use crate::doc::syncs::DataTable;
use anyhow::Result;
use cosmo_store::{common::i64_event_version::EventVersion, types::event_read::EventRead};
use cosmo_store_util::aggregate::Aggregate;
use db_services::services::patient::upsert_patient;
use log::{error, info};
use patient::domain::{
    patient_aggregate::PATIENT_AGGREGATE, patient_domain::PatientState,
    patient_events::PatientEvent,
};
use sqlx::{Pool, Sqlite};

pub async fn process_patient_events(
    read_pool: Pool<Sqlite>,
    patient_id: String,
    stream_id: String,
    read_events: Vec<EventRead<PatientEvent, PatientEvent, EventVersion>>,
) -> Result<()> {
    info!("Processing patient events");

    let patient_db =
        sqlx::query_as::<_, DataTable>("SELECT * from patient_table_state WHERE id = ? LIMIT 1")
            .bind(patient_id.clone())
            .fetch_optional(&read_pool)
            .await?;

    let patient_state: Option<PatientState> =
        patient_db.and_then(|data_table| serde_json::from_value(data_table.data).ok());

    let patient_updated_state = read_events
        .iter()
        .fold(patient_state, |a, b| PATIENT_AGGREGATE.apply(a, &b.data));

    match patient_updated_state {
        Some(p) => {
            info!("Patient state updated successfully");
            upsert_patient(
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
            error!("Patients with ID: {} not found", patient_id);
        }
    }

    Ok(())
}
