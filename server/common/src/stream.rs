use chrono::{DateTime, Utc};
use serde_json::Value;
use sqlx::FromRow;

#[derive(Debug, FromRow, Default)]
pub struct Stream {
    pub id: String,
    pub stream_id: String,
    pub version: i64,
    pub created_by: String,
    pub created_at: DateTime<Utc>,
    pub data: Value,
}
