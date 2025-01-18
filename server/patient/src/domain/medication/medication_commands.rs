use chrono::{DateTime, Utc};
use common::dto::status::Status;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum MedicationsCommand {
    CreateMedication(CreateMedication),
    UpdateMedication(UpdateMedication),
    DeleteMedication(DeleteMedication),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateMedication {
    pub id: String,
    pub org_id: String,
    pub patient_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub status: Status,
    pub drug: String,
    pub instruction: Option<String>,
    pub comments: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateMedication {
    pub id: String,
    pub org_id: String,
    pub patient_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub status: Status,
    pub drug: String,
    pub instruction: Option<String>,
    pub comments: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeleteMedication {
    pub id: String,
    pub org_id: String,
    pub patient_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}
