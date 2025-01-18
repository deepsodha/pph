use chrono::{DateTime, NaiveTime, Utc};
use common::dto::address::AddressInput;
use serde::{Deserialize, Serialize};

use crate::dto::select_servicelocation::SelectServiceLocation;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateSerivceLocation {
    pub id: String,
    pub org_id: String,
    pub service_location_name: String,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub address: AddressInput,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateServiceLocation {
    pub id: String,
    pub org_id: String,
    pub service_location_name: String,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub address: AddressInput,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeleteServiceLocation {
    pub id: String,
    pub org_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ServiceLocationSelect {
    pub id: String,
    pub name: String,
}

impl From<SelectServiceLocation> for ServiceLocationSelect {
    fn from(u: SelectServiceLocation) -> Self {
        ServiceLocationSelect {
            id: String::from(&u.id),
            name: String::from(&u.name),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ServiceLocationCommand {
    CreateSerivceLocation(CreateSerivceLocation),
    UpdateServiceLocation(UpdateServiceLocation),
    DeleteServiceLocation(DeleteServiceLocation),
    ServiceLocationSelect(ServiceLocationSelect),
}
