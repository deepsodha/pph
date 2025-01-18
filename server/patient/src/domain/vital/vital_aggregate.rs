use crate::domain::vital::vital_domain;
use crate::domain::vital::vital_events::{
    VitalsCreated, VitalsDeleted, VitalsEvent, VitalsUpdated,
};
use crate::domain::vital::{vital_commands::VitalsCommand, vital_domain::VitalsState};
use anyhow::Result;
use cosmo_store_util::aggregate::Aggregate;
use log::info;

#[derive(Debug, Clone)]
pub struct VitalsAggregate {}

impl Aggregate<Option<VitalsState>, VitalsCommand, VitalsEvent> for VitalsAggregate {
    fn init(&self) -> Option<VitalsState> {
        None
    }

    fn apply(&self, state: Option<VitalsState>, event: &VitalsEvent) -> Option<VitalsState> {
        match event {
            VitalsEvent::VitalsCreated(s) => {
                info!("Applying VitalsCreated event");
                Some(VitalsState::from(s.clone()))
            }
            VitalsEvent::VitalsUpdated(s) => {
                info!("Applying VitalsUpdated event");
                Some(VitalsState::from(s.clone()))
            }
            VitalsEvent::VitalsDeleted(s) => {
                info!("Applying VitalsDeleted event");
                let state = state.unwrap_or_default();
                Some(VitalsState {
                    org_id: String::from(&s.org_id),
                    patient_id: String::from(&s.patient_id),
                    id: String::from(&s.id),
                    is_deleted: true,
                    ..state // As we don't need to update all properties
                })
            }
        }
    }

    fn execute(
        &self,
        _state: &Option<VitalsState>,
        command: &VitalsCommand,
    ) -> Result<Vec<VitalsEvent>> {
        match command {
            VitalsCommand::CreateVitals(u) => {
                info!("Executing CreateVitals command");
                let input = vital_domain::Create::parse(u)?;
                Ok(vec![VitalsEvent::VitalsCreated(VitalsCreated::from(input))])
            }
            VitalsCommand::UpdateVitals(u) => {
                info!("Executing UpdateVitals command");
                let input = vital_domain::Update::parse(u)?;
                // TODO here update logic will come.
                Ok(vec![VitalsEvent::VitalsUpdated(VitalsUpdated::from(input))])
            }
            VitalsCommand::DeleteVitals(u) => {
                info!("Executing DeleteVitals command");
                let input = vital_domain::Delete::parse(u)?;
                // TODO here update logic will come.
                Ok(vec![VitalsEvent::VitalsDeleted(VitalsDeleted::from(input))])
            }
        }
    }
}

pub const VITALS_AGGREGATE: VitalsAggregate = VitalsAggregate {};
