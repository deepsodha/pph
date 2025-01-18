use common::dto::last_updated_input::LastUpdatedInput;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use validator::Validate;

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default, PartialEq, TS, Eq)]
#[ts(export, export_to = "../../bindings/PastMedicalHistoryAdd.ts")]
pub struct PastMedicalHistoryAdd {
    pub patient_id: String,
    pub blood_type: Option<String>,
    pub head: Option<Vec<String>>,
    pub respiratory: Option<Vec<String>>,
    pub musculoskeletal: Option<Vec<String>>,
    pub endocrine: Option<Vec<String>>,
    pub eyes: Option<Vec<String>>,
    pub gastrointestinal: Option<Vec<String>>,
    pub skin: Option<Vec<String>>,
    pub ears: Option<Vec<String>>,
    pub noses: Option<Vec<String>>,
    pub neurological: Option<Vec<String>>,
    pub heme: Option<Vec<String>>,
    pub mouth: Option<Vec<String>>,
    pub infectious: Option<Vec<String>>,
    pub cardiovascular: Option<Vec<String>>,
    pub genitourinary: Option<Vec<String>>,
    pub psychiatric: Option<Vec<String>>,
    pub comments: Option<String>,
    pub last_updated_input: LastUpdatedInput,
}
