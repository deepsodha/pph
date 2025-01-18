use super::{
    patient_commands::{
        UpdatePatientAddress, UpdatePatientContactDetails, UpdatePatientGovDetails,
        UpdatePatientUserDetails,
    },
    patient_events::PatientCreated,
};
use crate::domain::patient_commands::{CreatePatient, DeletePatient, UpdatePatient};
use anyhow::Result;
use chrono::{DateTime, Utc};
use common::domain::address::Address;
use common::domain::contact::Contact;
use common::domain::goverment::GovInfo;
use common::domain::required_string::RequiredString;
use common::domain::user::User;
use common::dto::address::AddressInput;
use common::dto::contact::ContactInput;
use common::dto::gov_info::GovInfoInput;
use common::dto::user::UserInput;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Clone, Debug, Default, sqlx::FromRow, Serialize, Deserialize, PartialEq, Eq, TS)]
#[ts(export, export_to = "../../bindings/PatientState.ts")]
pub struct PatientState {
    pub id: String,
    pub org_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub user: UserInput,
    pub address: AddressInput,
    pub phone: ContactInput,
    pub government_info: GovInfoInput,
    pub is_deleted: bool,
}

impl From<PatientCreated> for PatientState {
    fn from(u: PatientCreated) -> Self {
        PatientState {
            id: String::from(&u.id),
            org_id: String::from(&u.org_id),
            created_by: String::from(&u.created_by),
            updated_by: String::from(&u.updated_by),
            created_at: u.created_at,
            last_updated: u.last_updated,
            user: u.user,
            address: u.address,
            phone: u.phone,
            government_info: u.government_info,
            is_deleted: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Create {
    pub id: String,
    pub org_id: RequiredString,
    pub created_by: RequiredString,
    pub updated_by: RequiredString,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub user: User,
    pub address: Address,
    pub phone: Contact,
    pub government_info: GovInfo,
}

#[derive(Debug, Clone)]
pub struct Update {
    pub id: RequiredString,
    pub org_id: RequiredString,
    pub created_by: RequiredString,
    pub updated_by: RequiredString,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct PatientAddress {
    pub created_by: RequiredString,
    pub updated_by: RequiredString,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub address: Address,
}

#[derive(Debug, Clone)]
pub struct PatientUser {
    pub created_by: RequiredString,
    pub updated_by: RequiredString,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub user: User,
}

#[derive(Debug, Clone)]
pub struct PatientPhone {
    pub created_by: RequiredString,
    pub updated_by: RequiredString,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub phone: Contact,
}

#[derive(Debug, Clone)]
pub struct PatientGov {
    pub created_by: RequiredString,
    pub updated_by: RequiredString,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub government_info: GovInfo,
}

#[derive(Debug, Clone)]
pub struct Delete {
    pub id: RequiredString,
    pub org_id: RequiredString,
    pub created_by: RequiredString,
    pub updated_by: RequiredString,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}

impl Create {
    pub fn parse(a: &CreatePatient) -> Result<Create> {
        Ok(Create {
            id: a.id.to_owned(),
            org_id: RequiredString::parse(&a.org_id)?,
            created_by: RequiredString::parse(&a.created_by)?,
            updated_by: RequiredString::parse(&a.updated_by)?,
            created_at: (a.created_at).to_owned(),
            last_updated: (a.last_updated).to_owned(),
            user: User::parse(&a.user)?,
            address: Address::parse(&a.address)?,
            phone: Contact::parse(&a.phone)?,
            government_info: GovInfo::parse(&a.government_info)?,
        })
    }
}

impl Update {
    pub fn parse(a: &UpdatePatient) -> Result<Update> {
        Ok(Update {
            id: RequiredString::parse(&a.id)?,
            org_id: RequiredString::parse(&a.org_id)?,
            created_by: RequiredString::parse(&a.created_by)?,
            updated_by: RequiredString::parse(&a.updated_by)?,
            created_at: (a.created_at).to_owned(),
            last_updated: (a.last_updated).to_owned(),
        })
    }
}

impl PatientAddress {
    pub fn parse(a: &UpdatePatientAddress) -> Result<PatientAddress> {
        Ok(PatientAddress {
            created_by: RequiredString::parse(&a.created_by)?,
            updated_by: RequiredString::parse(&a.updated_by)?,
            created_at: (a.created_at).to_owned(),
            last_updated: (a.last_updated).to_owned(),
            address: Address::parse(&a.address)?,
        })
    }
}

impl PatientPhone {
    pub fn parse(a: &UpdatePatientContactDetails) -> Result<PatientPhone> {
        Ok(PatientPhone {
            created_by: RequiredString::parse(&a.created_by)?,
            updated_by: RequiredString::parse(&a.updated_by)?,
            created_at: (a.created_at).to_owned(),
            last_updated: (a.last_updated).to_owned(),
            phone: Contact::parse(&a.phone)?,
        })
    }
}

impl PatientGov {
    pub fn parse(a: &UpdatePatientGovDetails) -> Result<PatientGov> {
        Ok(PatientGov {
            created_by: RequiredString::parse(&a.created_by)?,
            updated_by: RequiredString::parse(&a.updated_by)?,
            created_at: (a.created_at).to_owned(),
            last_updated: (a.last_updated).to_owned(),
            government_info: GovInfo::parse(&a.government_info)?,
        })
    }
}

impl PatientUser {
    pub fn parse(a: &UpdatePatientUserDetails) -> Result<PatientUser> {
        Ok(PatientUser {
            created_by: RequiredString::parse(&a.created_by)?,
            updated_by: RequiredString::parse(&a.updated_by)?,
            created_at: (a.created_at).to_owned(),
            last_updated: (a.last_updated).to_owned(),
            user: User::parse(&a.user)?,
        })
    }
}

impl Delete {
    pub fn parse(a: &DeletePatient) -> Result<Delete> {
        Ok(Delete {
            id: RequiredString::parse(&a.id)?,
            org_id: RequiredString::parse(&a.org_id)?,
            created_by: RequiredString::parse(&a.created_by)?,
            updated_by: RequiredString::parse(&a.updated_by)?,
            created_at: (a.created_at).to_owned(),
            last_updated: (a.last_updated).to_owned(),
        })
    }
}
