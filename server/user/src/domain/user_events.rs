use chrono::{DateTime, Utc};
use common::dto::{
    address::AddressInput, contact::ContactInput, gov_info::GovInfoInput, user::UserInput,
};
use cosmo_store::types::event_write::EventWrite;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::user_domain::{
    Create, Delete, UserAddressUpdate, UserContactUpdate, UserDetailsUpdate, UserGovUpdate,
    UserPasswordUpdate,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserCreated {
    pub id: String,
    pub org_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub user: UserInput,
    pub phone: ContactInput,
    pub address: AddressInput,
    pub government_info: GovInfoInput,
    pub password: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserPasswordUpdated {
    pub id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub password: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserContactUpdated {
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub phone: ContactInput,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserAddressUpdated {
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub address: AddressInput,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserGovDetailsUpdated {
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub government_info: GovInfoInput,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserDetailsUpdated {
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub user: UserInput,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserDeleted {
    pub id: String,
    pub org_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum UserEvent {
    UserCreated(Box<UserCreated>),
    UserPasswordUpdated(UserPasswordUpdated),
    UserContactUpdated(UserContactUpdated),
    UserAddressUpdated(UserAddressUpdated),
    UserGovDetailsUpdated(UserGovDetailsUpdated),
    UserDetailsUpdated(UserDetailsUpdated),
    UserDeleted(UserDeleted),
}

impl From<Create> for UserCreated {
    fn from(s: Create) -> Self {
        UserCreated {
            id: s.id,
            org_id: String::from(s.org_id.as_ref()),
            password: String::from(s.password.as_ref()),
            created_by: String::from(s.created_by.as_ref()),
            updated_by: String::from(s.updated_by.as_ref()),
            created_at: s.created_at,
            last_updated: s.last_updated,
            phone: ContactInput::from(s.phone),
            user: UserInput::from(s.user),
            address: AddressInput::from(s.address),
            government_info: GovInfoInput::from(s.government_info),
        }
    }
}

impl From<UserContactUpdate> for UserContactUpdated {
    fn from(s: UserContactUpdate) -> Self {
        UserContactUpdated {
            created_by: String::from(s.created_by.as_ref()),
            updated_by: String::from(s.updated_by.as_ref()),
            created_at: s.created_at,
            last_updated: s.last_updated,
            phone: ContactInput::from(s.phone),
        }
    }
}

impl From<UserPasswordUpdate> for UserPasswordUpdated {
    fn from(s: UserPasswordUpdate) -> Self {
        UserPasswordUpdated {
            id: String::from(s.id.as_ref()),
            created_at: s.created_at,
            created_by: String::from(s.created_by.as_ref()),
            updated_by: String::from(s.updated_by.as_ref()),
            last_updated: s.last_updated,
            password: String::from(s.password.as_ref()),
        }
    }
}

impl From<UserAddressUpdate> for UserAddressUpdated {
    fn from(s: UserAddressUpdate) -> Self {
        UserAddressUpdated {
            created_by: String::from(s.created_by.as_ref()),
            updated_by: String::from(s.updated_by.as_ref()),
            created_at: s.created_at,
            last_updated: s.last_updated,
            address: s.address.into(),
        }
    }
}

impl From<UserGovUpdate> for UserGovDetailsUpdated {
    fn from(s: UserGovUpdate) -> Self {
        UserGovDetailsUpdated {
            created_by: String::from(s.created_by.as_ref()),
            updated_by: String::from(s.updated_by.as_ref()),
            created_at: s.created_at,
            last_updated: s.last_updated,
            government_info: s.government_info.into(),
        }
    }
}

impl From<UserDetailsUpdate> for UserDetailsUpdated {
    fn from(s: UserDetailsUpdate) -> Self {
        UserDetailsUpdated {
            created_by: String::from(s.created_by.as_ref()),
            updated_by: String::from(s.updated_by.as_ref()),
            created_at: s.created_at,
            last_updated: s.last_updated,
            user: s.user.into(),
        }
    }
}
impl From<Delete> for UserDeleted {
    fn from(s: Delete) -> Self {
        UserDeleted {
            id: String::from(s.id.as_ref()),
            org_id: String::from(s.org_id.as_ref()),
            created_by: String::from(s.created_by.as_ref()),
            updated_by: String::from(s.updated_by.as_ref()),
            created_at: s.created_at,
            last_updated: s.last_updated,
        }
    }
}

impl From<UserEvent> for EventWrite<UserEvent, UserEvent> {
    fn from(u: UserEvent) -> Self {
        EventWrite {
            id: Uuid::new_v4(),
            correlation_id: None,
            causation_id: None,
            name: String::from("user_event"),
            data: u,
            metadata: None,
        }
    }
}
