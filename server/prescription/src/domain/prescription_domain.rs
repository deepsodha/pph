use super::{
    prescription_commands::{CreatePrescription, DeletePrescription, UpdatedPrescription},
    prescription_events::{PrescriptionCreated, PrescriptionUpdated},
};

use anyhow::Result;
use chrono::{DateTime, Utc};
use common::domain::{
    min_max_words::RequiredMax300Words, name::Name, required_string::RequiredString,
};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq, TS)]
#[ts(export, export_to = "../../bindings/PrescriptionState.ts")]
pub struct PrescriptionState {
    pub id: String,
    pub org_id: String,
    pub patient_id: String,
    pub doctor_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub patient_name: String,
    pub presecription_name: String,
    pub instruction: Option<String>,
    pub date: DateTime<Utc>,
    pub drug_name: Vec<String>,
    pub is_deleted: bool,
}

impl From<PrescriptionCreated> for PrescriptionState {
    fn from(u: PrescriptionCreated) -> Self {
        PrescriptionState {
            id: String::from(&u.id),
            org_id: String::from(&u.org_id),
            patient_id: String::from(&u.patient_id),
            doctor_id: String::from(&u.doctor_id),
            created_by: String::from(&u.created_by),
            updated_by: String::from(&u.updated_by),
            created_at: u.created_at,
            last_updated: u.last_updated,
            patient_name: String::from(&u.patient_name),
            presecription_name: String::from(&u.presecription_name),
            instruction: u.instruction,
            date: u.date,
            drug_name: u.drug_name,
            is_deleted: false,
        }
    }
}

impl From<PrescriptionUpdated> for PrescriptionState {
    fn from(u: PrescriptionUpdated) -> Self {
        PrescriptionState {
            id: String::from(&u.id),
            org_id: String::from(&u.org_id),
            patient_id: String::from(&u.patient_id),
            doctor_id: String::from(&u.doctor_id),
            created_by: String::from(&u.created_by),
            updated_by: String::from(&u.updated_by),
            created_at: u.created_at,
            last_updated: u.last_updated,
            patient_name: String::from(&u.patient_name),
            presecription_name: String::from(&u.presecription_name),
            instruction: u.instruction,
            date: u.date,
            drug_name: u.drug_name,
            is_deleted: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Create {
    pub id: String,
    pub patient_id: RequiredString,
    pub doctor_id: RequiredString,
    pub created_by: RequiredString,
    pub updated_by: RequiredString,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub org_id: RequiredString,
    pub patient_name: Name,
    pub presecription_name: Name,
    pub instruction: Option<RequiredMax300Words>,
    pub date: DateTime<Utc>,
    pub drug_name: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Update {
    pub id: RequiredString,
    pub patient_id: RequiredString,
    pub doctor_id: RequiredString,
    pub created_by: RequiredString,
    pub updated_by: RequiredString,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub org_id: RequiredString,
    pub patient_name: Name,
    pub presecription_name: Name,
    pub instruction: Option<RequiredMax300Words>,
    pub date: DateTime<Utc>,
    pub drug_name: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Delete {
    pub id: RequiredString,
    pub patient_id: RequiredString,
    pub doctor_id: RequiredString,
    pub created_by: RequiredString,
    pub updated_by: RequiredString,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub org_id: RequiredString,
}

impl Create {
    pub fn parse(a: &CreatePrescription) -> Result<Create> {
        Ok(Create {
            id: a.id.to_owned(),
            patient_id: RequiredString::parse(&a.patient_id)?,
            doctor_id: RequiredString::parse(&a.doctor_id)?,
            created_by: RequiredString::parse(&a.created_by)?,
            updated_by: RequiredString::parse(&a.updated_by)?,
            created_at: (a.created_at).to_owned(),
            last_updated: (a.last_updated).to_owned(),
            org_id: RequiredString::parse(&a.org_id)?,
            patient_name: Name::parse(&a.patient_name, "Patient")?,
            presecription_name: Name::parse(&a.presecription_name, "Prescription")?,
            instruction: match &a.instruction {
                Some(instruction) => Some(RequiredMax300Words::parse(instruction)?),
                None => None,
            },
            date: a.date,
            drug_name: a.drug_name.clone(),
        })
    }
}

impl Update {
    pub fn parse(a: &UpdatedPrescription) -> Result<Update> {
        Ok(Update {
            id: RequiredString::parse(&a.id)?,
            org_id: RequiredString::parse(&a.org_id)?,
            patient_id: RequiredString::parse(&a.patient_id)?,
            doctor_id: RequiredString::parse(&a.doctor_id)?,
            created_by: RequiredString::parse(&a.created_by)?,
            updated_by: RequiredString::parse(&a.updated_by)?,
            created_at: (a.created_at).to_owned(),
            last_updated: (a.last_updated).to_owned(),
            patient_name: Name::parse(&a.patient_name, "Patient")?,
            presecription_name: Name::parse(&a.presecription_name, "Prescription")?,
            instruction: match &a.instruction {
                Some(instruction) => Some(RequiredMax300Words::parse(instruction)?),
                None => None,
            },
            date: a.date,
            drug_name: a.drug_name.clone(),
        })
    }
}
impl Delete {
    pub fn parse(a: &DeletePrescription) -> Result<Delete> {
        // Here all the validation will come
        Ok(Delete {
            id: RequiredString::parse(&a.id)?,
            org_id: RequiredString::parse(&a.org_id)?,
            patient_id: RequiredString::parse(&a.patient_id)?,
            doctor_id: RequiredString::parse(&a.doctor_id)?,
            created_by: RequiredString::parse(&a.created_by)?,
            updated_by: RequiredString::parse(&a.updated_by)?,
            created_at: (a.created_at).to_owned(),
            last_updated: (a.last_updated).to_owned(),
        })
    }
}
