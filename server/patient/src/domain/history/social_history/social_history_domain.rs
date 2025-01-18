use anyhow::Result;
use chrono::{DateTime, Utc};
use common::dto::gender::GenderType;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use super::{
    social_history_commands::{CreateSocialHistory, DeleteSocialHistory, UpdateSocialHistory},
    social_history_events::{SocialHistoryCreated, SocialHistoryUpdated},
};

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq, TS)]
#[ts(export, export_to = "../../bindings/SocialHistoryState.ts")]
pub struct SocialHistoryState {
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
    pub is_deleted: bool,
}

impl From<SocialHistoryCreated> for SocialHistoryState {
    fn from(u: SocialHistoryCreated) -> Self {
        SocialHistoryState {
            id: String::from(&u.id),
            org_id: String::from(&u.org_id),
            patient_id: String::from(&u.patient_id),
            created_by: String::from(&u.created_by),
            updated_by: String::from(&u.updated_by),
            created_at: u.created_at,
            last_updated: u.last_updated,
            birth_gender: u.birth_gender,
            tobacco: u.tobacco,
            alcohol: u.alcohol,
            cardiovascular: u.cardiovascular,
            sexual_activity: u.sexual_activity,
            drug_abuse: u.drug_abuse,
            safety: u.safety,
            comments: u.comments,
            is_deleted: false,
        }
    }
}

impl From<SocialHistoryUpdated> for SocialHistoryState {
    fn from(u: SocialHistoryUpdated) -> Self {
        SocialHistoryState {
            id: String::from(&u.id),
            org_id: String::from(&u.org_id),
            patient_id: String::from(&u.patient_id),
            created_by: String::from(&u.created_by),
            updated_by: String::from(&u.updated_by),
            created_at: u.created_at,
            last_updated: u.last_updated,
            birth_gender: u.birth_gender,
            tobacco: u.tobacco,
            alcohol: u.alcohol,
            cardiovascular: u.cardiovascular,
            sexual_activity: u.sexual_activity,
            drug_abuse: u.drug_abuse,
            safety: u.safety,
            comments: u.comments,
            is_deleted: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Create {
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

#[derive(Debug, Clone)]
pub struct Update {
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

#[derive(Debug, Clone)]
pub struct Delete {
    pub id: String,
    pub org_id: String,
    pub patient_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}

impl Create {
    pub fn parse(a: &CreateSocialHistory) -> Result<Create> {
        Ok(Create {
            id: a.id.to_owned(),
            org_id: String::from(&a.org_id),
            patient_id: String::from(&a.patient_id),
            created_by: String::from(&a.created_by),
            updated_by: String::from(&a.updated_by),
            created_at: (a.created_at).to_owned(),
            last_updated: (a.last_updated).to_owned(),
            birth_gender: (a.birth_gender).to_owned(),
            tobacco: (a.tobacco).to_owned(),
            alcohol: (a.alcohol).to_owned(),
            cardiovascular: (a.cardiovascular).to_owned(),
            sexual_activity: (a.sexual_activity).to_owned(),
            drug_abuse: (a.drug_abuse).to_owned(),
            safety: (a.safety).to_owned(),
            comments: (a.comments).to_owned(),
        })
    }
}
impl Update {
    pub fn parse(a: &UpdateSocialHistory) -> Result<Update> {
        Ok(Update {
            id: String::from(&a.id),
            org_id: String::from(&a.org_id),
            patient_id: String::from(&a.patient_id),
            created_by: String::from(&a.created_by),
            updated_by: String::from(&a.updated_by),
            created_at: (a.created_at).to_owned(),
            last_updated: (a.last_updated).to_owned(),
            birth_gender: (a.birth_gender).to_owned(),
            tobacco: (a.tobacco).to_owned(),
            alcohol: (a.alcohol).to_owned(),
            cardiovascular: (a.cardiovascular).to_owned(),
            sexual_activity: (a.sexual_activity).to_owned(),
            drug_abuse: (a.drug_abuse).to_owned(),
            safety: (a.safety).to_owned(),
            comments: (a.comments).to_owned(),
        })
    }
}
impl Delete {
    pub fn parse(a: &DeleteSocialHistory) -> Result<Delete> {
        Ok(Delete {
            id: String::from(&a.id),
            org_id: String::from(&a.org_id),
            patient_id: String::from(&a.patient_id),
            created_by: String::from(&a.created_by),
            updated_by: String::from(&a.updated_by),
            created_at: (a.created_at).to_owned(),
            last_updated: (a.last_updated).to_owned(),
        })
    }
}
