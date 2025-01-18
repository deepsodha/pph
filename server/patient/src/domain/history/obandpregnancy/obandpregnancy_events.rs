use chrono::{DateTime, Utc};
use cosmo_store::types::event_write::EventWrite;
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::obandpregnancy_domain::{Create, Delete, Update};

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default)]
pub struct OBandPregnancyCreated {
    pub id: String,
    pub org_id: String,
    pub patient_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub age_onset_of_menses: u32,
    pub age_at_menopause: u32,
    pub comments_ob: Option<String>,
    pub total_pregnancy: Option<u32>,
    pub full_term: Option<u32>,
    pub pre_term: Option<u32>,
    pub miscarriages: Option<u32>,
    pub living: Option<u32>,
    pub comments_pregnancy: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default)]
pub struct OBandPregnancyUpdated {
    pub id: String,
    pub org_id: String,
    pub patient_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub age_onset_of_menses: u32,
    pub age_at_menopause: u32,
    pub comments_ob: Option<String>,
    pub total_pregnancy: Option<u32>,
    pub full_term: Option<u32>,
    pub pre_term: Option<u32>,
    pub miscarriages: Option<u32>,
    pub living: Option<u32>,
    pub comments_pregnancy: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default)]
pub struct OBandPregnancyDeleted {
    pub id: String,
    pub org_id: String,
    pub patient_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum OBandPregnancyEvent {
    OBandPregnancyCreated(OBandPregnancyCreated),
    OBandPregnancyUpdated(OBandPregnancyUpdated),
    OBandPregnancyDeleted(OBandPregnancyDeleted),
}

impl From<Create> for OBandPregnancyCreated {
    fn from(s: Create) -> Self {
        OBandPregnancyCreated {
            id: s.id,
            org_id: String::from(&s.org_id),
            patient_id: String::from(&s.patient_id),
            created_by: String::from(&s.created_by),
            updated_by: String::from(&s.updated_by),
            created_at: s.created_at,
            last_updated: s.last_updated,
            age_onset_of_menses: s.age_onset_of_menses,
            age_at_menopause: s.age_at_menopause,
            comments_ob: s.comments_ob,
            total_pregnancy: s.total_pregnancy,
            full_term: s.full_term,
            pre_term: s.pre_term,
            miscarriages: s.miscarriages,
            living: s.living,
            comments_pregnancy: s.comments_pregnancy,
        }
    }
}

impl From<Update> for OBandPregnancyUpdated {
    fn from(s: Update) -> Self {
        OBandPregnancyUpdated {
            id: s.id,
            org_id: String::from(&s.org_id),
            patient_id: String::from(&s.patient_id),
            created_by: String::from(&s.created_by),
            updated_by: String::from(&s.updated_by),
            created_at: s.created_at,
            last_updated: s.last_updated,
            age_onset_of_menses: s.age_onset_of_menses,
            age_at_menopause: s.age_at_menopause,
            comments_ob: s.comments_ob,
            total_pregnancy: s.total_pregnancy,
            full_term: s.full_term,
            pre_term: s.pre_term,
            miscarriages: s.miscarriages,
            living: s.living,
            comments_pregnancy: s.comments_pregnancy,
        }
    }
}

impl From<Delete> for OBandPregnancyDeleted {
    fn from(s: Delete) -> Self {
        OBandPregnancyDeleted {
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

impl From<OBandPregnancyEvent> for EventWrite<OBandPregnancyEvent, OBandPregnancyEvent> {
    fn from(u: OBandPregnancyEvent) -> Self {
        EventWrite {
            id: uuid::Uuid::new_v4(),
            correlation_id: None,
            causation_id: None,
            name: String::from("obandpregnancy_event"),
            data: u,
            metadata: None,
        }
    }
}
