use super::hospitalization_add::HospitalizationAdd;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use validator::Validate;

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default, PartialEq, TS, Eq)]
#[ts(export, export_to = "../../bindings/HospitalizationUpdate.ts")]
pub struct HospitalizationUpdate {
    pub id: String,
    pub input: HospitalizationAdd,
}
