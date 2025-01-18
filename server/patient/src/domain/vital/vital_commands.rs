use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum VitalsCommand {
    CreateVitals(CreateVitals),
    UpdateVitals(UpdateVitals),
    DeleteVitals(DeleteVitals),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateVitals {
    pub id: String,
    pub org_id: String,
    pub patient_id: String,
    pub doctor_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub date_time: Option<DateTime<Utc>>,
    pub blood_pressure: Option<u32>,
    pub heart_rate: Option<u32>,
    pub height: Option<u32>,
    pub weight: Option<u32>,
    pub bmi: Option<u32>,
    pub temperature: Option<u32>,
    pub comments: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateVitals {
    pub id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub org_id: String,
    pub patient_id: String,
    pub doctor_id: String,
    pub date_time: Option<DateTime<Utc>>,
    pub blood_pressure: Option<u32>,
    pub heart_rate: Option<u32>,
    pub height: Option<u32>,
    pub weight: Option<u32>,
    pub bmi: Option<u32>,
    pub temperature: Option<u32>,
    pub comments: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeleteVitals {
    pub id: String,
    pub org_id: String,
    pub patient_id: String,
    pub doctor_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}
