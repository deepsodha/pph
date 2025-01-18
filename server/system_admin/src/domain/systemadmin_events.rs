use chrono::{DateTime, Utc};
use common::dto::{
    contact::ContactInput, gov_info::GovInfoInput, user::UserInput, user_role::SystemAdminRole,
};
use cosmo_store::types::event_write::EventWrite;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::systemadmin_domain::{
    Create, Delete, SystemAdminGov, SystemAdminPhone, SystemAdminUser, Update,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SystemAdminEvent {
    SystemAdminCreated(Box<SystemAdminCreated>),
    SystemAdminUpdated(SystemAdminUpdated),
    SystemAdminDeleted(SystemAdminDeleted),
    SystemAdminUserDetailsUpdated(Box<SystemAdminUserDetailsUpdated>),
    SystemAdminContactDetailsUpdated(SystemAdminContactDetailsUpdated),
    SystemAdminGovDetailsUpdated(SystemAdminGovDetailsUpdated),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SystemAdminCreated {
    pub id: String,
    pub org_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub user: UserInput,
    pub phone: ContactInput,
    pub government_info: GovInfoInput,
    pub roles: Vec<SystemAdminRole>,
    pub password: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SystemAdminUpdated {
    pub id: String,
    pub org_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub roles: Vec<SystemAdminRole>,
    pub password: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SystemAdminUserDetailsUpdated {
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub user: UserInput,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SystemAdminGovDetailsUpdated {
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub government_info: GovInfoInput,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SystemAdminContactDetailsUpdated {
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub phone: ContactInput,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SystemAdminDeleted {
    pub id: String,
    pub org_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}

impl From<SystemAdminEvent> for EventWrite<SystemAdminEvent, SystemAdminEvent> {
    fn from(u: SystemAdminEvent) -> Self {
        EventWrite {
            id: Uuid::new_v4(),
            correlation_id: None,
            causation_id: None,
            name: String::from("systemadmin_event"),
            data: u,
            metadata: None,
        }
    }
}

impl From<Create> for SystemAdminCreated {
    fn from(s: Create) -> Self {
        SystemAdminCreated {
            id: s.id,
            org_id: String::from(s.org_id.as_ref()),
            created_by: String::from(s.created_by.as_ref()),
            updated_by: String::from(s.updated_by.as_ref()),
            created_at: s.created_at,
            last_updated: s.last_updated,
            user: UserInput::from(s.user),
            phone: ContactInput::from(s.phone),
            government_info: GovInfoInput::from(s.government_info),
            roles: s.roles.into(),
            password: String::from(s.password.as_ref()),
        }
    }
}

impl From<Update> for SystemAdminUpdated {
    fn from(s: Update) -> Self {
        SystemAdminUpdated {
            id: String::from(s.id.as_ref()),
            org_id: String::from(s.org_id.as_ref()),
            created_by: String::from(s.created_by.as_ref()),
            updated_by: String::from(s.updated_by.as_ref()),
            created_at: s.created_at,
            last_updated: s.last_updated,
            roles: s.roles.into(),
            password: s.password,
        }
    }
}

impl From<SystemAdminUser> for SystemAdminUserDetailsUpdated {
    fn from(s: SystemAdminUser) -> Self {
        SystemAdminUserDetailsUpdated {
            created_by: String::from(s.created_by.as_ref()),
            updated_by: String::from(s.updated_by.as_ref()),
            created_at: s.created_at,
            last_updated: s.last_updated,
            user: s.user.into(),
        }
    }
}

impl From<SystemAdminPhone> for SystemAdminContactDetailsUpdated {
    fn from(s: SystemAdminPhone) -> Self {
        SystemAdminContactDetailsUpdated {
            created_by: String::from(s.created_by.as_ref()),
            updated_by: String::from(s.updated_by.as_ref()),
            created_at: s.created_at,
            last_updated: s.last_updated,
            phone: s.phone.into(),
        }
    }
}

impl From<SystemAdminGov> for SystemAdminGovDetailsUpdated {
    fn from(s: SystemAdminGov) -> Self {
        SystemAdminGovDetailsUpdated {
            created_by: String::from(s.created_by.as_ref()),
            updated_by: String::from(s.updated_by.as_ref()),
            created_at: s.created_at,
            last_updated: s.last_updated,
            government_info: s.government_info.into(),
        }
    }
}

impl From<Delete> for SystemAdminDeleted {
    fn from(s: Delete) -> Self {
        SystemAdminDeleted {
            id: String::from(s.id.as_ref()),
            org_id: String::from(s.org_id.as_ref()),
            created_by: String::from(s.created_by.as_ref()),
            updated_by: String::from(s.updated_by.as_ref()),
            created_at: s.created_at,
            last_updated: s.last_updated,
        }
    }
}
