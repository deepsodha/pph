use common::dto::address::AddressInput;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use validator::Validate;

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default, PartialEq, TS, Eq)]
#[ts(export, export_to = "../../bindings/OrganizationAdd.ts")]
pub struct OrganizationAdd {
    #[validate(length(min = 1, message = "field can't be empty"))]
    pub name: String,
    #[validate(length(min = 1, message = "field can't be empty"))]
    pub details: String,
    #[validate(length(equal = 1, message = "field can't be empty"))]
    pub phone_number: String,
    #[validate(length(min = 1, message = "field can't be empty"))]
    pub email: String,
    pub address: AddressInput,
}
