use super::{
    pastsurgical_history_commands::PastSurgicalCommand,
    pastsurgical_history_domain::{self, PastSurgicalHistoryState},
    pastsurgical_history_events::{
        PastSurgicalEvent, PastSurgicalHistoryCreated, PastSurgicalHistoryDeleted,
        PastSurgicalHistoryUpdated,
    },
};
use anyhow::Result;
use cosmo_store_util::aggregate::Aggregate;
use log::info;

#[derive(Debug, Clone)]
pub struct PastSurgicalHistoryAggregate {}

impl Aggregate<Option<PastSurgicalHistoryState>, PastSurgicalCommand, PastSurgicalEvent>
    for PastSurgicalHistoryAggregate
{
    fn init(&self) -> Option<PastSurgicalHistoryState> {
        None
    }

    fn apply(
        &self,
        state: Option<PastSurgicalHistoryState>,
        event: &PastSurgicalEvent,
    ) -> Option<PastSurgicalHistoryState> {
        match event {
            PastSurgicalEvent::PastSurgicalHistoryCreated(u) => {
                info!("Applying PastSurgicalHistoryCreated event");
                Some(PastSurgicalHistoryState::from(u.clone()))
            }

            PastSurgicalEvent::PastSurgicalHistoryUpdated(u) => {
                info!("Applying PastSurgicalHistoryUpdated event");
                Some(PastSurgicalHistoryState::from(u.clone()))
            }

            PastSurgicalEvent::PastSurgicalHistoryDeleted(u) => {
                info!("Applying PastSurgicalHistoryDeleted event");
                let state: PastSurgicalHistoryState = state.unwrap_or_default();
                Some(PastSurgicalHistoryState {
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
        _state: &Option<PastSurgicalHistoryState>,
        command: &PastSurgicalCommand,
    ) -> Result<Vec<PastSurgicalEvent>> {
        match command {
            PastSurgicalCommand::CreatePastSurgicalHistory(u) => {
                info!("Executing CreatePastSurgicalHistory command");
                let pastsurgicalhistory = pastsurgical_history_domain::Create::parse(u)?;
                Ok(vec![PastSurgicalEvent::PastSurgicalHistoryCreated(
                    PastSurgicalHistoryCreated::from(pastsurgicalhistory),
                )])
            }
            PastSurgicalCommand::UpdatePastSurgicalHistory(u) => {
                info!("Executing UpdatePastSurgicalHistory command");
                let pastsurgicalhistory = pastsurgical_history_domain::Update::parse(u)?;
                Ok(vec![PastSurgicalEvent::PastSurgicalHistoryUpdated(
                    PastSurgicalHistoryUpdated::from(pastsurgicalhistory),
                )])
            }

            PastSurgicalCommand::DeletePastSurgicalHistory(u) => {
                info!("Executing DeletePastSurgicalHistory command");
                let pastsurgicalhistory = pastsurgical_history_domain::Delete::parse(u)?;
                Ok(vec![PastSurgicalEvent::PastSurgicalHistoryDeleted(
                    PastSurgicalHistoryDeleted::from(pastsurgicalhistory),
                )])
            }
        }
    }
}

pub const PASTSURGICALHISTORY_AGGREGATE: PastSurgicalHistoryAggregate =
    PastSurgicalHistoryAggregate {};
