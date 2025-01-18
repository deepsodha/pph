use chrono::{DateTime, NaiveDate, Utc};
use common::dto::status::Status;
use cosmo_store::types::event_write::EventWrite;
use serde::{Deserialize, Serialize};

use crate::dto::allergies::allergy_severities::AllergySeveritiesType;

use super::allergies_domain::{Create, Delete, Update};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AllergiesEvent {
    AllergiesCreated(AllergiesCreated),
    AllergiesUpdated(AllergiesUpdated),
    AllergiesDeleted(AllergiesDeleted),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AllergiesCreated {
    pub id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub org_id: String,
    pub patient_id: String,
    pub allergen: String,
    pub reaction: String,
    pub allergy_severities: AllergySeveritiesType,
    pub input_date: NaiveDate,
    pub comments: String,
    pub status: Status,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AllergiesUpdated {
    pub org_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub patient_id: String,
    pub id: String,
    pub allergen: String,
    pub reaction: String,
    pub status: Status,
    pub allergy_severities: AllergySeveritiesType,
    pub input_date: NaiveDate,
    pub comments: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AllergiesDeleted {
    pub org_id: String,
    pub patient_id: String,
    pub id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}

impl From<Create> for AllergiesCreated {
    fn from(s: Create) -> Self {
        AllergiesCreated {
            id: s.id,
            org_id: String::from(s.org_id.as_ref()),
            patient_id: String::from(s.patient_id.as_ref()),
            allergen: String::from(s.allergen.as_ref()),
            reaction: String::from(s.reaction.as_ref()),
            allergy_severities: s.allergy_severities,
            input_date: s.input_date,
            created_by: String::from(s.created_by.as_ref()),
            updated_by: String::from(s.updated_by.as_ref()),
            created_at: s.created_at,
            last_updated: s.last_updated,
            status: s.status,
            comments: String::from(s.comments.as_ref()),
        }
    }
}

impl From<Update> for AllergiesUpdated {
    fn from(s: Update) -> Self {
        AllergiesUpdated {
            id: String::from(s.id.as_ref()),
            org_id: String::from(s.org_id.as_ref()),
            patient_id: String::from(s.patient_id.as_ref()),
            allergen: String::from(s.allergen.as_ref()),
            reaction: String::from(s.reaction.as_ref()),
            allergy_severities: s.allergy_severities,
            input_date: s.input_date,
            created_by: String::from(s.created_by.as_ref()),
            updated_by: String::from(s.updated_by.as_ref()),
            created_at: s.created_at,
            last_updated: s.last_updated,
            status: s.status,
            comments: String::from(s.comments.as_ref()),
        }
    }
}

impl From<Delete> for AllergiesDeleted {
    fn from(s: Delete) -> Self {
        AllergiesDeleted {
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

impl From<AllergiesEvent> for EventWrite<AllergiesEvent, AllergiesEvent> {
    fn from(u: AllergiesEvent) -> Self {
        EventWrite {
            id: uuid::Uuid::new_v4(),
            correlation_id: None,
            causation_id: None,
            name: String::from("allergies_event"),
            data: u,
            metadata: None,
        }
    }
}
