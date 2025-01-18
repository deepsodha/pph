use super::doctor_domain::{Create, Delete, Update};
use crate::dto::doctor_type::DoctorType;
use chrono::{DateTime, Utc};
use cosmo_store::types::event_write::EventWrite;
use serde::{Deserialize, Serialize};
use user::domain::user_domain::UserState;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DoctorCreated {
    pub id: String,
    pub user: UserState,
    pub org_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub doctor_role: Vec<DoctorType>,
    pub doctor_register_number: String,
    pub doctor_department: String,
    pub doctor_speciality: String,
    pub emergency: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DoctorUpdated {
    pub id: String,
    pub user: UserState,
    pub org_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub doctor_role: Vec<DoctorType>,
    pub doctor_register_number: String,
    pub doctor_department: String,
    pub doctor_speciality: String,
    pub emergency: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DoctorDeleted {
    pub id: String,
    pub org_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}

impl From<Create> for DoctorCreated {
    fn from(s: Create) -> Self {
        DoctorCreated {
            org_id: String::from(s.org_id.as_ref()),
            id: s.id,
            created_by: String::from(s.created_by.as_ref()),
            updated_by: String::from(s.updated_by.as_ref()),
            created_at: s.created_at,
            last_updated: s.last_updated,
            doctor_role: s.doctor_role.into(),
            doctor_register_number: String::from(s.doctor_register_number.as_ref()),
            doctor_department: String::from(s.doctor_department.as_ref()),
            doctor_speciality: String::from(s.doctor_speciality.as_ref()),
            emergency: s.emergency.into(),
            user: s.user,
        }
    }
}

impl From<Update> for DoctorUpdated {
    fn from(s: Update) -> Self {
        DoctorUpdated {
            org_id: String::from(s.org_id.as_ref()),
            id: String::from(s.id.as_ref()),
            user: s.user,
            created_by: String::from(s.created_by.as_ref()),
            updated_by: String::from(s.updated_by.as_ref()),
            created_at: s.created_at,
            last_updated: s.last_updated,
            doctor_role: s.doctor_role.into(),
            doctor_register_number: String::from(s.doctor_register_number.as_ref()),
            doctor_department: String::from(s.doctor_department.as_ref()),
            doctor_speciality: String::from(s.doctor_speciality.as_ref()),
            emergency: s.emergency.into(),
        }
    }
}

impl From<Delete> for DoctorDeleted {
    fn from(s: Delete) -> Self {
        DoctorDeleted {
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
pub enum DoctorEvent {
    DoctorCreated(Box<DoctorCreated>),
    DoctorUpdated(Box<DoctorUpdated>),
    DoctorDeleted(DoctorDeleted),
}

impl From<DoctorEvent> for EventWrite<DoctorEvent, DoctorEvent> {
    fn from(u: DoctorEvent) -> Self {
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
