use anyhow::Result;
use cosmo_store_util::aggregate::Aggregate;

use super::{
    command::AddressCommands,
    domain::{self, AddressState},
    events::{AddressCreated, AddressDeleted, AddressEvent, AddressUpdated},
};

#[derive(Debug, Clone)]
pub struct AddressAggregate {}

impl Aggregate<Option<AddressState>, AddressCommands, AddressEvent> for AddressAggregate {
    fn init(&self) -> Option<AddressState> {
        None
    }

    fn apply(&self, state: Option<AddressState>, event: &AddressEvent) -> Option<AddressState> {
        match event {
            AddressEvent::AddressCreated(u) => Some(AddressState::from(u.clone())),
            AddressEvent::AddressUpdated(u) => Some(AddressState::from(u.clone())),
            AddressEvent::AddressDeleted(u) => {
                let state = state.unwrap_or_default();
                Some(AddressState {
                    id: String::from(&u.id),
                    ..state // As we don't need to update all properties
                })
            }
        }
    }

    fn execute(
        &self,
        _state: &Option<AddressState>,
        command: &AddressCommands,
    ) -> Result<Vec<AddressEvent>> {
        match command {
            AddressCommands::CreateAddress(u) => {
                let data = domain::Create::parse(u)?;
                Ok(vec![AddressEvent::AddressCreated(AddressCreated::from(
                    data,
                ))])
            }
            AddressCommands::UpdateAddress(u) => {
                let data = domain::Update::parse(u)?;
                Ok(vec![AddressEvent::AddressUpdated(AddressUpdated::from(
                    data,
                ))])
            }
            AddressCommands::DeleteAddress(u) => {
                let data = domain::Delete::parse(u)?;
                Ok(vec![AddressEvent::AddressDeleted(AddressDeleted::from(
                    data,
                ))])
            }
        }
    }
}

pub const ADDRESS_AGGREGATE: AddressAggregate = AddressAggregate {};
