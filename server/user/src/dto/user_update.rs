use common::dto::{
    address::AddressInput, contact::ContactInput, gov_info::GovInfoInput,
    last_updated_input::LastUpdatedInput, user::UserInput,
};
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use validator::Validate;

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default, TS, PartialEq, Eq)]
#[ts(export, export_to = "../../bindings/UserUpdate.ts")]

pub struct UserUpdate {
    pub id: String,
    pub user: UserInput,
    pub phone: ContactInput,
    pub address: AddressInput,
    pub goverment_info: GovInfoInput,
    pub last_updated_input: LastUpdatedInput,
}
