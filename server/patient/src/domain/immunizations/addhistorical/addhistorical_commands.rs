use chrono::{DateTime, Utc};
use doctor::dto::doctor_type::DoctorType;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum HistoricalCommand {
    CreateHistorical(CreateHistorical),
    UpdateHistorical(UpdateHistorical),
    DeleteHistorical(DeleteHistorical),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateHistorical {
    pub id: String,
    pub org_id: String,
    pub patient_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub vaccine: String,
    pub types: String,
    pub date: DateTime<Utc>,
    pub number_in_series: String,
    pub provider: DoctorType,
    pub source_of_information: String,
    pub comments: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateHistorical {
    pub id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub org_id: String,
    pub patient_id: String,
    pub vaccine: String,
    pub types: String,
    pub date: DateTime<Utc>,
    pub number_in_series: String,
    pub provider: DoctorType,
    pub source_of_information: String,
    pub comments: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeleteHistorical {
    pub id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub org_id: String,
    pub patient_id: String,
}
