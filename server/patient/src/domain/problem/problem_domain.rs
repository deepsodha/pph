use crate::domain::problem::problem_commands::{CreateProblem, DeleteProblem, UpdateProblem};
use anyhow::Result;
use chrono::{DateTime, Utc};
use common::{
    domain::{min_max_words::RequiredMax300Words, required_string::RequiredString},
    dto::{status::Status, types::ProblemTypes},
};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use super::problem_events::{ProblemCreated, ProblemUpdated};

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq, TS)]
#[ts(export, export_to = "../../bindings/ProblemState.ts")]
pub struct ProblemState {
    pub id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub org_id: String,
    pub patient_id: String,
    pub status: Status,
    pub issue: String,
    pub icd_10_problem: Option<String>,
    pub issue_type: ProblemTypes,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub comment: String,
    pub is_deleted: bool,
}

impl From<ProblemCreated> for ProblemState {
    fn from(u: ProblemCreated) -> Self {
        ProblemState {
            id: String::from(&u.id),
            created_by: String::from(&u.created_by),
            updated_by: String::from(&u.updated_by),
            created_at: u.created_at,
            last_updated: u.last_updated,
            org_id: String::from(&u.org_id),
            patient_id: String::from(&u.patient_id),
            status: u.status,
            issue: String::from(&u.issue),
            icd_10_problem: u.icd_10_problem,
            issue_type: u.issue_type,
            start_date: Utc::now(),
            end_date: Utc::now(),
            comment: String::from(&u.comment),
            is_deleted: false,
        }
    }
}

impl From<ProblemUpdated> for ProblemState {
    fn from(u: ProblemUpdated) -> Self {
        ProblemState {
            id: String::from(&u.id),
            created_by: String::from(&u.created_by),
            updated_by: String::from(&u.updated_by),
            created_at: u.created_at,
            last_updated: u.last_updated,
            org_id: String::from(&u.org_id),
            patient_id: String::from(&u.patient_id),
            status: u.status,
            issue: String::from(&u.issue),
            icd_10_problem: u.icd_10_problem,
            issue_type: u.issue_type,
            start_date: Utc::now(),
            end_date: Utc::now(),
            comment: String::from(&u.comment),
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
    pub issue: RequiredString,
    pub icd_10_problem: Option<RequiredString>,
    pub issue_type: ProblemTypes,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub comment: RequiredMax300Words,
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
    pub issue: RequiredString,
    pub icd_10_problem: Option<RequiredString>,
    pub issue_type: ProblemTypes,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub comment: RequiredMax300Words,
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
    pub fn parse(a: &CreateProblem) -> Result<Create> {
        Ok(Create {
            id: a.id.to_owned(),
            created_by: RequiredString::parse(&a.created_by)?,
            updated_by: RequiredString::parse(&a.updated_by)?,
            created_at: (a.created_at).to_owned(),
            last_updated: (a.last_updated).to_owned(),
            org_id: RequiredString::parse(&a.org_id)?,
            patient_id: RequiredString::parse(&a.patient_id)?,
            status: (a.status).to_owned(),
            icd_10_problem: match &a.icd_10_problem {
                None => None,
                Some(a) => {
                    let instructions = RequiredString::parse(a)?;
                    Some(instructions)
                }
            },
            issue_type: (a.issue_type).to_owned(),
            issue: RequiredString::parse(&a.issue)?,
            start_date: (a.start_date).to_owned(),
            end_date: (a.end_date).to_owned(),
            comment: RequiredMax300Words::parse(&a.comment)?,
        })
    }
}

impl Update {
    pub fn parse(a: &UpdateProblem) -> Result<Update> {
        // Here all the validation will come
        Ok(Update {
            id: RequiredString::parse(&a.id)?,
            created_by: RequiredString::parse(&a.created_by)?,
            updated_by: RequiredString::parse(&a.updated_by)?,
            created_at: (a.created_at).to_owned(),
            last_updated: (a.last_updated).to_owned(),
            org_id: RequiredString::parse(&a.org_id)?,
            patient_id: RequiredString::parse(&a.patient_id)?,
            status: (a.status).to_owned(),
            icd_10_problem: match &a.icd_10_problem {
                None => None,
                Some(a) => {
                    let instructions = RequiredString::parse(a)?;
                    Some(instructions)
                }
            },
            issue_type: (a.issue_type).to_owned(),
            issue: RequiredString::parse(&a.issue)?,
            start_date: (a.start_date).to_owned(),
            end_date: (a.end_date).to_owned(),
            comment: RequiredMax300Words::parse(&a.comment)?,
        })
    }
}

impl Delete {
    pub fn parse(a: &DeleteProblem) -> Result<Delete> {
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
