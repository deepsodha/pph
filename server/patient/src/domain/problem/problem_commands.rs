use chrono::{DateTime, Utc};
use common::dto::{status::Status, types::ProblemTypes};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ProblemCommand {
    CreateProblem(CreateProblem),
    UpdateProblem(UpdateProblem),
    DeleteProblem(DeleteProblem),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateProblem {
    pub id: String,
    pub org_id: String,
    pub patient_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub status: Status,
    pub issue: String,
    pub icd_10_problem: Option<String>,
    pub issue_type: ProblemTypes,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub comment: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateProblem {
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
pub struct DeleteProblem {
    pub id: String,
    pub org_id: String,
    pub patient_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}
