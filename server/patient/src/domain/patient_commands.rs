use chrono::{DateTime, Utc};
use common::dto::contact::ContactInput;
use common::dto::gov_info::GovInfoInput;
use common::dto::{address::AddressInput, user::UserInput};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreatePatient {
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
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdatePatient {
    pub id: String,
    pub org_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdatePatientAddress {
    pub id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub address: AddressInput,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdatePatientContactDetails {
    pub id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub phone: ContactInput,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdatePatientGovDetails {
    pub id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub government_info: GovInfoInput,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdatePatientUserDetails {
    pub id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub user: UserInput,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeletePatient {
    pub id: String,
    pub org_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PatientCommand {
    CreatePatient(Box<CreatePatient>),
    UpdatePatient(UpdatePatient),
    UpdatePatientAddress(UpdatePatientAddress),
    UpdatePatientUserDetails(Box<UpdatePatientUserDetails>),
    UpdatePatientContactDetails(UpdatePatientContactDetails),
    UpdatePatientGovDetails(UpdatePatientGovDetails),
    DeletePatient(DeletePatient),
}
