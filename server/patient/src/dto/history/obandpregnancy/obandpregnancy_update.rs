use serde::{Deserialize, Serialize};
use ts_rs::TS;
use validator::Validate;

use super::obandpregnancy_add::OBandPregnancyAdd;

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default, PartialEq, TS, Eq)]
#[ts(export, export_to = "../../bindings/OBandPregnancyUpdate.ts")]
pub struct OBandPregnancyUpdate {
    pub id: String,
    pub input: OBandPregnancyAdd,
}
