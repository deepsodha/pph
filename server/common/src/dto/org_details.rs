use super::address::AddressInput;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use validator::Validate;

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default, TS, PartialEq, Eq)]
#[ts(export, export_to = "../../bindings/OrgDetails.ts")]
pub struct OrgDetails {
    pub id: String,
    pub name: String,
    pub details: String,
    pub phone_number: String,
    pub email: String,
    pub address: AddressInput,
}

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default, TS, PartialEq, Eq)]
#[ts(export, export_to = "../../bindings/OrgDetailsDto.ts")]
pub struct OrgDetailsDto {
    pub name: String,
    pub details: String,
    pub phone_number: String,
    pub email: String,
    pub address: AddressInput,
}
