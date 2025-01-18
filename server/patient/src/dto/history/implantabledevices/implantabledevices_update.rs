use serde::{Deserialize, Serialize};
use ts_rs::TS;
use validator::Validate;

use super::implantabledevices_add::ImplantableDevicesAdd;

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default, PartialEq, TS, Eq)]
#[ts(export, export_to = "../../bindings/ImplantableDevicesUpdate.ts")]
pub struct ImplantableDevicesUpdate {
    pub id: String,
    pub input: ImplantableDevicesAdd,
}
