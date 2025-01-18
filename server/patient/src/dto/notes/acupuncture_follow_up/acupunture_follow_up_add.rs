use common::dto::last_updated_input::LastUpdatedInput;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use validator::Validate;

use crate::dto::notes::notes_types::CurrentNoteState;

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default, TS, PartialEq, Eq)]
#[ts(export, export_to = "../../bindings/AcupunctureFollowUpAdd.ts")]
pub struct AcupunctureFollowUpAdd {
    pub patient_id: String,
    pub chief_complaint: String,
    pub medications_id: Vec<String>,
    pub allergies_id: Vec<String>,
    pub subjective: String,
    pub objective: String,
    pub last_updated_input: LastUpdatedInput,
    pub tcm_exam: String,
    pub assesment: String,
    pub plan: String,
    pub treatment: String,
    pub note_state: CurrentNoteState,
}
