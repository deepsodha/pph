use super::{
    order_commands::OrderCommand,
    order_domain::{self, OrderState},
    order_events::{OrderCreated, OrderDeleted, OrderEvent, OrderUpdated},
};
use anyhow::Result;
use cosmo_store_util::aggregate::Aggregate;
use log::info;

#[derive(Debug, Clone)]
pub struct OrderAggregate {}

impl Aggregate<Option<OrderState>, OrderCommand, OrderEvent> for OrderAggregate {
    fn init(&self) -> Option<OrderState> {
        None
    }

    fn apply(&self, state: Option<OrderState>, event: &OrderEvent) -> Option<OrderState> {
        match event {
            OrderEvent::OrderCreated(u) => {
                info!("Applying OrderCreated event");
                Some(OrderState::from(u.clone()))
            }

            OrderEvent::OrderUpdated(u) => {
                info!("Applying OrderUpdated event");
                Some(OrderState::from(u.clone()))
            }

            OrderEvent::OrderDeleted(u) => {
                info!("Applying OrderDeleted event");
                let state = state.unwrap_or_default();
                Some(OrderState {
                    id: String::from(&u.id),
                    patient_id: String::from(&u.patient_id),
                    org_id: String::from(&u.org_id),
                    is_deleted: true,
                    ..state // As we don't need to update all properties
                })
            }
        }
    }

    fn execute(
        &self,
        _state: &Option<OrderState>,
        command: &OrderCommand,
    ) -> Result<Vec<OrderEvent>> {
        match command {
            OrderCommand::CreateOrder(u) => {
                info!("Executing CreateOrder command");
                let order = order_domain::Create::parse(u)?;
                Ok(vec![OrderEvent::OrderCreated(OrderCreated::from(order))])
            }
            OrderCommand::UpdateOrder(u) => {
                info!("Executing UpdateOrder command");
                let order = order_domain::Update::parse(u)?;
                Ok(vec![OrderEvent::OrderUpdated(OrderUpdated::from(order))])
            }

            OrderCommand::DeleteOrder(u) => {
                info!("Executing DeleteOrder command");
                let order = order_domain::Delete::parse(u)?;
                Ok(vec![OrderEvent::OrderDeleted(OrderDeleted::from(order))])
            }
        }
    }
}

pub const ORDER_AGGREGATE: OrderAggregate = OrderAggregate {};
