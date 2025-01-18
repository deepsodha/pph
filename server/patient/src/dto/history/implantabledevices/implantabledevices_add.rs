use common::dto::last_updated_input::LastUpdatedInput;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use validator::Validate;

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default, PartialEq, TS, Eq)]
#[ts(export, export_to = "../../bindings/ImplantableDevicesAdd.ts")]
pub struct ImplantableDevicesAdd {
    pub patient_id: String,
    pub status: Option<String>,
    pub udi: Option<String>,
    pub udi_unknown: Option<bool>,
    pub comments: Option<String>,
    pub last_updated_input: LastUpdatedInput,
}
