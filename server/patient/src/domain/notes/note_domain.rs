use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::dto::notes::notes_types::{CurrentNoteState, NoteType};

use super::{note_command::CreateNote, note_events::NoteCreated};

#[derive(Clone, Debug, Serialize, Deserialize, TS, PartialEq, Eq)]
#[ts(export, export_to = "../../bindings/NoteState.ts")]
pub struct NoteState {
    pub id: String,
    pub org_id: String,
    pub patient_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub note: NoteType,
    pub note_state: CurrentNoteState,
    pub is_deleted: bool,
}

impl From<NoteCreated> for NoteState {
    fn from(u: NoteCreated) -> Self {
        NoteState {
            id: String::from(&u.id),
            org_id: String::from(&u.org_id),
            patient_id: String::from(&u.patient_id),
            created_by: String::from(&u.created_by),
            updated_by: String::from(&u.updated_by),
            created_at: u.created_at,
            last_updated: u.last_updated,
            note: u.note,
            note_state: u.note_state,
            is_deleted: false,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, TS)]
pub struct Note {
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

impl Note {
    pub fn parse(a: &CreateNote) -> Result<Note> {
        // Here all the validation will come
        Ok(Note {
            id: a.id.to_owned(),
            created_by: a.created_by.clone(),
            updated_by: a.updated_by.clone(),
            created_at: (a.created_at).to_owned(),
            last_updated: (a.last_updated).to_owned(),
            org_id: a.org_id.clone(),
            patient_id: a.patient_id.clone(),
            note: a.note.clone(),
            note_state: a.note_state.clone(),
        })
    }
}
