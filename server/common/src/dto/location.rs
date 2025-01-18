use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, Debug, Clone, TS)]
#[ts(export, export_to = "../../bindings/Location.ts")]
pub struct Location {
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[serde(rename = "state_name")]
    pub state_name: String,
    #[serde(rename = "country_code")]
    pub country_code: Option<String>,
    #[serde(rename = "country_name")]
    pub country_name: String,
}
