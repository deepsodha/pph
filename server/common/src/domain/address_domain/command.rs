use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::domain::address_dto::{
    address_add::AddressAdd, address_delete::AddressDelete, address_update::AddressUpdate,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateAddress {
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

impl From<AddressAdd> for CreateAddress {
    fn from(u: AddressAdd) -> Self {
        CreateAddress {
            // TODO: IT should come from token
            created_by: String::from("deep"),
            // TODO: IT should come from token
            updated_by: String::from("deep"),
            created_at: Utc::now(),
            last_updated: Utc::now(),
            pin_code: String::from(&u.pin_code),
            city: String::from(&u.city),
            state: String::from(&u.state),
            address_line: String::from(&u.address_line),
            country: String::from(&u.country),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateAddress {
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

impl From<AddressUpdate> for UpdateAddress {
    fn from(u: AddressUpdate) -> Self {
        UpdateAddress {
            id: String::from(&u.id),
            // TODO: IT should come from token
            created_by: String::from("deep"),
            // TODO: IT should come from token
            updated_by: String::from("deep"),
            created_at: Utc::now(),
            last_updated: Utc::now(),
            pin_code: String::from(&u.pin_code),
            city: String::from(&u.city),
            state: String::from(&u.state),
            address_line: String::from(&u.address_line),
            country: String::from(&u.country),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeleteAddress {
    pub id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}

impl From<AddressDelete> for DeleteAddress {
    fn from(u: AddressDelete) -> Self {
        DeleteAddress {
            id: String::from(&u.id),
            // TODO: IT should come from token
            created_by: String::from("deep"),
            // TODO: IT should come from token
            updated_by: String::from("deep"),
            created_at: Utc::now(),
            last_updated: Utc::now(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AddressCommands {
    CreateAddress(CreateAddress),
    UpdateAddress(UpdateAddress),
    DeleteAddress(DeleteAddress),
}
