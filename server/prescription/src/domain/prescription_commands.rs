use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreatePrescription {
    pub id: String,
    pub patient_id: String,
    pub doctor_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub org_id: String,
    pub patient_name: String,
    pub presecription_name: String,
    pub instruction: Option<String>,
    pub date: DateTime<Utc>,
    pub drug_name: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdatedPrescription {
    pub id: String,
    pub org_id: String,
    pub patient_id: String,
    pub doctor_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub patient_name: String,
    pub presecription_name: String,
    pub instruction: Option<String>,
    pub date: DateTime<Utc>,
    pub drug_name: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeletePrescription {
    pub id: String,
    pub patient_id: String,
    pub doctor_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub org_id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PrescriptionCommand {
    CreatePrescription(CreatePrescription),
    UpdatedPrescription(UpdatedPrescription),
    DeletePrescription(DeletePrescription),
}
