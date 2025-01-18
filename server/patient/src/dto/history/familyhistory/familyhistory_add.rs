use crate::dto::history::health_status::HealthStauts;
use common::dto::last_updated_input::LastUpdatedInput;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use validator::Validate;

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default, PartialEq, TS, Eq)]
#[ts(export, export_to = "../../bindings/FamilyHistoryAdd.ts")]
pub struct FamilyHistoryAdd {
    pub patient_id: String,
    pub family_member: String,
    pub health_status: HealthStauts,
    pub general: Option<Vec<String>>,
    pub cancer: Option<Vec<String>>,
    pub comments: Option<Vec<String>>,
    pub last_updated_input: LastUpdatedInput,
}
