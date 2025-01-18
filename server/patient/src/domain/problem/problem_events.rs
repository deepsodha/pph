use crate::domain::problem::problem_domain::{Create, Delete, Update};
use chrono::{DateTime, Utc};
use common::dto::{status::Status, types::ProblemTypes};
use cosmo_store::types::event_write::EventWrite;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ProblemEvent {
    ProblemCreated(ProblemCreated),
    ProblemUpdated(ProblemUpdated),
    ProblemDeleted(ProblemDeleted),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProblemCreated {
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
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProblemUpdated {
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
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProblemDeleted {
    pub id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub org_id: String,
    pub patient_id: String,
}

impl From<ProblemEvent> for EventWrite<ProblemEvent, ProblemEvent> {
    fn from(u: ProblemEvent) -> Self {
        EventWrite {
            id: Uuid::new_v4(),
            correlation_id: None,
            causation_id: None,
            name: String::from("problem_event"),
            data: u,
            metadata: None,
        }
    }
}

impl From<Create> for ProblemCreated {
    fn from(s: Create) -> Self {
        ProblemCreated {
            id: s.id,
            created_by: String::from(s.created_by.as_ref()),
            updated_by: String::from(s.updated_by.as_ref()),
            created_at: s.created_at,
            last_updated: s.last_updated,
            org_id: String::from(s.org_id.as_ref()),
            patient_id: String::from(s.patient_id.as_ref()),
            status: s.status,
            issue: String::from(s.issue.as_ref()),
            icd_10_problem: s.icd_10_problem.map(|x| String::from(x.as_ref())),
            issue_type: s.issue_type,
            start_date: s.start_date,
            end_date: s.end_date,
            comment: String::from(s.comment.as_ref()),
        }
    }
}

impl From<Update> for ProblemUpdated {
    fn from(s: Update) -> Self {
        ProblemUpdated {
            id: String::from(s.id.as_ref()),
            created_by: String::from(s.created_by.as_ref()),
            updated_by: String::from(s.updated_by.as_ref()),
            created_at: s.created_at,
            last_updated: s.last_updated,
            org_id: String::from(s.org_id.as_ref()),
            patient_id: String::from(s.patient_id.as_ref()),
            status: s.status,
            issue: String::from(s.issue.as_ref()),
            icd_10_problem: s.icd_10_problem.map(|x| String::from(x.as_ref())),
            issue_type: s.issue_type,
            start_date: s.start_date,
            end_date: s.end_date,
            comment: String::from(s.comment.as_ref()),
        }
    }
}

impl From<Delete> for ProblemDeleted {
    fn from(s: Delete) -> Self {
        ProblemDeleted {
            id: String::from(s.id.as_ref()),
            created_by: String::from(s.created_by.as_ref()),
            updated_by: String::from(s.updated_by.as_ref()),
            created_at: s.created_at,
            last_updated: s.last_updated,
            org_id: String::from(s.org_id.as_ref()),
            patient_id: String::from(s.patient_id.as_ref()),
        }
    }
}
