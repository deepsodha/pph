use anyhow::Result;
use chrono::{DateTime, Utc};
use common::{
    domain::{min_max_words::RequiredMax300Words, required_string::RequiredString},
    dto::status::Status,
};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use super::{
    medication_commands::{CreateMedication, DeleteMedication, UpdateMedication},
    medication_events::{MedicationCreated, MedicationUpdated},
};

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq, TS, sqlx::FromRow)]
#[ts(export, export_to = "../../bindings/MedicationsState.ts")]
pub struct MedicationsState {
    pub id: String,
    pub org_id: String,
    pub patient_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub status: Status,
    pub drug: String,
    pub instruction: Option<String>,
    pub comments: String,
    pub is_deleted: bool,
}

impl From<MedicationCreated> for MedicationsState {
    fn from(u: MedicationCreated) -> Self {
        MedicationsState {
            id: String::from(&u.id),
            org_id: String::from(&u.org_id),
            patient_id: String::from(&u.patient_id),
            status: u.status,
            drug: String::from(&u.drug),
            instruction: u.instruction,
            created_by: String::from(&u.created_by),
            updated_by: String::from(&u.updated_by),
            created_at: u.created_at,
            last_updated: u.last_updated,
            comments: String::from(&u.comments),
            is_deleted: false,
        }
    }
}

impl From<MedicationUpdated> for MedicationsState {
    fn from(u: MedicationUpdated) -> Self {
        MedicationsState {
            id: String::from(&u.id),
            org_id: String::from(&u.org_id),
            patient_id: String::from(&u.patient_id),
            status: u.status,
            drug: String::from(&u.drug),
            instruction: u.instruction,
            created_by: String::from(&u.created_by),
            updated_by: String::from(&u.updated_by),
            created_at: u.created_at,
            last_updated: u.last_updated,
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
    pub status: Status,
    pub drug: RequiredString,
    pub instruction: Option<RequiredMax300Words>,
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
    pub status: Status,
    pub drug: RequiredString,
    pub instruction: Option<RequiredMax300Words>,
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
    pub fn parse(a: &CreateMedication) -> Result<Create> {
        Ok(Create {
            id: a.id.to_owned(),
            created_by: RequiredString::parse(&a.created_by)?,
            updated_by: RequiredString::parse(&a.updated_by)?,
            created_at: (a.created_at).to_owned(),
            last_updated: (a.last_updated).to_owned(),
            org_id: RequiredString::parse(&a.org_id)?,
            patient_id: RequiredString::parse(&a.patient_id)?,
            status: a.status.clone(),
            drug: RequiredString::parse(&a.drug)?,
            instruction: match &a.instruction {
                None => None,
                Some(a) => {
                    let instructions = RequiredMax300Words::parse(a)?;
                    Some(instructions)
                }
            },
            comments: RequiredMax300Words::parse(&a.comments)?,
        })
    }
}

impl Update {
    pub fn parse(a: &UpdateMedication) -> Result<Update> {
        Ok(Update {
            id: RequiredString::parse(&a.id)?,
            created_by: RequiredString::parse(&a.created_by)?,
            updated_by: RequiredString::parse(&a.updated_by)?,
            created_at: (a.created_at).to_owned(),
            last_updated: (a.last_updated).to_owned(),
            org_id: RequiredString::parse(&a.org_id)?,
            patient_id: RequiredString::parse(&a.patient_id)?,
            status: a.status.clone(),
            drug: RequiredString::parse(&a.drug)?,
            instruction: match &a.instruction {
                None => None,
                Some(a) => {
                    let instructions = RequiredMax300Words::parse(a)?;
                    Some(instructions)
                }
            },
            comments: RequiredMax300Words::parse(&a.comments)?,
        })
    }
}

impl Delete {
    pub fn parse(a: &DeleteMedication) -> Result<Delete> {
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
