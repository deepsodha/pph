use common::dto::last_updated_input::LastUpdatedInput;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use validator::Validate;

use crate::dto::notes::notes_types::CurrentNoteState;

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default, TS, PartialEq, Eq)]
#[ts(export, export_to = "../../bindings/OfficeFormAdd.ts")]
pub struct OfficeFormAdd {
    pub patient_id: String,
    pub form: String,
    pub note_state: CurrentNoteState,
    pub last_updated_input: LastUpdatedInput,
}
