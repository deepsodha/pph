use crate::doc::syncs::DataTable;
use anyhow::Result;
use cosmo_store::{common::i64_event_version::EventVersion, types::event_read::EventRead};
use cosmo_store_util::aggregate::Aggregate;
use db_services::services::obandpregnancy::upsert_obandpregnancy;
use log::{debug, error, info};
use patient::domain::history::obandpregnancy::{
    obandpregnancy_aggregate::OBANDPREGNANCY_AGGREGATE, obandpregnancy_domain::OBandPregnancyState,
    obandpregnancy_events::OBandPregnancyEvent,
};
use sqlx::{Pool, Sqlite};

pub async fn process_obandpregnancy_events(
    read_pool: Pool<Sqlite>,
    obandpregnancy_id: String,
    stream_id: String,
    read_events: Vec<EventRead<OBandPregnancyEvent, OBandPregnancyEvent, EventVersion>>,
) -> Result<()> {
    info!("Processing OB and Pregnancy events...");

    let obandpregnancy_db = sqlx::query_as::<_, DataTable>(
        "SELECT * FROM obandpregnancy_table_state WHERE id = ? LIMIT 1",
    )
    .bind(obandpregnancy_id.clone())
    .fetch_optional(&read_pool)
    .await?;

    let obandpregnancy_state: Option<OBandPregnancyState> =
        obandpregnancy_db.and_then(|data_table| serde_json::from_value(data_table.data).ok());

    let obandpregnancy_updated_state = read_events.iter().fold(obandpregnancy_state, |a, b| {
        OBANDPREGNANCY_AGGREGATE.apply(a, &b.data)
    });

    match obandpregnancy_updated_state {
        Some(p) => {
            debug!("Updating OB and Pregnancy state: {:?}", &p);
            upsert_obandpregnancy(
                read_pool,
                p,
                read_events
                    .last()
                    .map_or_else(|| 0, |event| event.version.0),
                stream_id,
            )
            .await?;
            info!("OB and Pregnancy events processed successfully.");
        }
        None => {
            error!("OB and Pregnancy with ID: {} not found", obandpregnancy_id);
        }
    }
    Ok(())
}
