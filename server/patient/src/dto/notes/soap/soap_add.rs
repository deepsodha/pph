use common::dto::last_updated_input::LastUpdatedInput;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use validator::Validate;

use crate::dto::notes::notes_types::CurrentNoteState;

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default, TS, PartialEq, Eq)]
#[ts(export, export_to = "../../bindings/SoapAdd.ts")]
pub struct SoapAdd {
    pub patient_id: String,
    pub chief_complaint: String,
    pub subjective: String,
    pub medications_id: Vec<String>,
    pub allergies_id: Vec<String>,
    pub mental_or_functional: String,
    pub vitals_id: String,
    pub objective: String,
    pub assessment: String,
    pub plan: String,
    pub note_state: CurrentNoteState,
    pub last_updated_input: LastUpdatedInput,
}
