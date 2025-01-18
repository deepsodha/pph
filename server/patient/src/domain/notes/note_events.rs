use crate::dto::notes::notes_types::{CurrentNoteState, NoteType};
use chrono::{DateTime, Utc};
use cosmo_store::types::event_write::EventWrite;
use serde::{Deserialize, Serialize};

use super::note_domain::Note;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NoteCreated {
    pub id: String,
    pub org_id: String,
    pub patient_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub note: NoteType,
    pub note_state: CurrentNoteState,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum NoteEvent {
    NoteCreated(NoteCreated),
}

impl From<Note> for NoteCreated {
    fn from(s: Note) -> Self {
        NoteCreated {
            id: String::from(&s.id),
            org_id: String::from(&s.org_id),
            patient_id: String::from(&s.patient_id),
            created_by: String::from(&s.created_by),
            updated_by: String::from(&s.updated_by),
            created_at: s.created_at,
            last_updated: s.last_updated,
            note: s.note,
            note_state: s.note_state,
        }
    }
}

impl From<NoteEvent> for EventWrite<NoteEvent, NoteEvent> {
    fn from(u: NoteEvent) -> Self {
        EventWrite {
            id: uuid::Uuid::new_v4(),
            correlation_id: None,
            causation_id: None,
            name: String::from("note_event"),
            data: u,
            metadata: None,
        }
    }
}
