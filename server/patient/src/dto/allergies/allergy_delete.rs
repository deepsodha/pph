use common::dto::last_updated_input::LastUpdatedInput;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use validator::Validate;

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default, TS, PartialEq, Eq)]
#[ts(export, export_to = "../../bindings/AllergyDelete.ts")]
pub struct AllergyDelete {
    #[validate(length(min = 1, message = "field can't be empty"))]
    pub id: String,
    #[validate(length(min = 1, message = "field can't be empty"))]
    pub patient_id: String,
    pub last_updated_input: LastUpdatedInput,
}
