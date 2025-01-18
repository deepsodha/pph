use crate::domain::vital::vital_domain::{Create, Delete, Update};
use chrono::{DateTime, Utc};
use cosmo_store::types::event_write::EventWrite;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum VitalsEvent {
    VitalsCreated(VitalsCreated),
    VitalsUpdated(VitalsUpdated),
    VitalsDeleted(VitalsDeleted),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VitalsCreated {
    pub id: String,
    pub org_id: String,
    pub patient_id: String,
    pub doctor_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub date_time: Option<DateTime<Utc>>,
    pub blood_pressure: Option<u32>,
    pub heart_rate: Option<u32>,
    pub comments: Option<String>,
    pub height: Option<u32>,
    pub weight: Option<u32>,
    pub bmi: Option<u32>,
    pub temperature: Option<u32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VitalsUpdated {
    pub id: String,
    pub org_id: String,
    pub patient_id: String,
    pub doctor_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub date_time: Option<DateTime<Utc>>,
    pub blood_pressure: Option<u32>,
    pub heart_rate: Option<u32>,
    pub comments: Option<String>,
    pub height: Option<u32>,
    pub weight: Option<u32>,
    pub bmi: Option<u32>,
    pub temperature: Option<u32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VitalsDeleted {
    pub id: String,
    pub org_id: String,
    pub patient_id: String,
    pub doctor_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}

impl From<VitalsEvent> for EventWrite<VitalsEvent, VitalsEvent> {
    fn from(u: VitalsEvent) -> Self {
        EventWrite {
            id: Uuid::new_v4(),
            correlation_id: None,
            causation_id: None,
            name: String::from("vitals_event"),
            data: u,
            metadata: None,
        }
    }
}

impl From<Create> for VitalsCreated {
    fn from(s: Create) -> Self {
        VitalsCreated {
            id: s.id,
            created_by: String::from(s.created_by.as_ref()),
            updated_by: String::from(s.updated_by.as_ref()),
            created_at: s.created_at,
            last_updated: s.last_updated,
            org_id: String::from(s.org_id.as_ref()),
            patient_id: String::from(s.patient_id.as_ref()),
            doctor_id: String::from(s.doctor_id.as_ref()),
            blood_pressure: s.blood_pressure,
            heart_rate: s.heart_rate,
            date_time: s.date_time,
            comments: s.comments.map(|x| String::from(x.as_ref())),
            height: s.height,
            weight: s.weight,
            bmi: s.bmi,
            temperature: s.temperature,
        }
    }
}

impl From<Update> for VitalsUpdated {
    fn from(s: Update) -> Self {
        VitalsUpdated {
            id: String::from(s.id.as_ref()),
            created_by: String::from(s.created_by.as_ref()),
            updated_by: String::from(s.updated_by.as_ref()),
            created_at: s.created_at,
            last_updated: s.last_updated,
            org_id: String::from(s.org_id.as_ref()),
            patient_id: String::from(s.patient_id.as_ref()),
            doctor_id: String::from(s.doctor_id.as_ref()),
            blood_pressure: s.blood_pressure,
            heart_rate: s.heart_rate,
            date_time: s.date_time,
            comments: s.comments.map(|x| String::from(x.as_ref())),
            height: s.height,
            weight: s.weight,
            bmi: s.bmi,
            temperature: s.temperature,
        }
    }
}

impl From<Delete> for VitalsDeleted {
    fn from(s: Delete) -> Self {
        VitalsDeleted {
            id: String::from(s.id.as_ref()),
            org_id: String::from(s.org_id.as_ref()),
            patient_id: String::from(s.patient_id.as_ref()),
            doctor_id: String::from(s.doctor_id.as_ref()),
            created_by: String::from(s.created_by.as_ref()),
            updated_by: String::from(s.updated_by.as_ref()),
            created_at: s.created_at,
            last_updated: s.last_updated,
        }
    }
}
