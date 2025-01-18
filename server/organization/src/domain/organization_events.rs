use super::organization_domain::{Create, Delete, Select, Update};
use chrono::{DateTime, Utc};
use common::dto::address::AddressInput;
use cosmo_store::types::event_write::EventWrite;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum OrganizationEvent {
    OrganizationCreated(OrganizationCreated),
    OrganizationUpdated(OrganizationUpdated),
    OrganizationDeleted(OrganizationDeleted),
    OrganizationSelected(OrganizationSelected),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OrganizationCreated {
    pub id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub org_name: String,
    pub details: String,
    pub phone_number: String,
    pub email: String,
    pub address: AddressInput,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OrganizationUpdated {
    pub id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub org_name: String,
    pub details: String,
    pub phone_number: String,
    pub email: String,
    pub address: AddressInput,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OrganizationDeleted {
    pub id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OrganizationSelected {
    pub id: String,
    pub name: String,
    pub time: DateTime<Utc>,
}

impl From<OrganizationEvent> for EventWrite<OrganizationEvent, OrganizationEvent> {
    fn from(u: OrganizationEvent) -> Self {
        EventWrite {
            id: Uuid::new_v4(),
            correlation_id: None,
            causation_id: None,
            name: String::from("organization_event"),
            data: u,
            metadata: None,
        }
    }
}

impl From<Create> for OrganizationCreated {
    fn from(s: Create) -> Self {
        OrganizationCreated {
            id: s.id,
            created_by: String::from(s.created_by.as_ref()),
            updated_by: String::from(s.updated_by.as_ref()),
            created_at: s.created_at,
            last_updated: s.last_updated,
            org_name: String::from(s.name.as_ref()),
            details: String::from(s.details.as_ref()),
            phone_number: String::from(s.phone_number.as_ref()),
            email: String::from(s.email.as_ref()),
            address: s.address.into(),
        }
    }
}

impl From<Update> for OrganizationUpdated {
    fn from(s: Update) -> Self {
        OrganizationUpdated {
            id: String::from(s.id.as_ref()),
            created_by: String::from(s.created_by.as_ref()),
            updated_by: String::from(s.updated_by.as_ref()),
            created_at: s.created_at,
            last_updated: s.last_updated,
            org_name: String::from(s.name.as_ref()),
            details: String::from(s.details.as_ref()),
            phone_number: String::from(s.phone_number.as_ref()),
            email: String::from(s.email.as_ref()),
            address: s.address.into(),
        }
    }
}

impl From<Delete> for OrganizationDeleted {
    fn from(s: Delete) -> Self {
        OrganizationDeleted {
            id: String::from(s.id.as_ref()),
            created_by: String::from(s.created_by.as_ref()),
            updated_by: String::from(s.updated_by.as_ref()),
            created_at: s.created_at,
            last_updated: s.last_updated,
        }
    }
}

impl From<Select> for OrganizationSelected {
    fn from(s: Select) -> Self {
        OrganizationSelected {
            id: String::from(s.id.as_ref()),
            name: String::from(s.name.as_ref()),
            time: Utc::now(),
        }
    }
}
