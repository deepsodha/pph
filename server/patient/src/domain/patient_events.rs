use super::patient_domain::{
    Create, Delete, PatientAddress, PatientGov, PatientPhone, PatientUser, Update,
};
use chrono::{DateTime, Utc};
use common::dto::{
    address::AddressInput, contact::ContactInput, gov_info::GovInfoInput, user::UserInput,
};
use cosmo_store::types::event_write::EventWrite;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PatientCreated {
    pub id: String,
    pub org_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub user: UserInput,
    pub address: AddressInput,
    pub phone: ContactInput,
    pub government_info: GovInfoInput,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PatientUpdated {
    pub id: String,
    pub org_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PatientDeleted {
    pub id: String,
    pub org_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PatientAddressUpdated {
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub address: AddressInput,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PatientUserDetailsUpdated {
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub user: UserInput,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PatientGovDetailsUpdated {
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub government_info: GovInfoInput,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PatientContactDetailsUpdated {
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub phone: ContactInput,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PatientEvent {
    PatientCreated(Box<PatientCreated>),
    PatientUpdated(PatientUpdated),
    PatientDeleted(PatientDeleted),
    PatientAddressUpdated(PatientAddressUpdated),
    PatientUserDetailsUpdated(Box<PatientUserDetailsUpdated>),
    PatientContactDetailsUpdated(PatientContactDetailsUpdated),
    PatientGovDetailsUpdated(PatientGovDetailsUpdated),
}

impl From<Create> for PatientCreated {
    fn from(s: Create) -> Self {
        PatientCreated {
            id: s.id,
            org_id: String::from(s.org_id.as_ref()),
            created_by: String::from(s.created_by.as_ref()),
            updated_by: String::from(s.updated_by.as_ref()),
            created_at: s.created_at,
            last_updated: s.last_updated,
            user: UserInput::from(s.user),
            address: AddressInput::from(s.address),
            phone: ContactInput::from(s.phone),
            government_info: GovInfoInput::from(s.government_info),
        }
    }
}

impl From<Update> for PatientUpdated {
    fn from(s: Update) -> Self {
        PatientUpdated {
            id: String::from(s.id.as_ref()),
            org_id: String::from(s.org_id.as_ref()),
            created_by: String::from(s.created_by.as_ref()),
            updated_by: String::from(s.updated_by.as_ref()),
            created_at: s.created_at,
            last_updated: s.last_updated,
        }
    }
}

impl From<PatientAddress> for PatientAddressUpdated {
    fn from(s: PatientAddress) -> Self {
        PatientAddressUpdated {
            created_by: String::from(s.created_by.as_ref()),
            updated_by: String::from(s.updated_by.as_ref()),
            created_at: s.created_at,
            last_updated: s.last_updated,
            address: s.address.into(),
        }
    }
}

impl From<PatientUser> for PatientUserDetailsUpdated {
    fn from(s: PatientUser) -> Self {
        PatientUserDetailsUpdated {
            created_by: String::from(s.created_by.as_ref()),
            updated_by: String::from(s.updated_by.as_ref()),
            created_at: s.created_at,
            last_updated: s.last_updated,
            user: s.user.into(),
        }
    }
}

impl From<PatientPhone> for PatientContactDetailsUpdated {
    fn from(s: PatientPhone) -> Self {
        PatientContactDetailsUpdated {
            created_by: String::from(s.created_by.as_ref()),
            updated_by: String::from(s.updated_by.as_ref()),
            created_at: s.created_at,
            last_updated: s.last_updated,
            phone: s.phone.into(),
        }
    }
}

impl From<PatientGov> for PatientGovDetailsUpdated {
    fn from(s: PatientGov) -> Self {
        PatientGovDetailsUpdated {
            created_by: String::from(s.created_by.as_ref()),
            updated_by: String::from(s.updated_by.as_ref()),
            created_at: s.created_at,
            last_updated: s.last_updated,
            government_info: s.government_info.into(),
        }
    }
}

impl From<Delete> for PatientDeleted {
    fn from(s: Delete) -> Self {
        PatientDeleted {
            org_id: String::from(s.org_id.as_ref()),
            id: String::from(s.id.as_ref()),
            created_by: String::from(s.created_by.as_ref()),
            updated_by: String::from(s.updated_by.as_ref()),
            created_at: s.created_at,
            last_updated: s.last_updated,
        }
    }
}

impl From<PatientEvent> for EventWrite<PatientEvent, PatientEvent> {
    fn from(u: PatientEvent) -> Self {
        EventWrite {
            id: Uuid::new_v4(),
            correlation_id: None,
            causation_id: None,
            name: String::from("patient_event"),
            data: u,
            metadata: None,
        }
    }
}
