use chrono::{DateTime, Utc};
use common::dto::types::Types;
use cosmo_store::types::event_write::EventWrite;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::notadministered_domain::{Create, Delete, Update};
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum NotAdministerEvent {
    NotAdministeredCreated(NotAdministeredCreated),
    NotAdministeredUpdated(NotAdministeredUpdated),
    NotAdministeredDeleted(NotAdministeredDeleted),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NotAdministeredCreated {
    pub id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub org_id: String,
    pub patient_id: String,
    pub vaccine: String,
    pub types: Types,
    pub recorded: DateTime<Utc>,
    pub reason_for_non_administration: String,
    pub comments: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NotAdministeredUpdated {
    pub id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub org_id: String,
    pub patient_id: String,
    pub vaccine: String,
    pub types: Types,
    pub recorded: DateTime<Utc>,
    pub reason_for_non_administration: String,
    pub comments: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NotAdministeredDeleted {
    pub id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub org_id: String,
    pub patient_id: String,
}

impl From<NotAdministerEvent> for EventWrite<NotAdministerEvent, NotAdministerEvent> {
    fn from(u: NotAdministerEvent) -> Self {
        EventWrite {
            id: Uuid::new_v4(),
            correlation_id: None,
            causation_id: None,
            name: String::from("not_administer_event"),
            data: u,
            metadata: None,
        }
    }
}

impl From<Create> for NotAdministeredCreated {
    fn from(s: Create) -> Self {
        NotAdministeredCreated {
            id: s.id,
            created_by: String::from(s.created_by.as_ref()),
            updated_by: String::from(s.updated_by.as_ref()),
            created_at: s.created_at,
            last_updated: s.last_updated,
            org_id: String::from(s.org_id.as_ref()),
            patient_id: String::from(s.patient_id.as_ref()),
            vaccine: String::from(s.vaccine.as_ref()),
            types: s.types,
            recorded: s.recorded,
            reason_for_non_administration: String::from(s.reason_for_non_administration.as_ref()),
            comments: String::from(s.comments.as_ref()),
        }
    }
}

impl From<Update> for NotAdministeredUpdated {
    fn from(s: Update) -> Self {
        NotAdministeredUpdated {
            id: String::from(s.id.as_ref()),
            created_by: String::from(s.created_by.as_ref()),
            updated_by: String::from(s.updated_by.as_ref()),
            created_at: s.created_at,
            last_updated: s.last_updated,
            org_id: String::from(s.org_id.as_ref()),
            patient_id: String::from(s.patient_id.as_ref()),
            vaccine: String::from(s.vaccine.as_ref()),
            types: s.types,
            recorded: s.recorded,
            reason_for_non_administration: String::from(s.reason_for_non_administration.as_ref()),
            comments: String::from(s.comments.as_ref()),
        }
    }
}

impl From<Delete> for NotAdministeredDeleted {
    fn from(s: Delete) -> Self {
        NotAdministeredDeleted {
            id: String::from(s.id.as_ref()),
            created_by: String::from(s.created_by.as_ref()),
            updated_by: String::from(s.updated_by.as_ref()),
            created_at: s.created_at,
            last_updated: s.last_updated,
            org_id: String::from(s.org_id.as_ref()),
            patient_id: String::from(s.patient_id.as_ref()),
        }
    }
}
