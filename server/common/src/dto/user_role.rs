use serde::{Deserialize, Serialize};
use sqlx;
use std::fmt;
use ts_rs::TS;

#[derive(Deserialize, Default, Serialize, Debug, Clone, sqlx::Type, TS, PartialEq, Eq)]
#[ts(export, export_to = "../../bindings/DB/UserRole.ts")]
#[sqlx(type_name = "role", rename_all = "lowercase")]

pub enum UserRole {
    #[default]
    SuperAdmin,
    Patient,
    SystemAdmin,
    Doctor,
    ClinicalAssistant,
    OfficeStaff,
    Biller,
    BusinessManager,
}

impl fmt::Display for UserRole {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UserRole::SystemAdmin => write!(f, "systemadmin"),
            UserRole::SuperAdmin => write!(f, "superadmin"),
            UserRole::Patient => write!(f, "patient"),
            UserRole::Doctor => write!(f, "doctor"),
            UserRole::ClinicalAssistant => write!(f, "clinicalassistant"),
            UserRole::OfficeStaff => write!(f, "officestaff"),
            UserRole::Biller => write!(f, "biller"),
            UserRole::BusinessManager => write!(f, "bussinessmanager"),
        }
    }
}

#[derive(Deserialize, Default, Serialize, Debug, Clone, sqlx::Type, TS, PartialEq, Eq)]
#[ts(export, export_to = "../../bindings/DB/SystemAdminRole.ts")]
#[sqlx(type_name = "role", rename_all = "lowercase")]
pub enum SystemAdminRole {
    #[default]
    SystemAdmin,
    Patient,
    Doctor,
    ClinicalAssistant,
    OfficeStaff,
    Biller,
    BusinessManager,
}

impl fmt::Display for SystemAdminRole {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SystemAdminRole::SystemAdmin => write!(f, "SystemAdmin"),
            SystemAdminRole::Patient => write!(f, "Patient"),
            SystemAdminRole::Doctor => write!(f, "Doctor"),
            SystemAdminRole::ClinicalAssistant => write!(f, "ClinicalAssistant"),
            SystemAdminRole::OfficeStaff => write!(f, "OfficeStaff"),
            SystemAdminRole::Biller => write!(f, "Biller"),
            SystemAdminRole::BusinessManager => write!(f, "BusinessManager"),
        }
    }
}

#[derive(sqlx::FromRow, TS, Debug, Clone)]
#[ts(export, export_to = "../../bindings/DB/RolesRow.ts")]
pub struct RolesRow {
    pub role: UserRole,
}

#[derive(sqlx::FromRow, TS, Debug, Clone)]
#[ts(export, export_to = "../../bindings/DB/RolesRow.ts")]
pub struct RolesRow1 {
    pub first_name: String,
}
