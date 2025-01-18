use anyhow::Result;
use chrono::{DateTime, Utc};
use common::domain::{min_max_words::RequiredMax300Words, required_string::RequiredString};
use doctor::dto::doctor_type::DoctorType;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use super::{
    addhistorical_commands::{CreateHistorical, DeleteHistorical, UpdateHistorical},
    addhistorical_events::{HistoricalCreated, HistoricalUpdated},
};

impl From<HistoricalCreated> for HistoricalState {
    fn from(u: HistoricalCreated) -> Self {
        HistoricalState {
            id: String::from(&u.id),
            created_by: String::from(&u.created_by),
            updated_by: String::from(&u.updated_by),
            created_at: u.created_at,
            last_updated: u.last_updated,
            org_id: String::from(&u.org_id),
            patient_id: String::from(&u.patient_id),
            vaccine: String::from(&u.vaccine),
            types: String::from(&u.types),
            date: u.date,
            provider: u.provider,
            number_in_series: String::from(&u.number_in_series),
            source_of_information: String::from(&u.source_of_information),
            comments: String::from(&u.comments),
            is_deleted: false,
        }
    }
}

impl From<HistoricalUpdated> for HistoricalState {
    fn from(u: HistoricalUpdated) -> Self {
        HistoricalState {
            id: String::from(&u.id),
            created_by: String::from(&u.created_by),
            updated_by: String::from(&u.updated_by),
            created_at: u.created_at,
            last_updated: u.last_updated,
            org_id: String::from(&u.org_id),
            patient_id: String::from(&u.patient_id),
            vaccine: String::from(&u.vaccine),
            types: String::from(&u.types),
            provider: u.provider,
            date: u.date,
            number_in_series: String::from(&u.number_in_series),
            source_of_information: String::from(&u.source_of_information),
            comments: String::from(&u.comments),
            is_deleted: false,
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq, TS, sqlx::FromRow)]
#[ts(export, export_to = "../../bindings/HistoricalState.ts")]
pub struct HistoricalState {
    pub id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub org_id: String,
    pub patient_id: String,
    pub vaccine: String,
    pub types: String,
    pub date: DateTime<Utc>,
    pub number_in_series: String,
    pub provider: DoctorType,
    pub source_of_information: String,
    pub comments: String,
    pub is_deleted: bool,
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
    pub types: RequiredString,
    pub date: DateTime<Utc>,
    pub number_in_series: RequiredString,
    pub provider: DoctorType,
    pub source_of_information: RequiredString,
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
    pub types: RequiredString,
    pub date: DateTime<Utc>,
    pub number_in_series: RequiredString,
    pub provider: DoctorType,
    pub source_of_information: RequiredString,
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
    pub fn parse(a: &CreateHistorical) -> Result<Create> {
        Ok(Create {
            id: a.id.to_owned(),
            created_by: RequiredString::parse(&a.created_by)?,
            updated_by: RequiredString::parse(&a.updated_by)?,
            created_at: (a.created_at).to_owned(),
            last_updated: (a.last_updated).to_owned(),
            org_id: RequiredString::parse(&a.org_id)?,
            patient_id: RequiredString::parse(&a.patient_id)?,
            vaccine: RequiredString::parse(&a.vaccine)?,
            types: RequiredString::parse(&a.types)?,
            date: (a.date).to_owned(),
            number_in_series: RequiredString::parse(&a.number_in_series)?,
            provider: (a.provider).to_owned(),
            source_of_information: RequiredString::parse(&a.source_of_information)?,
            comments: RequiredMax300Words::parse(&a.comments)?,
        })
    }
}

impl Update {
    pub fn parse(a: &UpdateHistorical) -> Result<Update> {
        Ok(Update {
            id: RequiredString::parse(&a.id)?,
            created_by: RequiredString::parse(&a.created_by)?,
            updated_by: RequiredString::parse(&a.updated_by)?,
            created_at: (a.created_at).to_owned(),
            last_updated: (a.last_updated).to_owned(),
            org_id: RequiredString::parse(&a.org_id)?,
            patient_id: RequiredString::parse(&a.patient_id)?,
            vaccine: RequiredString::parse(&a.vaccine)?,
            types: RequiredString::parse(&a.types)?,
            date: (a.date).to_owned(),
            number_in_series: RequiredString::parse(&a.number_in_series)?,
            provider: (a.provider).to_owned(),
            source_of_information: RequiredString::parse(&a.source_of_information)?,
            comments: RequiredMax300Words::parse(&a.comments)?,
        })
    }
}
impl Delete {
    pub fn parse(a: &DeleteHistorical) -> Result<Delete> {
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
