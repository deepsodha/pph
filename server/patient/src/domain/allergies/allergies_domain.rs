use crate::dto::allergies::allergy_severities::AllergySeveritiesType;
use anyhow::Result;
use chrono::{DateTime, NaiveDate, Utc};
use common::{
    domain::{min_max_words::RequiredMax300Words, required_string::RequiredString},
    dto::status::Status,
};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use super::{
    allergies_commands::{CreateAllergies, DeleteAllergies, UpdateAllergies},
    allergies_events::{AllergiesCreated, AllergiesUpdated},
};

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq, TS, sqlx::FromRow)]
#[ts(export, export_to = "../../bindings/AllergiesState.ts")]
pub struct AllergiesState {
    pub id: String,
    pub org_id: String,
    pub patient_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub allergen: String,
    pub reaction: String,
    pub allergy_severities: AllergySeveritiesType,
    pub input_date: NaiveDate,
    pub comments: String,
    pub stauts: Status,
    pub is_deleted: bool,
}

impl From<AllergiesCreated> for AllergiesState {
    fn from(u: AllergiesCreated) -> Self {
        AllergiesState {
            id: String::from(&u.id),
            org_id: String::from(&u.org_id),
            patient_id: String::from(&u.patient_id),
            created_by: String::from(&u.created_by),
            updated_by: String::from(&u.updated_by),
            created_at: u.created_at,
            last_updated: u.last_updated,
            allergen: u.allergen,
            reaction: String::from(&u.reaction),
            stauts: u.status,
            allergy_severities: u.allergy_severities,
            input_date: u.input_date,
            comments: String::from(&u.comments),
            is_deleted: false,
        }
    }
}

impl From<AllergiesUpdated> for AllergiesState {
    fn from(u: AllergiesUpdated) -> Self {
        AllergiesState {
            id: String::from(&u.id),
            org_id: String::from(&u.org_id),
            patient_id: String::from(&u.patient_id),
            created_by: String::from(&u.created_by),
            updated_by: String::from(&u.updated_by),
            created_at: u.created_at,
            last_updated: u.last_updated,
            allergen: u.allergen,
            reaction: String::from(&u.reaction),
            allergy_severities: u.allergy_severities,
            input_date: u.input_date,
            stauts: u.status,
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
    pub allergen: RequiredString,
    pub reaction: RequiredString,
    pub allergy_severities: AllergySeveritiesType,
    pub status: Status,
    pub input_date: NaiveDate,
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
    pub allergen: RequiredString,
    pub reaction: RequiredString,
    pub allergy_severities: AllergySeveritiesType,
    pub input_date: NaiveDate,
    pub status: Status,
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
    pub fn parse(a: &CreateAllergies) -> Result<Create> {
        // Here all the validation will come
        Ok(Create {
            id: a.id.to_owned(),
            created_by: RequiredString::parse(&a.created_by)?,
            updated_by: RequiredString::parse(&a.updated_by)?,
            created_at: (a.created_at).to_owned(),
            last_updated: (a.last_updated).to_owned(),
            org_id: RequiredString::parse(&a.org_id)?,
            patient_id: RequiredString::parse(&a.patient_id)?,
            allergen: RequiredString::parse(&a.allergen)?,
            reaction: RequiredString::parse(&a.reaction)?,
            allergy_severities: a.allergy_severities,
            input_date: (a.input_date).to_owned(),
            status: (a.status).to_owned(),
            comments: RequiredMax300Words::parse(&a.comments)?,
        })
    }
}
impl Update {
    pub fn parse(a: &UpdateAllergies) -> Result<Update> {
        // Here all the validation will come
        Ok(Update {
            id: RequiredString::parse(&a.id)?,
            created_by: RequiredString::parse(&a.created_by)?,
            updated_by: RequiredString::parse(&a.updated_by)?,
            created_at: (a.created_at).to_owned(),
            last_updated: (a.last_updated).to_owned(),
            org_id: RequiredString::parse(&a.org_id)?,
            patient_id: RequiredString::parse(&a.patient_id)?,
            allergen: RequiredString::parse(&a.allergen)?,
            reaction: RequiredString::parse(&a.reaction)?,
            allergy_severities: a.allergy_severities,
            input_date: (a.input_date).to_owned(),
            status: (a.status).to_owned(),
            comments: RequiredMax300Words::parse(&a.comments)?,
        })
    }
}
impl Delete {
    pub fn parse(a: &DeleteAllergies) -> Result<Delete> {
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
