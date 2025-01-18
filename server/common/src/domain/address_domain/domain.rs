use crate::domain::{
    address::{AddressLine, City, Country, PinCode, State},
    required_string::RequiredString,
};
use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::{
    command::{CreateAddress, DeleteAddress, UpdateAddress},
    events::{AddressCreated, AddressUpdated},
};

impl From<AddressCreated> for AddressState {
    fn from(u: AddressCreated) -> Self {
        AddressState {
            id: String::from(&u.id),
            created_by: String::from(&u.created_by),
            updated_by: String::from(&u.updated_by),
            created_at: u.created_at,
            last_updated: u.last_updated,
            pin_code: String::from(&u.pin_code),
            city: String::from(&u.city),
            state: String::from(&u.state),
            address_line: String::from(&u.address_line),
            country: String::from(&u.country),
        }
    }
}

impl From<AddressUpdated> for AddressState {
    fn from(u: AddressUpdated) -> Self {
        AddressState {
            id: String::from(&u.id),
            created_by: String::from(&u.created_by),
            updated_by: String::from(&u.updated_by),
            created_at: u.created_at,
            last_updated: u.last_updated,
            pin_code: String::from(&u.pin_code),
            city: String::from(&u.city),
            state: String::from(&u.state),
            address_line: String::from(&u.address_line),
            country: String::from(&u.country),
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct AddressState {
    pub id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub pin_code: String,
    pub city: String,
    pub state: String,
    pub address_line: String,
    pub country: String,
}

#[derive(Debug, Clone)]
pub struct Create {
    pub id: String,
    pub created_by: RequiredString,
    pub updated_by: RequiredString,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub pin_code: PinCode,
    pub city: City,
    pub state: State,
    pub address_line: AddressLine,
    pub country: Country,
}

#[derive(Debug, Clone)]
pub struct Update {
    pub id: RequiredString,
    pub created_by: RequiredString,
    pub updated_by: RequiredString,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub pin_code: PinCode,
    pub city: City,
    pub state: State,
    pub address_line: AddressLine,
    pub country: Country,
}

#[derive(Debug, Clone)]
pub struct Delete {
    pub id: RequiredString,
    pub created_by: RequiredString,
    pub updated_by: RequiredString,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}

impl Create {
    pub fn parse(a: &CreateAddress) -> Result<Create> {
        let u_id: &str = &uuid::Uuid::new_v4().to_string();
        // Here all the validation will come
        Ok(Create {
            id: u_id.to_owned(),
            created_by: RequiredString::parse(&a.created_by)?,
            updated_by: RequiredString::parse(&a.updated_by)?,
            created_at: (a.created_at).to_owned(),
            last_updated: (a.last_updated).to_owned(),
            pin_code: PinCode::parse(&a.pin_code)?,
            city: City::parse(&a.city)?,
            state: State::parse(&a.state)?,
            address_line: AddressLine::parse(&a.address_line)?,
            country: Country::parse(&a.country)?,
        })
    }
}

impl Update {
    pub fn parse(a: &UpdateAddress) -> Result<Update> {
        Ok(Update {
            id: RequiredString::parse(&a.id)?,
            created_by: RequiredString::parse(&a.created_by)?,
            updated_by: RequiredString::parse(&a.updated_by)?,
            created_at: (a.created_at).to_owned(),
            last_updated: (a.last_updated).to_owned(),
            pin_code: PinCode::parse(&a.pin_code)?,
            city: City::parse(&a.city)?,
            state: State::parse(&a.state)?,
            address_line: AddressLine::parse(&a.address_line)?,
            country: Country::parse(&a.country)?,
        })
    }
}
impl Delete {
    pub fn parse(a: &DeleteAddress) -> Result<Delete> {
        Ok(Delete {
            id: RequiredString::parse(&a.id)?,
            created_by: RequiredString::parse(&a.created_by)?,
            updated_by: RequiredString::parse(&a.updated_by)?,
            created_at: (a.created_at).to_owned(),
            last_updated: (a.last_updated).to_owned(),
        })
    }
}
