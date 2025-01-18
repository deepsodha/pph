use crate::doc::syncs::DataTable;
use anyhow::Result;
use cosmo_store::{common::i64_event_version::EventVersion, types::event_read::EventRead};
use cosmo_store_util::aggregate::Aggregate;
use db_services::services::notes::upsert_notes;
use log::{error, info};
use patient::domain::notes::{
    note_aggregate::NOTE_AGGREGATE, note_domain::NoteState, note_events::NoteEvent,
};
use sqlx::{Pool, Sqlite};

pub async fn process_notes_events(
    read_pool: Pool<Sqlite>,
    note_id: String,
    stream_id: String,
    read_events: Vec<EventRead<NoteEvent, NoteEvent, EventVersion>>,
) -> Result<()> {
    info!("Processing note events...");

    let note_db =
        sqlx::query_as::<_, DataTable>("SELECT * from  note_table_state WHERE id = ? LIMIT 1")
            .bind(note_id.clone())
            .fetch_optional(&read_pool)
            .await?;

    let note_state: Option<NoteState> =
        note_db.and_then(|data_table| serde_json::from_value(data_table.data).ok());

    let note_updated_state = read_events
        .iter()
        .fold(note_state, |a, b| NOTE_AGGREGATE.apply(a, &b.data));

    match note_updated_state {
        Some(p) => {
            info!("Updating note state...");
            upsert_notes(
                read_pool,
                p,
                read_events
                    .last()
                    .map_or_else(|| 0, |event| event.version.0),
                stream_id,
            )
            .await?;
            info!("Note state updated successfully.");
        }
        None => {
            error!("Note with ID: {} not found", note_id);
        }
    }
    Ok(())
}
