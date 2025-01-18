use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, sqlx::FromRow)]
pub struct DataTable {
    pub id: String,
    pub org_id: String,
    pub stream_id: String,
    pub version: u32,
    pub data: serde_json::Value,
    pub last_updated: DateTime<Utc>,
}
