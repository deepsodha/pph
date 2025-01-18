use common::dto::last_updated_input::LastUpdatedInput;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use super::staff_type::StaffType;

#[derive(Deserialize, Serialize, Debug, Default, Clone, validator::Validate, TS, PartialEq, Eq)]
#[ts(export, export_to = "../../bindings/StaffAdd.ts")]
pub struct StaffAdd {
    pub user_id: String,
    pub staff_department: Vec<StaffType>,
    pub emergency: bool,
    pub last_updated_input: LastUpdatedInput,
}
