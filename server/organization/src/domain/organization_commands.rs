use chrono::{DateTime, Utc};
use common::dto::address::AddressInput;
use serde::{Deserialize, Serialize};

use crate::dto::select_organization::SelectOrganization;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum OrganizationCommand {
    CreateOrganization(CreateOrganization),
    UpdateOrganization(UpdateOrganization),
    DeleteOrganization(DeleteOrganization),
    SelectOrganization(OrganizationSelect),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateOrganization {
    pub id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub org_name: String,
    pub details: String,
    pub phone_number: String,
    pub email: String,
    pub address: AddressInput,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateOrganization {
    pub id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub org_name: String,
    pub details: String,
    pub phone_number: String,
    pub email: String,
    pub address: AddressInput,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeleteOrganization {
    pub id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OrganizationSelect {
    pub id: String,
    pub name: String,
}

impl From<SelectOrganization> for OrganizationSelect {
    fn from(u: SelectOrganization) -> Self {
        OrganizationSelect {
            id: String::from(&u.id),
            name: String::from(&u.name),
        }
    }
}
