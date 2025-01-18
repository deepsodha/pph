use super::staff_add::StaffAdd;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use validator::Validate;

#[derive(Deserialize, Serialize, Debug, Default, Clone, Validate, TS, PartialEq, Eq)]
#[ts(export, export_to = "../../bindings/StaffUpdate.ts")]
pub struct StaffUpdate {
    #[validate(length(min = 1, message = "field can't be empty"))]
    pub id: String,
    pub input: StaffAdd,
}
