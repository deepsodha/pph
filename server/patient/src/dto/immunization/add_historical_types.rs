use chrono::{DateTime, Utc};
use common::dto::last_updated_input::LastUpdatedInput;
use doctor::dto::doctor_type::DoctorType;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use validator::Validate;

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default, TS, PartialEq, Eq)]
#[ts(export, export_to = "../../bindings/HistoricalAdd.ts")]
pub struct HistoricalAdd {
    pub patient_id: String,
    pub vaccine: String,
    pub types: String,
    pub date: DateTime<Utc>,
    pub number_in_series: String,
    pub provider: DoctorType,
    pub source_of_information: String,
    pub comments: String,
    pub last_updated_input: LastUpdatedInput,
}

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default, TS, PartialEq, Eq)]
#[ts(export, export_to = "../../bindings/HistoricalUpdate.ts")]
pub struct HistoricalUpdate {
    pub id: String,
    pub input: HistoricalAdd,
}

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default, TS, PartialEq, Eq)]
#[ts(export, export_to = "../../bindings/HistoricalDelete.ts")]
pub struct HistoricalDelete {
    #[validate(length(min = 1, message = "field can't be empty"))]
    pub id: String,
    #[validate(length(min = 1, message = "field can't be empty"))]
    pub patient_id: String,
    pub last_updated_input: LastUpdatedInput,
}
