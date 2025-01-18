use anyhow::Result;
use chrono::{DateTime, Utc};
use common::{
    domain::{min_max_words::RequiredMax300Words, required_string::RequiredString},
    dto::types::Types,
};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use super::{
    order_commands::{CreateOrder, DeleteOrder, UpdateOrder},
    order_events::{OrderCreated, OrderUpdated},
};

#[derive(Debug, Clone)]
pub struct Create {
    pub id: String,
    pub created_by: RequiredString,
    pub updated_by: RequiredString,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub org_id: RequiredString,
    pub patient_id: RequiredString,
    pub vaccine: RequiredString,
    pub types: Types,
    pub ordered: DateTime<Utc>,
    pub provider: String,
    pub comments: RequiredMax300Words,
}

#[derive(Debug, Clone)]
pub struct Update {
    pub id: RequiredString,
    pub created_by: RequiredString,
    pub updated_by: RequiredString,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub org_id: RequiredString,
    pub patient_id: RequiredString,
    pub vaccine: RequiredString,
    pub types: Types,
    pub ordered: DateTime<Utc>,
    pub provider: String,
    pub comments: RequiredMax300Words,
}

#[derive(Debug, Clone)]
pub struct Delete {
    pub id: RequiredString,
    pub created_by: RequiredString,
    pub updated_by: RequiredString,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub org_id: RequiredString,
    pub patient_id: RequiredString,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq, TS)]
#[ts(export, export_to = "../../bindings/OrderState.ts")]
pub struct OrderState {
    pub id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub org_id: String,
    pub patient_id: String,
    pub vaccine: String,
    pub types: Types,
    pub ordered: DateTime<Utc>,
    pub provider: String,
    pub comments: String,
    pub is_deleted: bool,
}

impl From<OrderCreated> for OrderState {
    fn from(u: OrderCreated) -> Self {
        OrderState {
            id: String::from(&u.id),
            created_by: String::from(&u.created_by),
            updated_by: String::from(&u.updated_by),
            created_at: u.created_at,
            last_updated: u.last_updated,
            org_id: String::from(&u.org_id),
            patient_id: String::from(&u.patient_id),
            vaccine: String::from(&u.vaccine),
            types: u.types,
            ordered: u.ordered,
            provider: u.provider,
            comments: String::from(&u.comments),
            is_deleted: false,
        }
    }
}

impl From<OrderUpdated> for OrderState {
    fn from(u: OrderUpdated) -> Self {
        OrderState {
            id: String::from(&u.id),
            created_by: String::from(&u.created_by),
            updated_by: String::from(&u.updated_by),
            created_at: u.created_at,
            last_updated: u.last_updated,
            org_id: String::from(&u.org_id),
            patient_id: String::from(&u.patient_id),
            vaccine: String::from(&u.vaccine),
            types: u.types,
            ordered: u.ordered,
            provider: u.provider,
            comments: String::from(&u.comments),
            is_deleted: false,
        }
    }
}

impl Create {
    pub fn parse(a: &CreateOrder) -> Result<Create> {
        Ok(Create {
            id: a.id.to_owned(),
            created_by: RequiredString::parse(&a.created_by)?,
            updated_by: RequiredString::parse(&a.updated_by)?,
            created_at: (a.created_at).to_owned(),
            last_updated: (a.last_updated).to_owned(),
            org_id: RequiredString::parse(&a.org_id)?,
            patient_id: RequiredString::parse(&a.patient_id)?,
            vaccine: RequiredString::parse(&a.vaccine)?,
            types: (a.types).to_owned(),
            ordered: (a.ordered).to_owned(),
            provider: (a.provider).to_owned(),
            comments: RequiredMax300Words::parse(&a.comments)?,
        })
    }
}

impl Update {
    pub fn parse(a: &UpdateOrder) -> Result<Update> {
        Ok(Update {
            created_by: RequiredString::parse(&a.created_by)?,
            updated_by: RequiredString::parse(&a.updated_by)?,
            created_at: (a.created_at).to_owned(),
            last_updated: (a.last_updated).to_owned(),
            id: RequiredString::parse(&a.id)?,
            org_id: RequiredString::parse(&a.org_id)?,
            patient_id: RequiredString::parse(&a.patient_id)?,
            vaccine: RequiredString::parse(&a.vaccine)?,
            types: (a.types).to_owned(),
            ordered: (a.ordered).to_owned(),
            provider: (a.provider).to_owned(),
            comments: RequiredMax300Words::parse(&a.comments)?,
        })
    }
}

impl Delete {
    pub fn parse(a: &DeleteOrder) -> Result<Delete> {
        Ok(Delete {
            created_by: RequiredString::parse(&a.created_by)?,
            updated_by: RequiredString::parse(&a.updated_by)?,
            created_at: (a.created_at).to_owned(),
            last_updated: (a.last_updated).to_owned(),
            id: RequiredString::parse(&a.id)?,
            org_id: RequiredString::parse(&a.org_id)?,
            patient_id: RequiredString::parse(&a.patient_id)?,
        })
    }
}
