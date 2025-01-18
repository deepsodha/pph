use super::{
    user_commands::{
        CreateUser, DeleteUser, UpdateUserAddress, UpdateUserContact, UpdateUserDetails,
        UpdateUserGovDetails, UpdateUserPassword,
    },
    user_events::UserCreated,
};
use anyhow::{Ok, Result};
use chrono::{DateTime, Utc};
use common::{
    domain::{
        address::Address, contact::Contact, goverment::GovInfo, password::ValidPassword,
        required_string::RequiredString, user::User,
    },
    dto::{address::AddressInput, contact::ContactInput, gov_info::GovInfoInput, user::UserInput},
};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Clone, Debug, Default, Serialize, Deserialize, TS, PartialEq, Eq, sqlx::FromRow)]
#[ts(export, export_to = "../../bindings/UserState.ts")]
pub struct UserState {
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
    pub password: String,
    pub is_deleted: bool,
}

impl From<UserCreated> for UserState {
    fn from(u: UserCreated) -> Self {
        UserState {
            org_id: String::from(&u.org_id),
            id: String::from(&u.id),
            created_by: String::from(&u.created_by),
            updated_by: String::from(&u.updated_by),
            created_at: u.created_at,
            last_updated: u.last_updated,
            user: u.user,
            address: u.address,
            phone: u.phone,
            government_info: u.government_info,
            password: u.password,
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
    pub phone: Contact,
    pub user: User,
    pub address: Address,
    pub government_info: GovInfo,
    pub password: ValidPassword,
}

#[derive(Debug, Clone)]
pub struct UserPasswordUpdate {
    pub id: RequiredString,
    pub created_by: RequiredString,
    pub updated_by: RequiredString,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub password: ValidPassword,
}

#[derive(Debug, Clone)]
pub struct UserContactUpdate {
    pub created_by: RequiredString,
    pub updated_by: RequiredString,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub phone: Contact,
}

#[derive(Debug, Clone)]
pub struct UserDetailsUpdate {
    pub created_by: RequiredString,
    pub updated_by: RequiredString,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub user: User,
}

#[derive(Debug, Clone)]
pub struct UserAddressUpdate {
    pub created_by: RequiredString,
    pub updated_by: RequiredString,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub address: Address,
}

#[derive(Debug, Clone)]
pub struct UserGovUpdate {
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
    pub fn parse(a: &CreateUser) -> Result<Create> {
        Ok(Create {
            id: a.id.to_owned(),
            org_id: RequiredString::parse(&a.org_id)?,
            created_by: RequiredString::parse(&a.created_by)?,
            updated_by: RequiredString::parse(&a.updated_by)?,
            created_at: (a.created_at).to_owned(),
            last_updated: (a.last_updated).to_owned(),
            password: ValidPassword::parse(&a.password)?,
            phone: Contact::parse(&a.phone)?,
            user: User::parse(&a.user)?,
            address: Address::parse(&a.address)?,
            government_info: GovInfo::parse(&a.government_info)?,
        })
    }
}

impl UserContactUpdate {
    pub fn parse(a: &UpdateUserContact) -> Result<UserContactUpdate> {
        Ok(UserContactUpdate {
            created_by: RequiredString::parse(&a.created_by)?,
            updated_by: RequiredString::parse(&a.updated_by)?,
            created_at: (a.created_at).to_owned(),
            last_updated: (a.last_updated).to_owned(),
            phone: Contact::parse(&a.phone)?,
        })
    }
}

impl UserPasswordUpdate {
    pub fn parse(a: &UpdateUserPassword) -> Result<UserPasswordUpdate> {
        Ok(UserPasswordUpdate {
            id: RequiredString::parse(&a.id)?,
            created_by: RequiredString::parse(&a.created_by)?,
            updated_by: RequiredString::parse(&a.updated_by)?,
            created_at: (a.created_at).to_owned(),
            last_updated: (a.last_updated).to_owned(),
            password: ValidPassword::parse(&a.password)?,
        })
    }
}

impl UserAddressUpdate {
    pub fn parse(a: &UpdateUserAddress) -> Result<UserAddressUpdate> {
        Ok(UserAddressUpdate {
            created_by: RequiredString::parse(&a.created_by)?,
            updated_by: RequiredString::parse(&a.updated_by)?,
            created_at: (a.created_at).to_owned(),
            last_updated: (a.last_updated).to_owned(),
            address: Address::parse(&a.address)?,
        })
    }
}

impl UserGovUpdate {
    pub fn parse(a: &UpdateUserGovDetails) -> Result<UserGovUpdate> {
        Ok(UserGovUpdate {
            created_by: RequiredString::parse(&a.created_by)?,
            updated_by: RequiredString::parse(&a.updated_by)?,
            created_at: (a.created_at).to_owned(),
            last_updated: (a.last_updated).to_owned(),
            government_info: GovInfo::parse(&a.government_info)?,
        })
    }
}

impl UserDetailsUpdate {
    pub fn parse(a: &UpdateUserDetails) -> Result<UserDetailsUpdate> {
        Ok(UserDetailsUpdate {
            created_by: RequiredString::parse(&a.created_by)?,
            updated_by: RequiredString::parse(&a.updated_by)?,
            created_at: (a.created_at).to_owned(),
            last_updated: (a.last_updated).to_owned(),
            user: User::parse(&a.user)?,
        })
    }
}
impl Delete {
    pub fn parse(a: &DeleteUser) -> Result<Delete> {
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
