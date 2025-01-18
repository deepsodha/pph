use serde::{Deserialize, Serialize};
use ts_rs::TS;
use validator::Validate;

use super::{org_details::OrgDetails, systemadmin_details::SystemAdminDetails};

#[derive(
    Deserialize, Serialize, Debug, Clone, Validate, Default, TS, PartialEq, Eq, sqlx::FromRow,
)]
#[ts(export, export_to = "../../bindings/OrgUser.ts")]

pub struct OrgUser {
    pub org_details: OrgDetails,
    pub systemadmin_details: SystemAdminDetails,
}
