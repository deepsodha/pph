use serde::{Deserialize, Serialize};
use ts_rs::TS;
use validator::Validate;

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default, TS, PartialEq, Eq)]
#[ts(export, export_to = "../../bindings/OrganizationsByID.ts")]
pub struct OrganizationsByID {
    #[validate(length(min = 1, message = "field can't be empty"))]
    pub org_id: String,
}
