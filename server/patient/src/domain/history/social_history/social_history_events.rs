use chrono::{DateTime, Utc};
use common::dto::gender::GenderType;
use cosmo_store::types::event_write::EventWrite;
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::social_history_domain::{Create, Delete, Update};

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default)]
pub struct SocialHistoryCreated {
    pub id: String,
    pub org_id: String,
    pub patient_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub birth_gender: GenderType,
    pub tobacco: Option<Vec<String>>,
    pub alcohol: Option<Vec<String>>,
    pub cardiovascular: Option<Vec<String>>,
    pub sexual_activity: Option<Vec<String>>,
    pub drug_abuse: Option<Vec<String>>,
    pub safety: Option<Vec<String>>,
    pub comments: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default)]
pub struct SocialHistoryUpdated {
    pub id: String,
    pub org_id: String,
    pub patient_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub birth_gender: GenderType,
    pub tobacco: Option<Vec<String>>,
    pub alcohol: Option<Vec<String>>,
    pub cardiovascular: Option<Vec<String>>,
    pub sexual_activity: Option<Vec<String>>,
    pub drug_abuse: Option<Vec<String>>,
    pub safety: Option<Vec<String>>,
    pub comments: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default)]
pub struct SocialHistoryDeleted {
    pub id: String,
    pub org_id: String,
    pub patient_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SocialHistoryEvent {
    SocialHistoryCreated(SocialHistoryCreated),
    SocialHistoryUpdated(SocialHistoryUpdated),
    SocialHistoryDeleted(SocialHistoryDeleted),
}

impl From<Create> for SocialHistoryCreated {
    fn from(s: Create) -> Self {
        SocialHistoryCreated {
            id: s.id,
            org_id: String::from(&s.org_id),
            patient_id: String::from(&s.patient_id),
            created_by: String::from(&s.created_by),
            updated_by: String::from(&s.updated_by),
            created_at: s.created_at,
            last_updated: s.last_updated,
            birth_gender: s.birth_gender,
            tobacco: s.tobacco,
            alcohol: s.alcohol,
            cardiovascular: s.cardiovascular,
            sexual_activity: s.sexual_activity,
            drug_abuse: s.drug_abuse,
            safety: s.safety,
            comments: s.comments,
        }
    }
}

impl From<Update> for SocialHistoryUpdated {
    fn from(s: Update) -> Self {
        SocialHistoryUpdated {
            id: s.id,
            org_id: String::from(&s.org_id),
            patient_id: String::from(&s.patient_id),
            created_by: String::from(&s.created_by),
            updated_by: String::from(&s.updated_by),
            created_at: s.created_at,
            last_updated: s.last_updated,
            birth_gender: s.birth_gender,
            tobacco: s.tobacco,
            alcohol: s.alcohol,
            cardiovascular: s.cardiovascular,
            sexual_activity: s.sexual_activity,
            drug_abuse: s.drug_abuse,
            safety: s.safety,
            comments: s.comments,
        }
    }
}

impl From<Delete> for SocialHistoryDeleted {
    fn from(s: Delete) -> Self {
        SocialHistoryDeleted {
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

impl From<SocialHistoryEvent> for EventWrite<SocialHistoryEvent, SocialHistoryEvent> {
    fn from(u: SocialHistoryEvent) -> Self {
        EventWrite {
            id: uuid::Uuid::new_v4(),
            correlation_id: None,
            causation_id: None,
            name: String::from("socialhistory_event"),
            data: u,
            metadata: None,
        }
    }
}
