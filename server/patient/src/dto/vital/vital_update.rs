use serde::{Deserialize, Serialize};
use ts_rs::TS;
use validator::Validate;

use super::vital_add::VitalsAdd;

#[derive(Deserialize, Serialize, Debug, Clone, Default, PartialEq, TS, Validate, Eq)]
#[ts(export, export_to = "../../bindings/VitalsUpdate.ts")]
pub struct VitalsUpdate {
    #[validate(length(min = 1, message = "field can't be empty"))]
    pub id: String,
    pub input: VitalsAdd,
}
