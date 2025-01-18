use super::prescription_domain::{Create, Delete, Update};
use chrono::{DateTime, Utc};
use cosmo_store::types::event_write::EventWrite;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PrescriptionCreated {
    pub id: String,
    pub patient_id: String,
    pub doctor_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub org_id: String,
    pub patient_name: String,
    pub presecription_name: String,
    pub instruction: Option<String>,
    pub date: DateTime<Utc>,
    pub drug_name: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PrescriptionUpdated {
    pub id: String,
    pub patient_id: String,
    pub doctor_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub org_id: String,
    pub patient_name: String,
    pub presecription_name: String,
    pub instruction: Option<String>,
    pub date: DateTime<Utc>,
    pub drug_name: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PrescriptionDeleted {
    pub prescription_id: String,
    pub patient_id: String,
    pub doctor_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub org_id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PrescriptionEvent {
    PrescriptionCreated(PrescriptionCreated),
    PrescriptionUpdated(PrescriptionUpdated),
    PrescriptionDeleted(PrescriptionDeleted),
}

impl From<Create> for PrescriptionCreated {
    fn from(s: Create) -> Self {
        PrescriptionCreated {
            id: s.id,
            patient_id: String::from(s.patient_id.as_ref()),
            doctor_id: String::from(s.doctor_id.as_ref()),
            org_id: String::from(s.org_id.as_ref()),
            created_by: String::from(s.created_by.as_ref()),
            updated_by: String::from(s.updated_by.as_ref()),
            created_at: s.created_at,
            last_updated: s.last_updated,
            patient_name: String::from(s.patient_name.as_ref()),
            presecription_name: String::from(s.presecription_name.as_ref()),
            instruction: s
                .instruction
                .map(|instruction| String::from(instruction.as_ref())),
            date: s.date,
            drug_name: s.drug_name,
        }
    }
}

impl From<Update> for PrescriptionUpdated {
    fn from(s: Update) -> Self {
        PrescriptionUpdated {
            id: String::from(s.id.as_ref()),
            patient_id: String::from(s.patient_id.as_ref()),
            doctor_id: String::from(s.doctor_id.as_ref()),
            created_by: String::from(s.created_by.as_ref()),
            updated_by: String::from(s.updated_by.as_ref()),
            created_at: s.created_at,
            last_updated: s.last_updated,
            org_id: String::from(s.org_id.as_ref()),
            patient_name: String::from(s.patient_name.as_ref()),
            presecription_name: String::from(s.presecription_name.as_ref()),
            instruction: s
                .instruction
                .map(|instruction| String::from(instruction.as_ref())),
            date: s.date,
            drug_name: s.drug_name,
        }
    }
}

impl From<Delete> for PrescriptionDeleted {
    fn from(s: Delete) -> Self {
        PrescriptionDeleted {
            patient_id: String::from(s.patient_id.as_ref()),
            doctor_id: String::from(s.doctor_id.as_ref()),
            org_id: String::from(s.org_id.as_ref()),
            prescription_id: String::from(s.id.as_ref()),
            created_by: String::from(s.created_by.as_ref()),
            updated_by: String::from(s.updated_by.as_ref()),
            created_at: s.created_at,
            last_updated: s.last_updated,
        }
    }
}

impl From<PrescriptionEvent> for EventWrite<PrescriptionEvent, PrescriptionEvent> {
    fn from(u: PrescriptionEvent) -> Self {
        EventWrite {
            id: Uuid::new_v4(),
            correlation_id: None,
            causation_id: None,
            name: String::from("prescription_event"),
            data: u,
            metadata: None,
        }
    }
}
