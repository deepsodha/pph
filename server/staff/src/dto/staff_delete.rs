use common::dto::last_updated_input::LastUpdatedInput;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Deserialize, Serialize, Debug, Default, Clone, validator::Validate, TS, PartialEq, Eq)]
#[ts(export, export_to = "../../bindings/StaffDelete.ts")]
pub struct StaffDelete {
    #[validate(length(min = 1, message = "field can't be empty"))]
    pub id: String,
    pub last_updated_input: LastUpdatedInput,
}
