use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default)]
pub struct CreateImplantableDevices {
    pub id: String,
    pub org_id: String,
    pub patient_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub status: Option<String>,
    pub udi: Option<String>,
    pub udi_unknown: Option<bool>,
    pub comments: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default)]
pub struct UpdateImplantableDevices {
    pub id: String,
    pub org_id: String,
    pub patient_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub status: Option<String>,
    pub udi: Option<String>,
    pub udi_unknown: Option<bool>,
    pub comments: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default)]
pub struct DeleteImplantableDevices {
    pub id: String,
    pub org_id: String,
    pub patient_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ImplantableDevicesCommand {
    CreateImplantableDevices(CreateImplantableDevices),
    UpdateImplantableDevices(UpdateImplantableDevices),
    DeleteImplantableDevices(DeleteImplantableDevices),
}
