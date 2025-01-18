use serde::{Deserialize, Serialize};
use ts_rs::TS;
use validator::Validate;

use super::pastsurgicalhistory_add::PastSurgicalHistoryAdd;

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default, PartialEq, TS, Eq)]
#[ts(export, export_to = "../../bindings/PastSurgicalHistoryUpdate.ts")]
pub struct PastSurgicalHistoryUpdate {
    pub id: String,
    pub input: PastSurgicalHistoryAdd,
}
