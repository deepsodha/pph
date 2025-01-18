use chrono::{DateTime, Utc};
use common::dto::types::Types;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum NotAdministerCommand {
    CreateNotAdministered(CreateNotAdministered),
    UpdateNotAdministered(UpdateNotAdministered),
    DeleteNotAdministered(DeleteNotAdministered),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateNotAdministered {
    pub id: String,
    pub org_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub patient_id: String,
    pub vaccine: String,
    pub types: Types,
    pub recorded: DateTime<Utc>,
    pub reason_for_non_administration: String,
    pub comments: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateNotAdministered {
    pub id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub org_id: String,
    pub patient_id: String,
    pub vaccine: String,
    pub types: Types,
    pub recorded: DateTime<Utc>,
    pub reason_for_non_administration: String,
    pub comments: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeleteNotAdministered {
    pub id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub org_id: String,
    pub patient_id: String,
}
