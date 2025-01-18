use chrono::{DateTime, Utc};
use cosmo_store::types::event_write::EventWrite;
use serde::{Deserialize, Serialize};

use crate::dto::history::health_status::HealthStauts;

use super::familyhistory_domain::{Create, Delete, Update};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct FamilyHistoryCreated {
    pub id: String,
    pub org_id: String,
    pub patient_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub family_member: String,
    pub health_status: HealthStauts,
    pub general: Option<Vec<String>>,
    pub cancer: Option<Vec<String>>,
    pub comments: Option<Vec<String>>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct FamilyHistoryUpdated {
    pub id: String,
    pub org_id: String,
    pub patient_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub family_member: String,
    pub health_status: HealthStauts,
    pub general: Option<Vec<String>>,
    pub cancer: Option<Vec<String>>,
    pub comments: Option<Vec<String>>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct FamilyHistoryDeleted {
    pub id: String,
    pub org_id: String,
    pub patient_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum FamilyHistoryEvent {
    FamilyHistoryCreated(FamilyHistoryCreated),
    FamilyHistoryUpdated(FamilyHistoryUpdated),
    FamilyHistoryDeleted(FamilyHistoryDeleted),
}

impl From<Create> for FamilyHistoryCreated {
    fn from(s: Create) -> Self {
        FamilyHistoryCreated {
            id: s.id,
            org_id: String::from(&s.org_id),
            patient_id: String::from(&s.patient_id),
            created_by: String::from(&s.created_by),
            updated_by: String::from(&s.updated_by),
            created_at: s.created_at,
            last_updated: s.last_updated,
            comments: s.comments,
            family_member: s.family_member,
            health_status: s.health_status,
            general: s.general,
            cancer: s.cancer,
        }
    }
}

impl From<Update> for FamilyHistoryUpdated {
    fn from(s: Update) -> Self {
        FamilyHistoryUpdated {
            id: s.id,
            org_id: String::from(&s.org_id),
            patient_id: String::from(&s.patient_id),
            created_by: String::from(&s.created_by),
            updated_by: String::from(&s.updated_by),
            created_at: s.created_at,
            last_updated: s.last_updated,
            comments: s.comments,
            family_member: s.family_member,
            health_status: s.health_status,
            general: s.general,
            cancer: s.cancer,
        }
    }
}

impl From<Delete> for FamilyHistoryDeleted {
    fn from(s: Delete) -> Self {
        FamilyHistoryDeleted {
            id: s.id,
            org_id: String::from(&s.org_id),
            patient_id: String::from(&s.patient_id),
            created_by: String::from(&s.created_by),
            updated_by: String::from(&s.updated_by),
            created_at: s.created_at,
            last_updated: s.last_updated,
        }
    }
}

impl From<FamilyHistoryEvent> for EventWrite<FamilyHistoryEvent, FamilyHistoryEvent> {
    fn from(u: FamilyHistoryEvent) -> Self {
        EventWrite {
            id: uuid::Uuid::new_v4(),
            correlation_id: None,
            causation_id: None,
            name: String::from("familyhistory_event"),
            data: u,
            metadata: None,
        }
    }
}
