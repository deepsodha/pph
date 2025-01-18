use chrono::{DateTime, Utc};
use cosmo_store::types::event_write::EventWrite;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::domain::{Create, Delete, Update};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AddressEvent {
    AddressCreated(AddressCreated),
    AddressUpdated(AddressUpdated),
    AddressDeleted(AddressDeleted),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AddressCreated {
    pub id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub pin_code: String,
    pub city: String,
    pub state: String,
    pub address_line: String,
    pub country: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AddressUpdated {
    pub id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub pin_code: String,
    pub city: String,
    pub state: String,
    pub address_line: String,
    pub country: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AddressDeleted {
    pub id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}

impl From<AddressEvent> for EventWrite<AddressEvent, AddressEvent> {
    fn from(u: AddressEvent) -> Self {
        EventWrite {
            id: Uuid::new_v4(),
            correlation_id: None,
            causation_id: None,
            name: String::from("appointment_event"),
            data: u,
            metadata: None,
        }
    }
}

impl From<Create> for AddressCreated {
    fn from(s: Create) -> Self {
        AddressCreated {
            id: s.id,
            created_by: String::from(s.created_by.as_ref()),
            updated_by: String::from(s.updated_by.as_ref()),
            created_at: s.created_at,
            last_updated: s.last_updated,
            pin_code: String::from(s.pin_code.as_ref()),
            city: String::from(s.city.as_ref()),
            state: String::from(s.state.as_ref()),
            address_line: String::from(s.address_line.as_ref()),
            country: String::from(s.country.as_ref()),
        }
    }
}

impl From<Update> for AddressUpdated {
    fn from(s: Update) -> Self {
        AddressUpdated {
            id: String::from(s.id.as_ref()),
            created_by: String::from(s.created_by.as_ref()),
            updated_by: String::from(s.updated_by.as_ref()),
            created_at: s.created_at,
            last_updated: s.last_updated,
            pin_code: String::from(s.pin_code.as_ref()),
            city: String::from(s.city.as_ref()),
            state: String::from(s.state.as_ref()),
            address_line: String::from(s.address_line.as_ref()),
            country: String::from(s.country.as_ref()),
        }
    }
}

impl From<Delete> for AddressDeleted {
    fn from(s: Delete) -> Self {
        AddressDeleted {
            id: String::from(s.id.as_ref()),
            created_by: String::from(s.created_by.as_ref()),
            updated_by: String::from(s.updated_by.as_ref()),
            created_at: s.created_at,
            last_updated: s.last_updated,
        }
    }
}
