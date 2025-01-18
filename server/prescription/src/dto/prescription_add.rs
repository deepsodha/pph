use chrono::{DateTime, Utc};
use common::dto::last_updated_input::LastUpdatedInput;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Deserialize, Serialize, Debug, Default, Clone, validator::Validate, PartialEq, TS, Eq)]
#[ts(export, export_to = "../../bindings/PrescriptionAdd.ts")]
pub struct PrescriptionAdd {
    #[validate(length(min = 1, message = "field can't be empty"))]
    pub patient_id: String,
    #[validate(length(min = 1, message = "field can't be empty"))]
    pub patient_name: String,
    #[validate(length(min = 1, message = "field can't be empty"))]
    pub presecription_name: String,
    #[validate(length(min = 1, message = "field can't be empty"))]
    pub instruction: Option<String>,
    pub date: DateTime<Utc>,
    #[validate(length(min = 1, message = "field can't be empty"))]
    pub drug_name: Vec<String>,
    pub last_updated_input: LastUpdatedInput,
}
