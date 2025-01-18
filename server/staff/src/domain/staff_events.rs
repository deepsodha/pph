use chrono::{DateTime, Utc};
use cosmo_store::types::event_write::EventWrite;
use serde::{Deserialize, Serialize};
use user::domain::user_domain::UserState;
use uuid::Uuid;

use crate::dto::staff_type::StaffType;

use super::staff_domain::{Create, Delete, Update};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StaffCreated {
    pub id: String,
    pub org_id: String,
    pub user: UserState,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub staff_department: Vec<StaffType>,
    pub emergency: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StaffUpdated {
    pub id: String,
    pub org_id: String,
    pub user: UserState,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub staff_department: Vec<StaffType>,
    pub emergency: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StaffDeleted {
    pub id: String,
    pub org_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}

impl From<Create> for StaffCreated {
    fn from(s: Create) -> Self {
        StaffCreated {
            org_id: String::from(s.org_id.as_ref()),
            id: s.id,
            created_by: String::from(s.created_by.as_ref()),
            updated_by: String::from(s.updated_by.as_ref()),
            created_at: s.created_at,
            last_updated: s.last_updated,
            user: s.user,
            staff_department: s.staff_department.into(),
            emergency: s.emergency.into(),
        }
    }
}

impl From<Update> for StaffUpdated {
    fn from(s: Update) -> Self {
        StaffUpdated {
            org_id: String::from(s.org_id.as_ref()),
            id: String::from(s.id.as_ref()),
            created_by: String::from(s.created_by.as_ref()),
            updated_by: String::from(s.updated_by.as_ref()),
            created_at: s.created_at,
            last_updated: s.last_updated,
            staff_department: s.staff_department.into(),
            user: s.user,
            emergency: s.emergency.into(),
        }
    }
}

impl From<Delete> for StaffDeleted {
    fn from(s: Delete) -> Self {
        StaffDeleted {
            org_id: String::from(s.org_id.as_ref()),
            id: String::from(s.id.as_ref()),
            created_by: String::from(s.created_by.as_ref()),
            updated_by: String::from(s.updated_by.as_ref()),
            created_at: s.created_at,
            last_updated: s.last_updated,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum StaffEvent {
    StaffCreated(Box<StaffCreated>),
    StaffUpdated(Box<StaffUpdated>),
    StaffDeleted(StaffDeleted),
}

impl From<StaffEvent> for EventWrite<StaffEvent, StaffEvent> {
    fn from(u: StaffEvent) -> Self {
        EventWrite {
            id: Uuid::new_v4(),
            correlation_id: None,
            causation_id: None,
            name: String::from("doctor_event"),
            data: u,
            metadata: None,
        }
    }
}
