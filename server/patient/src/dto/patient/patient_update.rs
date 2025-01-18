use serde::{Deserialize, Serialize};
use ts_rs::TS;
use validator::Validate;

use super::patient_add::PatientAdd;

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default, TS, PartialEq, Eq)]
#[ts(export, export_to = "../../bindings/PatientUpdate.ts")]
pub struct PatientUpdate {
    #[validate(length(min = 1, message = "field can't be empty"))]
    pub id: String,
    pub input: PatientAdd,
}
