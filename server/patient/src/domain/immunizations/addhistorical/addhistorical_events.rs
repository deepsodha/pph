use chrono::{DateTime, Utc};
use cosmo_store::types::event_write::EventWrite;
use doctor::dto::doctor_type::DoctorType;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::addhistorical_domain::{Create, Delete, Update};
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum HistoricalEvent {
    HistoricalCreated(HistoricalCreated),
    HistoricalUpdated(HistoricalUpdated),
    HistoricalDeleted(HistoricalDeleted),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HistoricalCreated {
    pub id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub org_id: String,
    pub patient_id: String,
    pub vaccine: String,
    pub types: String,
    pub date: DateTime<Utc>,
    pub number_in_series: String,
    pub provider: DoctorType,
    pub source_of_information: String,
    pub comments: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HistoricalUpdated {
    pub id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub org_id: String,
    pub patient_id: String,
    pub vaccine: String,
    pub types: String,
    pub date: DateTime<Utc>,
    pub number_in_series: String,
    pub provider: DoctorType,
    pub source_of_information: String,
    pub comments: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HistoricalDeleted {
    pub id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub org_id: String,
    pub patient_id: String,
}

impl From<HistoricalEvent> for EventWrite<HistoricalEvent, HistoricalEvent> {
    fn from(u: HistoricalEvent) -> Self {
        EventWrite {
            id: Uuid::new_v4(),
            correlation_id: None,
            causation_id: None,
            name: String::from("historical_event"),
            data: u,
            metadata: None,
        }
    }
}

impl From<Create> for HistoricalCreated {
    fn from(s: Create) -> Self {
        HistoricalCreated {
            id: s.id,
            created_by: String::from(s.created_by.as_ref()),
            updated_by: String::from(s.updated_by.as_ref()),
            created_at: s.created_at,
            last_updated: s.last_updated,
            org_id: String::from(s.org_id.as_ref()),
            patient_id: String::from(s.patient_id.as_ref()),
            vaccine: String::from(s.vaccine.as_ref()),
            types: String::from(s.types.as_ref()),
            provider: s.provider,
            date: s.date,
            source_of_information: String::from(s.source_of_information.as_ref()),
            number_in_series: String::from(s.number_in_series.as_ref()),
            comments: String::from(s.comments.as_ref()),
        }
    }
}

impl From<Update> for HistoricalUpdated {
    fn from(s: Update) -> Self {
        HistoricalUpdated {
            id: String::from(s.id.as_ref()),
            created_by: String::from(s.created_by.as_ref()),
            updated_by: String::from(s.updated_by.as_ref()),
            created_at: s.created_at,
            last_updated: s.last_updated,
            org_id: String::from(s.org_id.as_ref()),
            patient_id: String::from(s.patient_id.as_ref()),
            vaccine: String::from(s.vaccine.as_ref()),
            types: String::from(s.types.as_ref()),
            provider: s.provider,
            date: s.date,
            source_of_information: String::from(s.source_of_information.as_ref()),
            number_in_series: String::from(s.number_in_series.as_ref()),
            comments: String::from(s.comments.as_ref()),
        }
    }
}

impl From<Delete> for HistoricalDeleted {
    fn from(s: Delete) -> Self {
        HistoricalDeleted {
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
