use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default)]
pub struct CreateHospitalization {
    pub id: String,
    pub org_id: String,
    pub patient_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub admission_date: DateTime<Utc>,
    pub related_to: Option<String>,
    pub status: Option<String>,
    pub length_of_stay: Option<u32>,
    pub procedure: Option<String>,
    pub comments: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default)]
pub struct UpdateHospitalization {
    pub id: String,
    pub org_id: String,
    pub patient_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub admission_date: DateTime<Utc>,
    pub related_to: Option<String>,
    pub status: Option<String>,
    pub length_of_stay: Option<u32>,
    pub procedure: Option<String>,
    pub comments: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default)]
pub struct DeleteHospitalization {
    pub id: String,
    pub org_id: String,
    pub patient_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum HospitalizationCommand {
    CreateHospitalization(CreateHospitalization),
    UpdateHospitalization(UpdateHospitalization),
    DeleteHospitalization(DeleteHospitalization),
}
