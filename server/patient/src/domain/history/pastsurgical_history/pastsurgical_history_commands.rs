use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default)]
pub struct CreatePastSurgicalHistory {
    pub id: String,
    pub org_id: String,
    pub patient_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub common_surgeries: Option<Vec<String>>,
    pub comments: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default)]
pub struct UpdatePastSurgicalHistory {
    pub id: String,
    pub org_id: String,
    pub patient_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub common_surgeries: Option<Vec<String>>,
    pub comments: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default)]
pub struct DeletePastSurgicalHistory {
    pub id: String,
    pub org_id: String,
    pub patient_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PastSurgicalCommand {
    CreatePastSurgicalHistory(CreatePastSurgicalHistory),
    UpdatePastSurgicalHistory(UpdatePastSurgicalHistory),
    DeletePastSurgicalHistory(DeletePastSurgicalHistory),
}
