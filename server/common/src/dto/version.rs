use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../bindings/Version.ts")]
pub struct Version {
    pub version: String,
    pub info: String,
    pub updates: Vec<String>,
    pub updated_date: DateTime<Utc>,
}
