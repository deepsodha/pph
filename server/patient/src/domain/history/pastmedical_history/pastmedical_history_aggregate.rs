use anyhow::Result;
use cosmo_store_util::aggregate::Aggregate;
use log::info;

use super::{
    pastmedical_history_commands::PastMedicalHistoryCommand,
    pastmedical_history_domain::{self, PastMedicalHistoryState},
    pastmedical_history_events::{
        PastMedicalHistoryCreated, PastMedicalHistoryDeleted, PastMedicalHistoryEvent,
        PastMedicalHistoryUpdated,
    },
};

#[derive(Debug, Clone)]
pub struct PastMedicalHistoryAggregate {}

impl Aggregate<Option<PastMedicalHistoryState>, PastMedicalHistoryCommand, PastMedicalHistoryEvent>
    for PastMedicalHistoryAggregate
{
    fn init(&self) -> Option<PastMedicalHistoryState> {
        None
    }

    fn apply(
        &self,
        state: Option<PastMedicalHistoryState>,
        event: &PastMedicalHistoryEvent,
    ) -> Option<PastMedicalHistoryState> {
        match event {
            PastMedicalHistoryEvent::PastMedicalHistoryCreated(u) => {
                info!("Applying PastMedicalHistoryCreated event");
                Some(PastMedicalHistoryState::from(u.clone()))
            }

            PastMedicalHistoryEvent::PastMedicalHistoryUpdated(u) => {
                info!("Applying PastMedicalHistoryUpdated event");
                Some(PastMedicalHistoryState::from(u.clone()))
            }

            PastMedicalHistoryEvent::PastMedicalHistoryDeleted(u) => {
                info!("Applying PastMedicalHistoryDeleted event");
                let state: PastMedicalHistoryState = state.unwrap_or_default();
                Some(PastMedicalHistoryState {
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
        _state: &Option<PastMedicalHistoryState>,
        command: &PastMedicalHistoryCommand,
    ) -> Result<Vec<PastMedicalHistoryEvent>> {
        match command {
            PastMedicalHistoryCommand::CreatePastMedicalHistory(u) => {
                info!("Executing CreatePastMedicalHistory command");
                let pastmedicalhistory = pastmedical_history_domain::Create::parse(u)?;
                Ok(vec![PastMedicalHistoryEvent::PastMedicalHistoryCreated(
                    PastMedicalHistoryCreated::from(pastmedicalhistory),
                )])
            }
            PastMedicalHistoryCommand::UpdatePastMedicalHistory(u) => {
                info!("Executing UpdatePastMedicalHistory command");
                let pastmedicalhistory = pastmedical_history_domain::Update::parse(u)?;
                Ok(vec![PastMedicalHistoryEvent::PastMedicalHistoryUpdated(
                    PastMedicalHistoryUpdated::from(pastmedicalhistory),
                )])
            }

            PastMedicalHistoryCommand::DeletePastMedicalHistory(u) => {
                info!("Executing DeletePastMedicalHistory command");
                let pastmedicalhistory = pastmedical_history_domain::Delete::parse(u)?;
                Ok(vec![PastMedicalHistoryEvent::PastMedicalHistoryDeleted(
                    PastMedicalHistoryDeleted::from(pastmedicalhistory),
                )])
            }
        }
    }
}

pub const PASTMEDICALHISTORY_AGGREGATE: PastMedicalHistoryAggregate =
    PastMedicalHistoryAggregate {};
