use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, Debug, Clone, TS)]
#[ts(export, export_to = "../../bindings/ICD10.ts")]
pub struct ICD10 {
    #[serde(rename = "code")]
    pub code: String,
    #[serde(rename = "category")]
    pub category: Option<String>,
    #[serde(rename = "icd_code")]
    pub icd_code: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "long_description")]
    pub long_description: String,
    #[serde(rename = "id")]
    pub id: i64,
}
