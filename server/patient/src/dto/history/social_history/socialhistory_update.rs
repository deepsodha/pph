use super::socialhistory_add::SocialHistoryAdd;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use validator::Validate;

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default, PartialEq, TS, Eq)]
#[ts(export, export_to = "../../bindings/SocialHistoryUpdate.ts")]
pub struct SocialHistoryUpdate {
    pub id: String,
    pub input: SocialHistoryAdd,
}
