use serde::{Deserialize, Serialize};
use ts_rs::TS;
use validator::Validate;

#[derive(
    Deserialize, Serialize, Debug, Clone, Validate, Default, TS, PartialEq, Eq, sqlx::FromRow,
)]
#[ts(export, export_to = "../../bindings/OrgName.ts")]

pub struct OrgName {
    pub org_name: String,
}
