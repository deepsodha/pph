use anyhow::Result;
use cosmo_store_util::aggregate::Aggregate;
use log::info;

use super::{
    allergies_commands::AllergiesCommand,
    allergies_domain::{self, AllergiesState},
    allergies_events::{AllergiesCreated, AllergiesDeleted, AllergiesEvent, AllergiesUpdated},
};

#[derive(Debug, Clone)]
pub struct AllergiesAggregate {}

impl Aggregate<Option<AllergiesState>, AllergiesCommand, AllergiesEvent> for AllergiesAggregate {
    fn init(&self) -> Option<AllergiesState> {
        None
    }

    fn apply(
        &self,
        state: Option<AllergiesState>,
        event: &AllergiesEvent,
    ) -> Option<AllergiesState> {
        match event {
            AllergiesEvent::AllergiesCreated(u) => {
                info!("Applying AllergiesCreated event");
                Some(AllergiesState::from(u.clone()))
            }

            AllergiesEvent::AllergiesUpdated(u) => {
                info!("Applying AllergiesUpdated event");
                Some(AllergiesState::from(u.clone()))
            }

            AllergiesEvent::AllergiesDeleted(u) => {
                info!("Applying AllergiesDeleted event");
                let state = state.unwrap_or_default();
                Some(AllergiesState {
                    org_id: String::from(&u.org_id),
                    patient_id: String::from(&u.patient_id),
                    id: String::from(&u.id),
                    is_deleted: true,
                    ..state // As we don't need to update all properties
                })
            }
        }
    }

    fn execute(
        &self,
        _state: &Option<AllergiesState>,
        command: &AllergiesCommand,
    ) -> Result<Vec<AllergiesEvent>> {
        match command {
            AllergiesCommand::CreateAllergies(u) => {
                info!("Executing CreateAllergies command");
                let allergies = allergies_domain::Create::parse(u)?;
                Ok(vec![AllergiesEvent::AllergiesCreated(
                    AllergiesCreated::from(allergies),
                )])
            }
            AllergiesCommand::UpdateAllergies(u) => {
                info!("Executing UpdateAllergies command");
                let allergies = allergies_domain::Update::parse(u)?;
                // TODO here update logic will come.
                Ok(vec![AllergiesEvent::AllergiesUpdated(
                    AllergiesUpdated::from(allergies),
                )])
            }

            AllergiesCommand::DeleteAllergies(u) => {
                info!("Executing DeleteAllergies command");
                let allergies = allergies_domain::Delete::parse(u)?;
                // TODO here update logic will come.
                Ok(vec![AllergiesEvent::AllergiesDeleted(
                    AllergiesDeleted::from(allergies),
                )])
            }
        }
    }
}

pub const ALLERGIES_AGGREGATE: AllergiesAggregate = AllergiesAggregate {};
