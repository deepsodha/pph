use super::{
    familyhistory_commands::FamilyHistoryCommand,
    familyhistory_domain::{self, FamilyHistoryState},
    familyhistory_events::{
        FamilyHistoryCreated, FamilyHistoryDeleted, FamilyHistoryEvent, FamilyHistoryUpdated,
    },
};
use anyhow::Result;
use cosmo_store_util::aggregate::Aggregate;
use log::info;

#[derive(Debug, Clone)]
pub struct FamilyHistoryAggregate {}

impl Aggregate<Option<FamilyHistoryState>, FamilyHistoryCommand, FamilyHistoryEvent>
    for FamilyHistoryAggregate
{
    fn init(&self) -> Option<FamilyHistoryState> {
        None
    }

    fn apply(
        &self,
        state: Option<FamilyHistoryState>,
        event: &FamilyHistoryEvent,
    ) -> Option<FamilyHistoryState> {
        match event {
            FamilyHistoryEvent::FamilyHistoryCreated(u) => {
                info!("Applying FamilyHistoryCreated event");
                Some(FamilyHistoryState::from(u.clone()))
            }

            FamilyHistoryEvent::FamilyHistoryUpdated(u) => {
                info!("Applying FamilyHistoryUpdated event");
                Some(FamilyHistoryState::from(u.clone()))
            }

            FamilyHistoryEvent::FamilyHistoryDeleted(u) => {
                info!("Applying FamilyHistoryDeleted event");
                let state: FamilyHistoryState = state.unwrap_or_default();
                Some(FamilyHistoryState {
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
        _state: &Option<FamilyHistoryState>,
        command: &FamilyHistoryCommand,
    ) -> Result<Vec<FamilyHistoryEvent>> {
        match command {
            FamilyHistoryCommand::CreateFamilyHistory(u) => {
                info!("Executing CreateFamilyHistory command");
                let familyhistory = familyhistory_domain::Create::parse(u)?;
                Ok(vec![FamilyHistoryEvent::FamilyHistoryCreated(
                    FamilyHistoryCreated::from(familyhistory),
                )])
            }
            FamilyHistoryCommand::UpdateFamilyHistory(u) => {
                info!("Executing UpdateFamilyHistory command");
                let familyhistory = familyhistory_domain::Update::parse(u)?;
                // TODO here update logic will come.
                Ok(vec![FamilyHistoryEvent::FamilyHistoryUpdated(
                    FamilyHistoryUpdated::from(familyhistory),
                )])
            }

            FamilyHistoryCommand::DeleteFamilyHistory(u) => {
                info!("Executing DeleteFamilyHistory command");
                let familyhistory = familyhistory_domain::Delete::parse(u)?;
                // TODO here update logic will come.
                Ok(vec![FamilyHistoryEvent::FamilyHistoryDeleted(
                    FamilyHistoryDeleted::from(familyhistory),
                )])
            }
        }
    }
}

pub const FAMILYHISTORY_AGGREGATE: FamilyHistoryAggregate = FamilyHistoryAggregate {};
