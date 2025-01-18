use common::dto::last_updated_input::LastUpdatedInput;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use validator::Validate;

use crate::dto::notes::notes_types::CurrentNoteState;

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default, TS, PartialEq, Eq)]
#[ts(export, export_to = "../../bindings/GroupAdd.ts")]
pub struct GroupAdd {
    pub patient_id: String,
    pub last_updated_input: LastUpdatedInput,
    pub group_session_content: String,
    pub session_details: String,
    pub individual_behavior_during_session: String,
    pub dsm_5: String,
    pub assesment: String,
    pub plan: String,
    pub note_state: CurrentNoteState,
}
