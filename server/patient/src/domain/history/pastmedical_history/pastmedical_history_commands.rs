use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default)]
pub struct CreatePastMedicalHistory {
    pub id: String,
    pub org_id: String,
    pub patient_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub blood_type: Option<String>,
    pub head: Option<Vec<String>>,
    pub respiratory: Option<Vec<String>>,
    pub musculoskeletal: Option<Vec<String>>,
    pub endocrine: Option<Vec<String>>,
    pub eyes: Option<Vec<String>>,
    pub gastrointestinal: Option<Vec<String>>,
    pub skin: Option<Vec<String>>,
    pub ears: Option<Vec<String>>,
    pub noses: Option<Vec<String>>,
    pub neurological: Option<Vec<String>>,
    pub heme: Option<Vec<String>>,
    pub mouth: Option<Vec<String>>,
    pub infectious: Option<Vec<String>>,
    pub cardiovascular: Option<Vec<String>>,
    pub genitourinary: Option<Vec<String>>,
    pub psychiatric: Option<Vec<String>>,
    pub comments: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default)]
pub struct UpdatePastMedicalHistory {
    pub id: String,
    pub org_id: String,
    pub patient_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub blood_type: Option<String>,
    pub head: Option<Vec<String>>,
    pub respiratory: Option<Vec<String>>,
    pub musculoskeletal: Option<Vec<String>>,
    pub endocrine: Option<Vec<String>>,
    pub eyes: Option<Vec<String>>,
    pub gastrointestinal: Option<Vec<String>>,
    pub skin: Option<Vec<String>>,
    pub ears: Option<Vec<String>>,
    pub noses: Option<Vec<String>>,
    pub neurological: Option<Vec<String>>,
    pub heme: Option<Vec<String>>,
    pub mouth: Option<Vec<String>>,
    pub infectious: Option<Vec<String>>,
    pub cardiovascular: Option<Vec<String>>,
    pub genitourinary: Option<Vec<String>>,
    pub psychiatric: Option<Vec<String>>,
    pub comments: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default)]
pub struct DeletePastMedicalHistory {
    pub id: String,
    pub org_id: String,
    pub patient_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PastMedicalHistoryCommand {
    CreatePastMedicalHistory(CreatePastMedicalHistory),
    UpdatePastMedicalHistory(UpdatePastMedicalHistory),
    DeletePastMedicalHistory(DeletePastMedicalHistory),
}
