use common::dto::last_updated_input::LastUpdatedInput;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use validator::Validate;

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default, PartialEq, TS, Eq)]
#[ts(export, export_to = "../../bindings/ImplantableDevicesDelete.ts")]
pub struct ImplantableDevicesDelete {
    pub id: String,
    pub patient_id: String,
    pub last_updated_input: LastUpdatedInput,
}
