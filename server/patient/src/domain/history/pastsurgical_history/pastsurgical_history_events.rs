use chrono::{DateTime, Utc};
use cosmo_store::types::event_write::EventWrite;
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::pastsurgical_history_domain::{Create, Delete, Update};

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default)]
pub struct PastSurgicalHistoryCreated {
    pub id: String,
    pub org_id: String,
    pub patient_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub common_surgeries: Option<Vec<String>>,
    pub comments: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default)]
pub struct PastSurgicalHistoryUpdated {
    pub id: String,
    pub org_id: String,
    pub patient_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub common_surgeries: Option<Vec<String>>,
    pub comments: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default)]
pub struct PastSurgicalHistoryDeleted {
    pub id: String,
    pub org_id: String,
    pub patient_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PastSurgicalEvent {
    PastSurgicalHistoryCreated(PastSurgicalHistoryCreated),
    PastSurgicalHistoryUpdated(PastSurgicalHistoryUpdated),
    PastSurgicalHistoryDeleted(PastSurgicalHistoryDeleted),
}

impl From<Create> for PastSurgicalHistoryCreated {
    fn from(s: Create) -> Self {
        PastSurgicalHistoryCreated {
            id: s.id,
            org_id: String::from(&s.org_id),
            patient_id: String::from(&s.patient_id),
            created_by: String::from(&s.created_by),
            updated_by: String::from(&s.updated_by),
            created_at: s.created_at,
            last_updated: s.last_updated,
            common_surgeries: s.common_surgeries,
            comments: s.comments,
        }
    }
}

impl From<Update> for PastSurgicalHistoryUpdated {
    fn from(s: Update) -> Self {
        PastSurgicalHistoryUpdated {
            id: s.id,
            org_id: String::from(&s.org_id),
            patient_id: String::from(&s.patient_id),
            created_by: String::from(&s.created_by),
            updated_by: String::from(&s.updated_by),
            created_at: s.created_at,
            last_updated: s.last_updated,
            common_surgeries: s.common_surgeries,
            comments: s.comments,
        }
    }
}

impl From<Delete> for PastSurgicalHistoryDeleted {
    fn from(s: Delete) -> Self {
        PastSurgicalHistoryDeleted {
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

impl From<PastSurgicalEvent> for EventWrite<PastSurgicalEvent, PastSurgicalEvent> {
    fn from(u: PastSurgicalEvent) -> Self {
        EventWrite {
            id: uuid::Uuid::new_v4(),
            correlation_id: None,
            causation_id: None,
            name: String::from("pastsurgicalhistory_event"),
            data: u,
            metadata: None,
        }
    }
}
