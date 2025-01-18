use serde::{Deserialize, Serialize};
use ts_rs::TS;
use validator::Validate;

use super::medication_add::MedicationsAdd;

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default, TS, PartialEq, Eq)]
#[ts(export, export_to = "../../bindings/MedicationUpdate.ts")]
pub struct MedicationUpdate {
    pub id: String,
    pub input: MedicationsAdd,
}
