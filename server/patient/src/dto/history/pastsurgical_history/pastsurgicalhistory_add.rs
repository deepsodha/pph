use common::dto::last_updated_input::LastUpdatedInput;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use validator::Validate;

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default, PartialEq, TS, Eq)]
#[ts(export, export_to = "../../bindings/PastSurgicalHistoryAdd.ts")]
pub struct PastSurgicalHistoryAdd {
    pub patient_id: String,
    pub common_surgeries: Option<Vec<String>>,
    pub comments: Option<String>,
    pub last_updated_input: LastUpdatedInput,
}
