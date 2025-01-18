use serde::{Deserialize, Serialize};
use ts_rs::TS;
use validator::Validate;

use super::system_admin_add::SystemAdminAdd;

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default, TS, PartialEq, Eq)]
#[ts(export, export_to = "../../bindings/SystemAdminUpdate.ts")]
pub struct SystemAdminUpdate {
    pub id: String,
    pub org_id: String,
    pub input: SystemAdminAdd,
}
