use common::dto::last_updated_input::LastUpdatedInput;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use validator::Validate;

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default, TS, PartialEq, Eq)]
#[ts(export, export_to = "../../bindings/PatientDelete.ts")]
pub struct PatientDelete {
    #[validate(length(min = 1, message = "field can't be empty"))]
    pub id: String,
    pub last_updated_input: LastUpdatedInput,
}
