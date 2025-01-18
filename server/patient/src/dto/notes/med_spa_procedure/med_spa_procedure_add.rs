use common::dto::last_updated_input::LastUpdatedInput;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use validator::Validate;

use crate::dto::notes::notes_types::CurrentNoteState;

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default, TS, PartialEq, Eq)]
#[ts(export, export_to = "../../bindings/MedSpaProcedureAdd.ts")]
pub struct MedSpaProcedureAdd {
    pub patient_id: String,
    pub last_updated_input: LastUpdatedInput,
    pub procedure: String,
    pub performed_by: String,
    pub indication: String,
    pub comments: String,
    pub note_state: CurrentNoteState,
}
