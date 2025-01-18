use common::dto::last_updated_input::LastUpdatedInput;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use validator::Validate;

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default, TS, PartialEq, Eq)]
#[ts(export, export_to = "../../bindings/SystemAdminDelete.ts")]
pub struct SystemAdminDelete {
    pub id: String,
    pub org_id: String,
    pub last_updated_input: LastUpdatedInput,
}
