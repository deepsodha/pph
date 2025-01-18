use chrono::{DateTime, Utc};
use common::dto::last_updated_input::LastUpdatedInput;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use validator::Validate;

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default, PartialEq, TS, Eq)]
#[ts(export, export_to = "../../bindings/HospitalizationAdd.ts")]
pub struct HospitalizationAdd {
    pub patient_id: String,
    pub admission_date: DateTime<Utc>,
    pub related_to: Option<String>,
    pub status: Option<String>,
    pub length_of_stay: Option<u32>,
    pub procedure: Option<String>,
    pub comments: Option<String>,
    pub last_updated_input: LastUpdatedInput,
}
