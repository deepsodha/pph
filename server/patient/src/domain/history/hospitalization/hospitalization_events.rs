use chrono::{DateTime, Utc};
use cosmo_store::types::event_write::EventWrite;
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::hospitalization_domain::{Create, Delete, Update};

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default)]
pub struct HospitalizationCreated {
    pub id: String,
    pub org_id: String,
    pub patient_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub admission_date: DateTime<Utc>,
    pub related_to: Option<String>,
    pub status: Option<String>,
    pub length_of_stay: Option<u32>,
    pub procedure: Option<String>,
    pub comments: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default)]
pub struct HospitalizationUpdated {
    pub id: String,
    pub org_id: String,
    pub patient_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub admission_date: DateTime<Utc>,
    pub related_to: Option<String>,
    pub status: Option<String>,
    pub length_of_stay: Option<u32>,
    pub procedure: Option<String>,
    pub comments: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default)]
pub struct HospitalizationDeleted {
    pub id: String,
    pub org_id: String,
    pub patient_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum HospitalizationEvent {
    HospitalizationCreated(HospitalizationCreated),
    HospitalizationUpdated(HospitalizationUpdated),
    HospitalizationDeleted(HospitalizationDeleted),
}

impl From<Create> for HospitalizationCreated {
    fn from(s: Create) -> Self {
        HospitalizationCreated {
            id: s.id,
            org_id: String::from(&s.org_id),
            patient_id: String::from(&s.patient_id),
            created_by: String::from(&s.created_by),
            updated_by: String::from(&s.updated_by),
            created_at: s.created_at,
            last_updated: s.last_updated,
            comments: s.comments,
            admission_date: s.admission_date,
            related_to: s.related_to,
            status: s.status,
            length_of_stay: s.length_of_stay,
            procedure: s.procedure,
        }
    }
}

impl From<Update> for HospitalizationUpdated {
    fn from(s: Update) -> Self {
        HospitalizationUpdated {
            id: s.id,
            org_id: String::from(&s.org_id),
            patient_id: String::from(&s.patient_id),
            created_by: String::from(&s.created_by),
            updated_by: String::from(&s.updated_by),
            created_at: s.created_at,
            last_updated: s.last_updated,
            comments: s.comments,
            admission_date: s.admission_date,
            related_to: s.related_to,
            status: s.status,
            length_of_stay: s.length_of_stay,
            procedure: s.procedure,
        }
    }
}

impl From<Delete> for HospitalizationDeleted {
    fn from(s: Delete) -> Self {
        HospitalizationDeleted {
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

impl From<HospitalizationEvent> for EventWrite<HospitalizationEvent, HospitalizationEvent> {
    fn from(u: HospitalizationEvent) -> Self {
        EventWrite {
            id: uuid::Uuid::new_v4(),
            correlation_id: None,
            causation_id: None,
            name: String::from("hospitalization_event"),
            data: u,
            metadata: None,
        }
    }
}
