use common::dto::{
    address::AddressInput, contact::ContactInput, gov_info::GovInfoInput,
    last_updated_input::LastUpdatedInput, user::UserInput,
};
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use validator::Validate;

#[derive(Deserialize, Serialize, Debug, Default, Clone, Validate, TS, PartialEq, Eq)]
#[ts(export, export_to = "../../bindings/PatientAdd.ts")]
pub struct PatientAdd {
    pub user: UserInput,
    pub address: AddressInput,
    pub phone: ContactInput,
    pub government_info: GovInfoInput,
    pub last_updated_input: LastUpdatedInput,
}
