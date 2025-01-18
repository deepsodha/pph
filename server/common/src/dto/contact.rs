use serde::{Deserialize, Serialize};
use ts_rs::TS;
use validator::Validate;
#[derive(Deserialize, Serialize, Debug, Clone, Validate, Default, TS, PartialEq, Eq)]
#[ts(export, export_to = "../../bindings/Contact.ts")]

pub struct Contact {
    pub number: String,
    pub number_type: PhoneNoTypeForContact,
}

#[derive(Deserialize, Serialize, Debug, Default, Clone, Validate, TS, PartialEq, Eq)]
#[ts(export, export_to = "../../bindings/ContactInput.ts")]

pub struct ContactInput {
    pub number: String,
    pub number_type: PhoneNoTypeForContact,
}

#[derive(Clone, Debug, Serialize, Default, PartialEq, Deserialize, TS, Copy, Eq, sqlx::Type)]
#[ts(export, export_to = "../../bindings/PhoneNoTypeForContact.ts")]

pub enum PhoneNoTypeForContact {
    #[default]
    Mobile,
    Home,
    Office,
}

impl PhoneNoTypeForContact {
    pub fn as_str(&self) -> &'static str {
        match self {
            PhoneNoTypeForContact::Mobile => "Mobile",
            PhoneNoTypeForContact::Home => "Home",
            PhoneNoTypeForContact::Office => "Office",
        }
    }
}
