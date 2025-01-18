use crate::dto::history::health_status::HealthStauts;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default)]
pub struct CreateFamilyHistory {
    pub id: String,
    pub org_id: String,
    pub patient_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub family_member: String,
    pub health_status: HealthStauts,
    pub general: Option<Vec<String>>,
    pub cancer: Option<Vec<String>>,
    pub comments: Option<Vec<String>>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default)]
pub struct UpdateFamilyHistory {
    pub id: String,
    pub org_id: String,
    pub patient_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub family_member: String,
    pub health_status: HealthStauts,
    pub general: Option<Vec<String>>,
    pub cancer: Option<Vec<String>>,
    pub comments: Option<Vec<String>>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default)]
pub struct DeleteFamilyHistory {
    pub id: String,
    pub org_id: String,
    pub patient_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum FamilyHistoryCommand {
    CreateFamilyHistory(CreateFamilyHistory),
    UpdateFamilyHistory(UpdateFamilyHistory),
    DeleteFamilyHistory(DeleteFamilyHistory),
}
