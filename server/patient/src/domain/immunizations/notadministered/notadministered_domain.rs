use anyhow::Result;
use chrono::{DateTime, Utc};
use common::{
    domain::{min_max_words::RequiredMax300Words, required_string::RequiredString},
    dto::types::Types,
};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use super::{
    notadministered_commands::{
        CreateNotAdministered, DeleteNotAdministered, UpdateNotAdministered,
    },
    notadministered_events::{NotAdministeredCreated, NotAdministeredUpdated},
};

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq, TS)]
#[ts(export, export_to = "../../bindings/NotAdministeredState.ts")]
pub struct NotAdministeredState {
    pub id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub org_id: String,
    pub patient_id: String,
    pub vaccine: String,
    pub types: Types,
    pub recorded: DateTime<Utc>,
    pub reason_for_non_administration: String,
    pub comments: String,
    pub is_deleted: bool,
}

impl From<NotAdministeredCreated> for NotAdministeredState {
    fn from(u: NotAdministeredCreated) -> Self {
        NotAdministeredState {
            id: String::from(&u.id),
            created_by: String::from(&u.created_by),
            updated_by: String::from(&u.updated_by),
            created_at: u.created_at,
            last_updated: u.last_updated,
            org_id: String::from(&u.org_id),
            patient_id: String::from(&u.patient_id),
            vaccine: String::from(&u.vaccine),
            types: u.types,
            recorded: u.recorded,
            reason_for_non_administration: String::from(&u.reason_for_non_administration),
            comments: String::from(&u.comments),
            is_deleted: false,
        }
    }
}

impl From<NotAdministeredUpdated> for NotAdministeredState {
    fn from(u: NotAdministeredUpdated) -> Self {
        NotAdministeredState {
            id: String::from(&u.id),
            created_by: String::from(&u.created_by),
            updated_by: String::from(&u.updated_by),
            created_at: u.created_at,
            last_updated: u.last_updated,
            org_id: String::from(&u.org_id),
            patient_id: String::from(&u.patient_id),
            vaccine: String::from(&u.vaccine),
            types: u.types,
            recorded: u.recorded,
            reason_for_non_administration: String::from(&u.reason_for_non_administration),
            comments: String::from(&u.comments),
            is_deleted: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Create {
    pub id: String,
    pub created_by: RequiredString,
    pub updated_by: RequiredString,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub org_id: RequiredString,
    pub patient_id: RequiredString,
    pub vaccine: RequiredString,
    pub types: Types,
    pub recorded: DateTime<Utc>,
    pub reason_for_non_administration: RequiredString,
    pub comments: RequiredMax300Words,
}

#[derive(Debug, Clone)]
pub struct Update {
    pub id: RequiredString,
    pub created_by: RequiredString,
    pub updated_by: RequiredString,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub org_id: RequiredString,
    pub patient_id: RequiredString,
    pub vaccine: RequiredString,
    pub types: Types,
    pub recorded: DateTime<Utc>,
    pub reason_for_non_administration: RequiredString,
    pub comments: RequiredMax300Words,
}

#[derive(Debug, Clone)]
pub struct Delete {
    pub id: RequiredString,
    pub created_by: RequiredString,
    pub updated_by: RequiredString,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub org_id: RequiredString,
    pub patient_id: RequiredString,
}

impl Create {
    pub fn parse(a: &CreateNotAdministered) -> Result<Create> {
        Ok(Create {
            id: a.id.to_owned(),
            created_by: RequiredString::parse(&a.created_by)?,
            updated_by: RequiredString::parse(&a.updated_by)?,
            created_at: (a.created_at).to_owned(),
            last_updated: (a.last_updated).to_owned(),
            org_id: RequiredString::parse(&a.org_id)?,
            patient_id: RequiredString::parse(&a.patient_id)?,
            vaccine: RequiredString::parse(&a.vaccine)?,
            types: (a.types).to_owned(),
            recorded: (a.recorded).to_owned(),
            reason_for_non_administration: RequiredString::parse(&a.reason_for_non_administration)?,
            comments: RequiredMax300Words::parse(&a.comments)?,
        })
    }
}

impl Update {
    pub fn parse(a: &UpdateNotAdministered) -> Result<Update> {
        // Here all the validation will come
        Ok(Update {
            created_by: RequiredString::parse(&a.created_by)?,
            updated_by: RequiredString::parse(&a.updated_by)?,
            created_at: (a.created_at).to_owned(),
            last_updated: (a.last_updated).to_owned(),
            id: RequiredString::parse(&a.id)?,
            org_id: RequiredString::parse(&a.org_id)?,
            patient_id: RequiredString::parse(&a.patient_id)?,
            vaccine: RequiredString::parse(&a.vaccine)?,
            types: (a.types).to_owned(),
            recorded: (a.recorded).to_owned(),
            reason_for_non_administration: RequiredString::parse(&a.reason_for_non_administration)?,
            comments: RequiredMax300Words::parse(&a.comments)?,
        })
    }
}

impl Delete {
    pub fn parse(a: &DeleteNotAdministered) -> Result<Delete> {
        // Here all the validation will come
        Ok(Delete {
            id: RequiredString::parse(&a.id)?,
            created_by: RequiredString::parse(&a.created_by)?,
            updated_by: RequiredString::parse(&a.updated_by)?,
            created_at: (a.created_at).to_owned(),
            last_updated: (a.last_updated).to_owned(),
            org_id: RequiredString::parse(&a.org_id)?,
            patient_id: RequiredString::parse(&a.patient_id)?,
        })
    }
}
