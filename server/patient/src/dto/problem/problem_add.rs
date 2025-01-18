use chrono::{DateTime, Utc};
use common::dto::{last_updated_input::LastUpdatedInput, status::Status, types::ProblemTypes};
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use validator::Validate;

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default, TS, PartialEq, Eq)]
#[ts(export, export_to = "../../bindings/ProblemsAdd.ts")]
pub struct ProblemsAdd {
    pub patient_id: String,
    pub status: Status,
    #[validate(length(min = 1, message = "field can't be empty"))]
    pub issue: String,
    #[validate(length(min = 1, message = "field can't be empty"))]
    pub icd_10_problem: Option<String>,
    pub issue_type: ProblemTypes,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    #[validate(length(min = 1, message = "field can't be empty"))]
    pub comment: String,
    pub last_updated_input: LastUpdatedInput,
}
