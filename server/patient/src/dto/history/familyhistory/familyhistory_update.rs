use super::familyhistory_add::FamilyHistoryAdd;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use validator::Validate;

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default, PartialEq, TS, Eq)]
#[ts(export, export_to = "../../bindings/FamilyHistoryUpdate.ts")]
pub struct FamilyHistoryUpdate {
    pub id: String,
    pub input: FamilyHistoryAdd,
}
