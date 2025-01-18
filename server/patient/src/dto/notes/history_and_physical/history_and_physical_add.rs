use common::dto::last_updated_input::LastUpdatedInput;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use validator::Validate;

use crate::dto::notes::notes_types::CurrentNoteState;

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default, TS, PartialEq, Eq)]
#[ts(export, export_to = "../../bindings/HistoryAndPhysicalAdd.ts")]
pub struct HistoryAndPhysicalAdd {
    pub patient_id: String,
    pub chief_complaint: String,
    pub history_of_present_illness: String,
    pub past_medical_history: String,
    pub past_surgical_history: String,
    pub family_history: String,
    pub social_history: String,
    pub obstetric_and_pregnancy_history: String,
    pub hospitalizations: String,
    pub implantable_devices: String,
    pub review_of_systems: String,
    pub medications_id: Vec<String>,
    pub allergies_id: Vec<String>,
    pub mental_or_functional: String,
    pub vitals_id: String,
    pub exam: String,
    pub assessment: String,
    pub plan: String,
    pub minor_procedures: String,
    pub goals: String,
    pub health_concerns: String,
    pub note_state: CurrentNoteState,
    pub last_updated_input: LastUpdatedInput,
}
