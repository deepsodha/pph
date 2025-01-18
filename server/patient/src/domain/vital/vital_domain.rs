use crate::domain::vital::vital_commands::{CreateVitals, DeleteVitals, UpdateVitals};
use anyhow::Result;
use chrono::{DateTime, Utc};
use common::domain::{min_max_words::RequiredMax300Words, required_string::RequiredString};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use super::vital_events::{VitalsCreated, VitalsUpdated};

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq, TS, sqlx::FromRow)]
#[ts(export, export_to = "../../bindings/VitalsState.ts")]
pub struct VitalsState {
    pub id: String,
    pub org_id: String,
    pub patient_id: String,
    pub doctor_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub date_time: Option<DateTime<Utc>>,
    pub blood_pressure: Option<u32>,
    pub heart_rate: Option<u32>,
    pub comments: Option<String>,
    pub height: Option<u32>,
    pub weight: Option<u32>,
    pub bmi: Option<u32>,
    pub temperature: Option<u32>,
    pub is_deleted: bool,
}

impl From<VitalsCreated> for VitalsState {
    fn from(u: VitalsCreated) -> Self {
        VitalsState {
            id: String::from(&u.id),
            doctor_id: String::from(&u.doctor_id),
            created_by: String::from(&u.created_by),
            updated_by: String::from(&u.updated_by),
            created_at: u.created_at,
            last_updated: u.last_updated,
            org_id: String::from(&u.org_id),
            patient_id: String::from(&u.patient_id),
            blood_pressure: u.blood_pressure,
            heart_rate: u.heart_rate,
            date_time: u.date_time,
            comments: u.comments,
            height: u.height,
            weight: u.weight,
            bmi: u.bmi,
            temperature: u.temperature,
            is_deleted: false,
        }
    }
}

impl From<VitalsUpdated> for VitalsState {
    fn from(u: VitalsUpdated) -> Self {
        VitalsState {
            id: String::from(&u.id),
            doctor_id: String::from(&u.doctor_id),
            created_by: String::from(&u.created_by),
            updated_by: String::from(&u.updated_by),
            created_at: u.created_at,
            last_updated: u.last_updated,
            org_id: String::from(&u.org_id),
            patient_id: String::from(&u.patient_id),
            blood_pressure: u.blood_pressure,
            heart_rate: u.heart_rate,
            date_time: u.date_time,
            comments: u.comments,
            height: u.height,
            weight: u.weight,
            bmi: u.bmi,
            temperature: u.temperature,
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
    pub doctor_id: RequiredString,
    pub date_time: Option<DateTime<Utc>>,
    pub blood_pressure: Option<u32>,
    pub heart_rate: Option<u32>,
    pub comments: Option<RequiredMax300Words>,
    pub height: Option<u32>,
    pub weight: Option<u32>,
    pub bmi: Option<u32>,
    pub temperature: Option<u32>,
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
    pub doctor_id: RequiredString,
    pub date_time: Option<DateTime<Utc>>,
    pub blood_pressure: Option<u32>,
    pub heart_rate: Option<u32>,
    pub comments: Option<RequiredMax300Words>,
    pub height: Option<u32>,
    pub weight: Option<u32>,
    pub bmi: Option<u32>,
    pub temperature: Option<u32>,
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
    pub doctor_id: RequiredString,
}

impl Create {
    pub fn parse(a: &CreateVitals) -> Result<Create> {
        Ok(Create {
            id: a.id.to_owned(),
            created_by: RequiredString::parse(&a.created_by)?,
            updated_by: RequiredString::parse(&a.updated_by)?,
            created_at: (a.created_at).to_owned(),
            last_updated: (a.last_updated).to_owned(),
            org_id: RequiredString::parse(&a.org_id)?,
            doctor_id: RequiredString::parse(&a.doctor_id)?,
            patient_id: RequiredString::parse(&a.patient_id)?,
            date_time: a.date_time.to_owned(),
            blood_pressure: a.blood_pressure,
            heart_rate: a.heart_rate,
            comments: match &a.comments {
                None => None,
                Some(x) => {
                    let blood_pressure = RequiredMax300Words::parse(x)?;
                    Some(blood_pressure)
                }
            },
            height: a.height,
            weight: a.weight,
            bmi: a.bmi,
            temperature: a.temperature,
        })
    }
}

impl Update {
    pub fn parse(a: &UpdateVitals) -> Result<Update> {
        // Here all the validation will come
        Ok(Update {
            id: RequiredString::parse(&a.id)?,
            created_by: RequiredString::parse(&a.created_by)?,
            updated_by: RequiredString::parse(&a.updated_by)?,
            created_at: (a.created_at).to_owned(),
            last_updated: (a.last_updated).to_owned(),
            org_id: RequiredString::parse(&a.org_id)?,
            patient_id: RequiredString::parse(&a.patient_id)?,
            doctor_id: RequiredString::parse(&a.doctor_id)?,
            date_time: a.date_time.to_owned(),
            blood_pressure: a.blood_pressure,
            heart_rate: a.heart_rate,
            comments: match &a.comments {
                None => None,
                Some(x) => {
                    let blood_pressure = RequiredMax300Words::parse(x)?;
                    Some(blood_pressure)
                }
            },
            height: a.height,
            weight: a.weight,
            bmi: a.bmi,
            temperature: a.temperature,
        })
    }
}

impl Delete {
    pub fn parse(a: &DeleteVitals) -> Result<Delete> {
        // Here all the validation will come
        Ok(Delete {
            id: RequiredString::parse(&a.id)?,
            created_by: RequiredString::parse(&a.created_by)?,
            updated_by: RequiredString::parse(&a.updated_by)?,
            created_at: (a.created_at).to_owned(),
            last_updated: (a.last_updated).to_owned(),
            org_id: RequiredString::parse(&a.org_id)?,
            patient_id: RequiredString::parse(&a.patient_id)?,
            doctor_id: RequiredString::parse(&a.doctor_id)?,
        })
    }
}
