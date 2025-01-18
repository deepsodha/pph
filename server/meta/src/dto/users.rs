use std::fmt;

use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, TS, sqlx::FromRow)]
#[ts(export, export_to = "../../bindings/Users.ts")]
pub struct Users {
    pub mobile_number: String,
    pub password: String,
    pub role: Vec<RoleType>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, TS, sqlx::FromRow)]
#[ts(export, export_to = "../../bindings/UpdateUsers.ts")]
pub struct UpdateUsers {
    pub mobile_number: String,
    pub role: Vec<RoleType>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, TS, sqlx::FromRow)]
#[ts(export, export_to = "../../bindings/GetUsers.ts")]
pub struct GetUsers {
    pub id: String,
    pub mobile_number: String,
    pub role: String,
    pub is_deleted: bool,
}

#[derive(Clone, Debug, Serialize, Default, PartialEq, Deserialize, TS, Copy, Eq, sqlx::Type)]
#[ts(export, export_to = "../../bindings/RoleType.ts")]
pub enum RoleType {
    #[default]
    SuperAdmin,
    SystemAdmin,
}

impl RoleType {
    pub fn as_str(&self) -> &str {
        match self {
            RoleType::SuperAdmin => "superadmin",
            RoleType::SystemAdmin => "systemadmin",
        }
    }
}

impl fmt::Display for RoleType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RoleType::SystemAdmin => write!(f, "systemadmin"),
            RoleType::SuperAdmin => write!(f, "superadmin"),
        }
    }
}

#[derive(sqlx::FromRow, TS, Debug, Clone)]
#[ts(export, export_to = "../../bindings/Roles.ts")]
pub struct Roles {
    pub role: RoleType,
}
