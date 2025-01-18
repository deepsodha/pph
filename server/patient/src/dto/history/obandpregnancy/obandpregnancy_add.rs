use common::dto::last_updated_input::LastUpdatedInput;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use validator::Validate;

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default, PartialEq, TS, Eq)]
#[ts(export, export_to = "../../bindings/OBandPregnancyAdd.ts")]
pub struct OBandPregnancyAdd {
    pub patient_id: String,
    pub age_onset_of_menses: u32,
    pub age_at_menopause: u32,
    pub comments_ob: Option<String>,
    pub total_pregnancy: Option<u32>,
    pub full_term: Option<u32>,
    pub pre_term: Option<u32>,
    pub miscarriages: Option<u32>,
    pub living: Option<u32>,
    pub comments_pregnancy: Option<String>,
    pub last_updated_input: LastUpdatedInput,
}
