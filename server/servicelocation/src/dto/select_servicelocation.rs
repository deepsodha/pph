use serde::{Deserialize, Serialize};
use ts_rs::TS;
use validator::Validate;

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default, TS, PartialEq, Eq)]
#[ts(export, export_to = "../../bindings/SelectServiceLocation.ts")]
pub struct SelectServiceLocation {
    #[validate(length(min = 1, message = "field can't be empty"))]
    pub id: String,
    #[validate(length(min = 1, message = "field can't be empty"))]
    pub name: String,
}
