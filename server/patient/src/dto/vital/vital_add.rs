use chrono::{DateTime, Utc};
use common::dto::last_updated_input::LastUpdatedInput;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use validator::Validate;

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default, TS, PartialEq, Eq)]
#[ts(export, export_to = "../../bindings/VitalsAdd.ts")]
pub struct VitalsAdd {
    pub patient_id: String,
    pub date_time: Option<DateTime<Utc>>,
    pub blood_pressure: Option<u32>,
    pub heart_rate: Option<u32>,
    pub height: Option<u32>,
    pub weight: Option<u32>,
    pub temperature: Option<u32>,
    pub bmi: Option<u32>,
    #[validate(length(min = 1, message = "field can't be empty"))]
    pub comments: Option<String>,
    pub last_updated_input: LastUpdatedInput,
}
