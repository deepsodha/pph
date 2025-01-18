use super::pastmedical_add::PastMedicalHistoryAdd;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use validator::Validate;

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default, PartialEq, TS, Eq)]
#[ts(export, export_to = "../../bindings/PastMedicalHistoryUpdate.ts")]
pub struct PastMedicalHistoryUpdate {
    pub id: String,
    pub input: PastMedicalHistoryAdd,
}
