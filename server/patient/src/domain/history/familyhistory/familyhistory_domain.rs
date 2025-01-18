use crate::dto::history::health_status::HealthStauts;
use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use super::{
    familyhistory_commands::{CreateFamilyHistory, DeleteFamilyHistory, UpdateFamilyHistory},
    familyhistory_events::{FamilyHistoryCreated, FamilyHistoryUpdated},
};

#[derive(Clone, Debug, Default, Serialize, PartialEq, Eq, Deserialize, TS)]
#[ts(export, export_to = "../../bindings/FamilyHistoryState.ts")]
pub struct FamilyHistoryState {
    pub id: String,
    pub org_id: String,
    pub patient_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub family_member: String,
    pub health_status: HealthStauts,
    pub general: Option<Vec<String>>,
    pub cancer: Option<Vec<String>>,
    pub comments: Option<Vec<String>>,
    pub is_deleted: bool,
}

impl From<FamilyHistoryCreated> for FamilyHistoryState {
    fn from(u: FamilyHistoryCreated) -> Self {
        FamilyHistoryState {
            id: String::from(&u.id),
            org_id: String::from(&u.org_id),
            patient_id: String::from(&u.patient_id),
            created_by: String::from(&u.created_by),
            updated_by: String::from(&u.updated_by),
            created_at: u.created_at,
            last_updated: u.last_updated,
            family_member: String::from(&u.family_member),
            health_status: u.health_status,
            general: u.general,
            cancer: u.cancer,
            comments: u.comments,
            is_deleted: false,
        }
    }
}

impl From<FamilyHistoryUpdated> for FamilyHistoryState {
    fn from(u: FamilyHistoryUpdated) -> Self {
        FamilyHistoryState {
            id: String::from(&u.id),
            org_id: String::from(&u.org_id),
            patient_id: String::from(&u.patient_id),
            created_by: String::from(&u.created_by),
            updated_by: String::from(&u.updated_by),
            created_at: u.created_at,
            last_updated: u.last_updated,
            family_member: String::from(&u.family_member),
            health_status: u.health_status,
            general: u.general,
            cancer: u.cancer,
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
    pub family_member: String,
    pub health_status: HealthStauts,
    pub general: Option<Vec<String>>,
    pub cancer: Option<Vec<String>>,
    pub comments: Option<Vec<String>>,
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
    pub family_member: String,
    pub health_status: HealthStauts,
    pub general: Option<Vec<String>>,
    pub cancer: Option<Vec<String>>,
    pub comments: Option<Vec<String>>,
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
    pub fn parse(a: &CreateFamilyHistory) -> Result<Create> {
        // Here all the validation will come
        Ok(Create {
            id: a.id.to_owned(),
            org_id: String::from(&a.org_id),
            patient_id: String::from(&a.patient_id),
            created_by: String::from(&a.created_by),
            updated_by: String::from(&a.updated_by),
            created_at: (a.created_at).to_owned(),
            last_updated: (a.last_updated).to_owned(),
            family_member: (a.family_member).to_owned(),
            health_status: (a.health_status).to_owned(),
            general: (a.general).to_owned(),
            cancer: (a.cancer).to_owned(),
            comments: (a.comments).to_owned(),
        })
    }
}
impl Update {
    pub fn parse(a: &UpdateFamilyHistory) -> Result<Update> {
        // Here all the validation will come
        Ok(Update {
            id: String::from(&a.id),
            org_id: String::from(&a.org_id),
            patient_id: String::from(&a.patient_id),
            created_by: String::from(&a.created_by),
            updated_by: String::from(&a.updated_by),
            created_at: (a.created_at).to_owned(),
            last_updated: (a.last_updated).to_owned(),
            family_member: (a.family_member).to_owned(),
            health_status: (a.health_status).to_owned(),
            general: (a.general).to_owned(),
            cancer: (a.cancer).to_owned(),
            comments: (a.comments).to_owned(),
        })
    }
}
impl Delete {
    pub fn parse(a: &DeleteFamilyHistory) -> Result<Delete> {
        // Here all the validation will come
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
