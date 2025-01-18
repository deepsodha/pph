use common::dto::gender::GenderType;
use common::dto::last_updated_input::LastUpdatedInput;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use validator::Validate;

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default, PartialEq, TS, Eq)]
#[ts(export, export_to = "../../bindings/SocialHistoryAdd.ts")]
pub struct SocialHistoryAdd {
    pub patient_id: String,
    pub birth_gender: GenderType,
    pub tobacco: Option<Vec<String>>,
    pub alcohol: Option<Vec<String>>,
    pub cardiovascular: Option<Vec<String>>,
    pub sexual_activity: Option<Vec<String>>,
    pub drug_abuse: Option<Vec<String>>,
    pub safety: Option<Vec<String>>,
    pub comments: Option<String>,
    pub last_updated_input: LastUpdatedInput,
}
