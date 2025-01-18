use chrono::{DateTime, NaiveTime, Utc};
use common::dto::address::AddressInput;
use cosmo_store::types::event_write::EventWrite;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::service_location_domain::{Create, Delete, Select, Update};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ServiceLocationCreated {
    pub id: String,
    pub org_id: String,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
    pub service_location_name: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub address: AddressInput,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ServiceLocationUpdated {
    pub id: String,
    pub org_id: String,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
    pub service_location_name: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub address: AddressInput,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ServiceLocationDeleted {
    pub id: String,
    pub org_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ServiceLocationSelected {
    pub id: String,
    pub name: String,
    pub time: DateTime<Utc>,
}

impl From<Create> for ServiceLocationCreated {
    fn from(s: Create) -> Self {
        ServiceLocationCreated {
            org_id: String::from(s.org_id.as_ref()),
            id: s.id,
            service_location_name: String::from(s.service_location_name.as_ref()),
            created_by: String::from(s.created_by.as_ref()),
            updated_by: String::from(s.updated_by.as_ref()),
            created_at: s.created_at,
            last_updated: s.last_updated,
            start_time: s.start_time,
            end_time: s.end_time,
            address: AddressInput::from(s.address),
        }
    }
}

impl From<Update> for ServiceLocationUpdated {
    fn from(s: Update) -> Self {
        ServiceLocationUpdated {
            id: String::from(s.id.as_ref()),
            org_id: String::from(s.org_id.as_ref()),
            created_by: String::from(s.created_by.as_ref()),
            updated_by: String::from(s.updated_by.as_ref()),
            service_location_name: String::from(s.service_location_name.as_ref()),
            created_at: s.created_at,
            last_updated: s.last_updated,
            start_time: s.start_time,
            end_time: s.end_time,
            address: AddressInput::from(s.address),
        }
    }
}

impl From<Delete> for ServiceLocationDeleted {
    fn from(s: Delete) -> Self {
        ServiceLocationDeleted {
            org_id: String::from(s.org_id.as_ref()),
            id: String::from(s.id.as_ref()),
            created_by: String::from(s.created_by.as_ref()),
            updated_by: String::from(s.updated_by.as_ref()),
            created_at: s.created_at,
            last_updated: s.last_updated,
        }
    }
}
impl From<Select> for ServiceLocationSelected {
    fn from(s: Select) -> Self {
        ServiceLocationSelected {
            id: String::from(s.id.as_ref()),
            name: String::from(s.name.as_ref()),
            time: Utc::now(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ServiceLocationEvent {
    ServiceLocationCreated(ServiceLocationCreated),
    ServiceLocationUpdated(ServiceLocationUpdated),
    ServiceLocationDeleted(ServiceLocationDeleted),
    ServiceLocationSelected(ServiceLocationSelected),
}

impl From<ServiceLocationEvent> for EventWrite<ServiceLocationEvent, ServiceLocationEvent> {
    fn from(u: ServiceLocationEvent) -> Self {
        EventWrite {
            id: Uuid::new_v4(),
            correlation_id: None,
            causation_id: None,
            name: String::from("servicelocation_event"),
            data: u,
            metadata: None,
        }
    }
}
