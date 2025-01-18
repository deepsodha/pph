use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Deserialize, Default, Serialize, Debug, Clone, PartialEq, TS, Copy, Eq, sqlx::Type)]
#[ts(export, export_to = "../../bindings/Role.ts")]

pub enum Role {
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

impl Role {
    pub fn as_str(&self) -> &'static str {
        match self {
            Role::SuperAdmin => "superadmin",
            Role::Patient => "patient",
            Role::SystemAdmin => "systemadmin",
            Role::Doctor => "Doctor",
            Role::ClinicalAssistant => "ClinicalAssistant",
            Role::OfficeStaff => "OfficeStaff",
            Role::Biller => "Biller",
            Role::BusinessManager => "BusinessManager",
        }
    }

    pub fn as_str_array(&self) -> Vec<&'static str> {
        match self {
            Role::SuperAdmin => vec!["SuperAdmin"],
            Role::Patient => vec!["Patient"],
            Role::SystemAdmin => vec!["SystemAdmin"],
            Role::Doctor => vec!["Doctor"],
            Role::ClinicalAssistant => vec!["ClinicalAssistant"],
            Role::OfficeStaff => vec!["OfficeStaff"],
            Role::Biller => vec!["Biller"],
            Role::BusinessManager => vec!["BusinessManager"],
        }
    }
}
