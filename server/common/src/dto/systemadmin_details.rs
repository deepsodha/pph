use serde::{Deserialize, Serialize};
use ts_rs::TS;
use validator::Validate;

use super::{
    address::AddressInput, contact::ContactInput, gov_info::GovInfoInput,
    last_updated_input::LastUpdatedInput, user::UserInput, user_role::UserRole,
};

#[derive(
    Deserialize, Serialize, Debug, Clone, Validate, Default, TS, PartialEq, Eq, sqlx::FromRow,
)]
#[ts(export, export_to = "../../bindings/SystemAdminDetails.ts")]

pub struct SystemAdminDetails {
    pub user: UserInput,
    pub address: AddressInput,
    pub roles: Vec<UserRole>,
    pub phone: ContactInput,
    pub government_info: GovInfoInput,
    pub password: String,
    pub confirm_password: String,
    pub last_updated_input: LastUpdatedInput,
}
