use serde::{Deserialize, Serialize};
use ts_rs::TS;

use super::prescription_add::PrescriptionAdd;

#[derive(Deserialize, Serialize, Debug, Default, Clone, validator::Validate, PartialEq, TS, Eq)]
#[ts(export, export_to = "../../bindings/PrescriptionUpdate.ts")]
pub struct PrescriptionUpdate {
    #[validate(length(min = 1, message = "field can't be empty"))]
    pub id: String,
    pub input: PrescriptionAdd,
}
