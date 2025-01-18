use chrono::{DateTime, Utc};
use common::dto::{contact::PhoneNoTypeForContact, gender::GenderType, gov_info::GovIdType};
use serde::{Deserialize, Serialize};
use sqlx;
use ts_rs::TS;

#[derive(Deserialize, Serialize, Debug, Clone, Default, PartialEq, Eq, sqlx::FromRow, TS)]
#[ts(export, export_to = "../../bindings/GetSystemadmin.ts")]
pub struct GetSystemadmin {
    pub id: String,
    pub org_id: String,
    pub first_name: String,
    pub middle_name: String,
    pub last_name: String,
    pub dob: DateTime<Utc>,
    pub email: String,
    pub gender: GenderType,
    pub photo_url: String,
    pub pin_code: String,
    pub city: String,
    pub state: String,
    pub address_line: String,
    pub country: String,
    pub number: String,
    pub number_type: PhoneNoTypeForContact,
    pub id_no: String,
    pub id_type: GovIdType,
}
