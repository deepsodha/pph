use serde::{Deserialize, Serialize};
use ts_rs::TS;
use validator::Validate;

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default, PartialEq, Eq, TS)]
#[ts(export, export_to = "../../bindings/LoginMobile.ts")]
pub struct LoginMobile {
    #[validate(length(equal = 10, message = "field can't be empty"))]
    pub mobile_number: String,
    #[validate(length(min = 6, max = 12, message = "field can't be empty"))]
    pub password: String,
}
