use common::dto::{last_updated_input::LastUpdatedInput, status::Status};
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use validator::Validate;

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default, TS, PartialEq, Eq)]
#[ts(export, export_to = "../../bindings/MedicationsAdd.ts")]
pub struct MedicationsAdd {
    pub patient_id: String,
    pub status: Status,
    #[validate(length(min = 1, message = "field can't be empty"))]
    pub drug: String,
    #[validate(length(min = 1, message = "field can't be empty"))]
    pub instruction: Option<String>,
    #[validate(length(min = 1, message = "field can't be empty"))]
    pub comments: String,
    pub last_updated_input: LastUpdatedInput,
}
