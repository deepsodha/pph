use anyhow::Result;
use cosmo_store_util::aggregate::Aggregate;
use log::info;

use crate::domain::prescription_domain;

use super::{
    prescription_commands::PrescriptionCommand,
    prescription_domain::PrescriptionState,
    prescription_events::{
        PrescriptionCreated, PrescriptionDeleted, PrescriptionEvent, PrescriptionUpdated,
    },
};

#[derive(Debug, Clone)]
pub struct PrescriptionAggregate {}

impl Aggregate<Option<PrescriptionState>, PrescriptionCommand, PrescriptionEvent>
    for PrescriptionAggregate
{
    fn init(&self) -> Option<PrescriptionState> {
        None
    }

    fn apply(
        &self,
        state: Option<PrescriptionState>,
        event: &PrescriptionEvent,
    ) -> Option<PrescriptionState> {
        match event {
            PrescriptionEvent::PrescriptionCreated(u) => {
                info!("Applying PrescriptionCreated event");
                Some(PrescriptionState::from(u.clone()))
            }

            PrescriptionEvent::PrescriptionUpdated(u) => {
                info!("Applying PrescriptionUpdated event");
                Some(PrescriptionState::from(u.clone()))
            }

            PrescriptionEvent::PrescriptionDeleted(s) => {
                info!("Applying PrescriptionDeleted event");
                let state = state.unwrap_or_default();
                Some(PrescriptionState {
                    id: String::from(&s.prescription_id),
                    org_id: String::from(&s.org_id),
                    is_deleted: true,
                    ..state
                })
            }
        }
    }

    fn execute(
        &self,
        _state: &Option<PrescriptionState>,
        command: &PrescriptionCommand,
    ) -> Result<Vec<PrescriptionEvent>> {
        match command {
            PrescriptionCommand::CreatePrescription(u) => {
                info!("Executing CreatePrescription command");
                let input = prescription_domain::Create::parse(u)?;
                Ok(vec![PrescriptionEvent::PrescriptionCreated(
                    PrescriptionCreated::from(input),
                )])
            }
            PrescriptionCommand::UpdatedPrescription(u) => {
                info!("Executing UpdatedPrescription command");
                let input = prescription_domain::Update::parse(u)?;
                Ok(vec![PrescriptionEvent::PrescriptionUpdated(
                    PrescriptionUpdated::from(input),
                )])
            }

            PrescriptionCommand::DeletePrescription(u) => {
                info!("Executing DeletePrescription command");
                let input = prescription_domain::Delete::parse(u)?;
                Ok(vec![PrescriptionEvent::PrescriptionDeleted(
                    PrescriptionDeleted::from(input),
                )])
            }
        }
    }
}

pub const PRESCRIPTION_AGGREGATE: PrescriptionAggregate = PrescriptionAggregate {};
