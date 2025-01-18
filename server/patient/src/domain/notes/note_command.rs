use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::dto::notes::notes_types::{CurrentNoteState, NoteType};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateNote {
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
pub enum NoteCommand {
    CreateNote(CreateNote),
}
