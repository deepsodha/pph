use common::dto::last_updated_input::LastUpdatedInput;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use validator::Validate;

#[derive(Deserialize, Serialize, Debug, Clone, Default, PartialEq, TS, Validate, Eq)]
#[ts(export, export_to = "../../bindings/ProblemsDelete.ts")]
pub struct ProblemsDelete {
    #[validate(length(min = 1, message = "field can't be empty"))]
    pub id: String,
    #[validate(length(min = 1, message = "field can't be empty"))]
    pub patient_id: String,
    pub last_updated_input: LastUpdatedInput,
}
