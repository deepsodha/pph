use crate::dto::allergies::allergy_severities::AllergySeveritiesType;
use chrono::{DateTime, NaiveDate, Utc};
use common::dto::status::Status;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateAllergies {
    pub id: String,
    pub patient_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub org_id: String,
    pub allergen: String,
    pub reaction: String,
    pub allergy_severities: AllergySeveritiesType,
    pub input_date: NaiveDate,
    pub status: Status,
    pub comments: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateAllergies {
    pub id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub patient_id: String,
    pub org_id: String,
    pub allergen: String,
    pub reaction: String,
    pub allergy_severities: AllergySeveritiesType,
    pub input_date: NaiveDate,
    pub status: Status,
    pub comments: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeleteAllergies {
    pub id: String,
    pub patient_id: String,
    pub org_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AllergiesCommand {
    CreateAllergies(CreateAllergies),
    UpdateAllergies(UpdateAllergies),
    DeleteAllergies(DeleteAllergies),
}
