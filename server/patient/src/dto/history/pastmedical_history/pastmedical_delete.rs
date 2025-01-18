use common::dto::last_updated_input::LastUpdatedInput;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use validator::Validate;

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default, PartialEq, TS, Eq)]
#[ts(export, export_to = "../../bindings/PastMedicalHistoryDelete.ts")]
pub struct PastMedicalHistoryDelete {
    pub id: String,
    pub patient_id: String,
    pub last_updated_input: LastUpdatedInput,
}
