use chrono::{DateTime, Utc};
use common::dto::types::Types;
use cosmo_store::types::event_write::EventWrite;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::order_domain::{Create, Delete, Update};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum OrderEvent {
    OrderCreated(OrderCreated),
    OrderUpdated(OrderUpdated),
    OrderDeleted(OrderDeleted),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OrderCreated {
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
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OrderUpdated {
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
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OrderDeleted {
    pub id: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub org_id: String,
    pub patient_id: String,
}

impl From<OrderEvent> for EventWrite<OrderEvent, OrderEvent> {
    fn from(u: OrderEvent) -> Self {
        EventWrite {
            id: Uuid::new_v4(),
            correlation_id: None,
            causation_id: None,
            name: String::from("order_new_event"),
            data: u,
            metadata: None,
        }
    }
}

impl From<Create> for OrderCreated {
    fn from(s: Create) -> Self {
        OrderCreated {
            id: s.id,
            created_by: String::from(s.created_by.as_ref()),
            updated_by: String::from(s.updated_by.as_ref()),
            created_at: s.created_at,
            last_updated: s.last_updated,
            org_id: String::from(s.org_id.as_ref()),
            patient_id: String::from(s.patient_id.as_ref()),
            vaccine: String::from(s.vaccine.as_ref()),
            types: s.types,
            ordered: s.ordered,
            provider: s.provider,
            comments: String::from(s.comments.as_ref()),
        }
    }
}

impl From<Update> for OrderUpdated {
    fn from(s: Update) -> Self {
        OrderUpdated {
            id: String::from(s.id.as_ref()),
            created_by: String::from(s.created_by.as_ref()),
            updated_by: String::from(s.updated_by.as_ref()),
            created_at: s.created_at,
            last_updated: s.last_updated,
            org_id: String::from(s.org_id.as_ref()),
            patient_id: String::from(s.patient_id.as_ref()),
            vaccine: String::from(s.vaccine.as_ref()),
            types: s.types,
            ordered: s.ordered,
            provider: s.provider,
            comments: String::from(s.comments.as_ref()),
        }
    }
}

impl From<Delete> for OrderDeleted {
    fn from(s: Delete) -> Self {
        OrderDeleted {
            id: String::from(s.id.as_ref()),
            created_by: String::from(s.created_by.as_ref()),
            updated_by: String::from(s.updated_by.as_ref()),
            created_at: s.created_at,
            last_updated: s.last_updated,
            org_id: String::from(s.org_id.as_ref()),
            patient_id: String::from(s.patient_id.as_ref()),
        }
    }
}
