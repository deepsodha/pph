use common::dto::last_updated_input::LastUpdatedInput;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use validator::Validate;

use crate::dto::notes::notes_types::CurrentNoteState;

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default, TS, PartialEq, Eq)]
#[ts(export, export_to = "../../bindings/AmendmentAdd.ts")]
pub struct AmendmentAdd {
    pub patient_id: String,
    pub last_updated_input: LastUpdatedInput,
    pub source_of_request: String,
    pub request_details: String,
    pub decision: String,
    pub note_state: CurrentNoteState,
}
