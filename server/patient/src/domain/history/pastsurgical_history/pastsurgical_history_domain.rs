use super::{
    pastsurgical_history_commands::{
        CreatePastSurgicalHistory, DeletePastSurgicalHistory, UpdatePastSurgicalHistory,
    },
    pastsurgical_history_events::{PastSurgicalHistoryCreated, PastSurgicalHistoryUpdated},
};
use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq, TS)]
#[ts(export, export_to = "../../bindings/PastSurgicalHistoryState.ts")]
pub struct PastSurgicalHistoryState {
    pub id: String,
    pub org_id: String,
    pub patient_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub common_surgeries: Option<Vec<String>>,
    pub comments: Option<String>,
    pub is_deleted: bool,
}
impl From<PastSurgicalHistoryCreated> for PastSurgicalHistoryState {
    fn from(u: PastSurgicalHistoryCreated) -> Self {
        PastSurgicalHistoryState {
            id: String::from(&u.id),
            org_id: String::from(&u.org_id),
            patient_id: String::from(&u.patient_id),
            created_by: String::from(&u.created_by),
            updated_by: String::from(&u.updated_by),
            created_at: u.created_at,
            last_updated: u.last_updated,
            common_surgeries: u.common_surgeries,
            comments: u.comments,
            is_deleted: false,
        }
    }
}

impl From<PastSurgicalHistoryUpdated> for PastSurgicalHistoryState {
    fn from(u: PastSurgicalHistoryUpdated) -> Self {
        PastSurgicalHistoryState {
            id: String::from(&u.id),
            org_id: String::from(&u.org_id),
            patient_id: String::from(&u.patient_id),
            created_by: String::from(&u.created_by),
            updated_by: String::from(&u.updated_by),
            created_at: u.created_at,
            last_updated: u.last_updated,
            common_surgeries: u.common_surgeries,
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
    pub common_surgeries: Option<Vec<String>>,
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
    pub common_surgeries: Option<Vec<String>>,
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
    pub fn parse(a: &CreatePastSurgicalHistory) -> Result<Create> {
        Ok(Create {
            id: a.id.to_owned(),
            org_id: String::from(&a.org_id),
            patient_id: String::from(&a.patient_id),
            created_by: String::from(&a.created_by),
            updated_by: String::from(&a.updated_by),
            created_at: (a.created_at).to_owned(),
            last_updated: (a.last_updated).to_owned(),
            common_surgeries: (a.common_surgeries).to_owned(),
            comments: (a.comments).to_owned(),
        })
    }
}
impl Update {
    pub fn parse(a: &UpdatePastSurgicalHistory) -> Result<Update> {
        Ok(Update {
            id: String::from(&a.id),
            org_id: String::from(&a.org_id),
            patient_id: String::from(&a.patient_id),
            created_by: String::from(&a.created_by),
            updated_by: String::from(&a.updated_by),
            created_at: (a.created_at).to_owned(),
            last_updated: (a.last_updated).to_owned(),
            common_surgeries: (a.common_surgeries).to_owned(),
            comments: (a.comments).to_owned(),
        })
    }
}
impl Delete {
    pub fn parse(a: &DeletePastSurgicalHistory) -> Result<Delete> {
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
