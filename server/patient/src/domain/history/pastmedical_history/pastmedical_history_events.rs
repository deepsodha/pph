use super::pastmedical_history_domain::{Create, Delete, Update};
use chrono::{DateTime, Utc};
use cosmo_store::types::event_write::EventWrite;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default)]
pub struct PastMedicalHistoryCreated {
    pub id: String,
    pub org_id: String,
    pub patient_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub blood_type: Option<String>,
    pub head: Option<Vec<String>>,
    pub respiratory: Option<Vec<String>>,
    pub musculoskeletal: Option<Vec<String>>,
    pub endocrine: Option<Vec<String>>,
    pub eyes: Option<Vec<String>>,
    pub gastrointestinal: Option<Vec<String>>,
    pub skin: Option<Vec<String>>,
    pub ears: Option<Vec<String>>,
    pub noses: Option<Vec<String>>,
    pub neurological: Option<Vec<String>>,
    pub heme: Option<Vec<String>>,
    pub mouth: Option<Vec<String>>,
    pub infectious: Option<Vec<String>>,
    pub cardiovascular: Option<Vec<String>>,
    pub genitourinary: Option<Vec<String>>,
    pub psychiatric: Option<Vec<String>>,
    pub comments: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default)]
pub struct PastMedicalHistoryUpdated {
    pub id: String,
    pub org_id: String,
    pub patient_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub blood_type: Option<String>,
    pub head: Option<Vec<String>>,
    pub respiratory: Option<Vec<String>>,
    pub musculoskeletal: Option<Vec<String>>,
    pub endocrine: Option<Vec<String>>,
    pub eyes: Option<Vec<String>>,
    pub gastrointestinal: Option<Vec<String>>,
    pub skin: Option<Vec<String>>,
    pub ears: Option<Vec<String>>,
    pub noses: Option<Vec<String>>,
    pub neurological: Option<Vec<String>>,
    pub heme: Option<Vec<String>>,
    pub mouth: Option<Vec<String>>,
    pub infectious: Option<Vec<String>>,
    pub cardiovascular: Option<Vec<String>>,
    pub genitourinary: Option<Vec<String>>,
    pub psychiatric: Option<Vec<String>>,
    pub comments: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default)]
pub struct PastMedicalHistoryDeleted {
    pub id: String,
    pub org_id: String,
    pub patient_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PastMedicalHistoryEvent {
    PastMedicalHistoryCreated(PastMedicalHistoryCreated),
    PastMedicalHistoryUpdated(PastMedicalHistoryUpdated),
    PastMedicalHistoryDeleted(PastMedicalHistoryDeleted),
}

impl From<Create> for PastMedicalHistoryCreated {
    fn from(s: Create) -> Self {
        PastMedicalHistoryCreated {
            id: s.id,
            org_id: String::from(&s.org_id),
            patient_id: String::from(&s.patient_id),
            created_by: String::from(&s.created_by),
            updated_by: String::from(&s.updated_by),
            created_at: s.created_at,
            last_updated: s.last_updated,
            blood_type: s.blood_type,
            head: s.head,
            respiratory: s.respiratory,
            musculoskeletal: s.musculoskeletal,
            endocrine: s.endocrine,
            eyes: s.eyes,
            gastrointestinal: s.gastrointestinal,
            skin: s.skin,
            ears: s.ears,
            noses: s.noses,
            neurological: s.neurological,
            heme: s.heme,
            mouth: s.mouth,
            infectious: s.infectious,
            cardiovascular: s.cardiovascular,
            genitourinary: s.genitourinary,
            psychiatric: s.psychiatric,
            comments: s.comments,
        }
    }
}

impl From<Update> for PastMedicalHistoryUpdated {
    fn from(s: Update) -> Self {
        PastMedicalHistoryUpdated {
            id: s.id,
            org_id: String::from(&s.org_id),
            patient_id: String::from(&s.patient_id),
            created_by: String::from(&s.created_by),
            updated_by: String::from(&s.updated_by),
            created_at: s.created_at,
            last_updated: s.last_updated,
            blood_type: s.blood_type,
            head: s.head,
            respiratory: s.respiratory,
            musculoskeletal: s.musculoskeletal,
            endocrine: s.endocrine,
            eyes: s.eyes,
            gastrointestinal: s.gastrointestinal,
            skin: s.skin,
            ears: s.ears,
            noses: s.noses,
            neurological: s.neurological,
            heme: s.heme,
            mouth: s.mouth,
            infectious: s.infectious,
            cardiovascular: s.cardiovascular,
            genitourinary: s.genitourinary,
            psychiatric: s.psychiatric,
            comments: s.comments,
        }
    }
}

impl From<Delete> for PastMedicalHistoryDeleted {
    fn from(s: Delete) -> Self {
        PastMedicalHistoryDeleted {
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

impl From<PastMedicalHistoryEvent>
    for EventWrite<PastMedicalHistoryEvent, PastMedicalHistoryEvent>
{
    fn from(u: PastMedicalHistoryEvent) -> Self {
        EventWrite {
            id: uuid::Uuid::new_v4(),
            correlation_id: None,
            causation_id: None,
            name: String::from("pastmedicalhistory_event"),
            data: u,
            metadata: None,
        }
    }
}
