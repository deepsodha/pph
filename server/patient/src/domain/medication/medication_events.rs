use chrono::{DateTime, Utc};
use common::dto::status::Status;
use cosmo_store::types::event_write::EventWrite;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::medication_domain::{Create, Delete, Update};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum MedicationsEvent {
    MedicationCreated(MedicationCreated),
    MedicationUpdated(MedicationUpdated),
    MedicationDeleted(MedicationDeleted),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MedicationCreated {
    pub id: String,
    pub org_id: String,
    pub patient_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub status: Status,
    pub drug: String,
    pub instruction: Option<String>,
    pub comments: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MedicationUpdated {
    pub id: String,
    pub org_id: String,
    pub patient_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub status: Status,
    pub drug: String,
    pub instruction: Option<String>,
    pub comments: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MedicationDeleted {
    pub id: String,
    pub org_id: String,
    pub patient_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}

impl From<MedicationsEvent> for EventWrite<MedicationsEvent, MedicationsEvent> {
    fn from(u: MedicationsEvent) -> Self {
        EventWrite {
            id: Uuid::new_v4(),
            correlation_id: None,
            causation_id: None,
            name: String::from("medications_event"),
            data: u,
            metadata: None,
        }
    }
}

impl From<Create> for MedicationCreated {
    fn from(s: Create) -> Self {
        MedicationCreated {
            id: s.id,
            created_by: String::from(s.created_by.as_ref()),
            updated_by: String::from(s.updated_by.as_ref()),
            created_at: s.created_at,
            last_updated: s.last_updated,
            org_id: String::from(s.org_id.as_ref()),
            patient_id: String::from(s.patient_id.as_ref()),
            status: s.status,
            drug: String::from(s.drug.as_ref()),
            instruction: s.instruction.map(|x| String::from(x.as_ref())),
            comments: String::from(s.comments.as_ref()),
        }
    }
}

impl From<Update> for MedicationUpdated {
    fn from(s: Update) -> Self {
        MedicationUpdated {
            id: String::from(s.id.as_ref()),
            org_id: String::from(s.org_id.as_ref()),
            patient_id: String::from(s.patient_id.as_ref()),
            status: s.status,
            drug: String::from(s.drug.as_ref()),
            instruction: s.instruction.map(|x| String::from(x.as_ref())),
            comments: String::from(s.comments.as_ref()),
            created_by: String::from(s.created_by.as_ref()),
            updated_by: String::from(s.updated_by.as_ref()),
            created_at: s.created_at,
            last_updated: s.last_updated,
        }
    }
}

impl From<Delete> for MedicationDeleted {
    fn from(s: Delete) -> Self {
        MedicationDeleted {
            id: String::from(s.id.as_ref()),
            org_id: String::from(s.org_id.as_ref()),
            patient_id: String::from(s.patient_id.as_ref()),
            created_by: String::from(s.created_by.as_ref()),
            updated_by: String::from(s.updated_by.as_ref()),
            created_at: s.created_at,
            last_updated: s.last_updated,
        }
    }
}
