use common::dto::{
    contact::ContactInput, gov_info::GovInfoInput, last_updated_input::LastUpdatedInput,
    user::UserInput, user_role::SystemAdminRole,
};
use organization::dto::organization_add::OrganizationAdd;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use validator::Validate;

#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default, TS, PartialEq, Eq)]
#[ts(export, export_to = "../../bindings/SystemAdminAdd.ts")]
pub struct SystemAdminAdd {
    pub org_details: OrganizationAdd,
    pub user: UserInput,
    pub roles: Vec<SystemAdminRole>,
    pub phone: ContactInput,
    pub government_info: GovInfoInput,
    #[validate(length(min = 1, message = "field can't be empty"))]
    pub password: String,
    pub confirm_password: String,
    pub last_updated_input: LastUpdatedInput,
}
