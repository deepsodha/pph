use super::{
    hospitalization_commands::HospitalizationCommand,
    hospitalization_domain::{self, HospitalizationState},
    hospitalization_events::{
        HospitalizationCreated, HospitalizationDeleted, HospitalizationEvent,
        HospitalizationUpdated,
    },
};
use anyhow::Result;
use cosmo_store_util::aggregate::Aggregate;
use log::info;

#[derive(Debug, Clone)]
pub struct HospitalizationAggregate {}

impl Aggregate<Option<HospitalizationState>, HospitalizationCommand, HospitalizationEvent>
    for HospitalizationAggregate
{
    fn init(&self) -> Option<HospitalizationState> {
        None
    }

    fn apply(
        &self,
        state: Option<HospitalizationState>,
        event: &HospitalizationEvent,
    ) -> Option<HospitalizationState> {
        match event {
            HospitalizationEvent::HospitalizationCreated(u) => {
                info!("Applying HospitalizationCreated event");
                Some(HospitalizationState::from(u.clone()))
            }

            HospitalizationEvent::HospitalizationUpdated(u) => {
                info!("Applying HospitalizationUpdated event");
                Some(HospitalizationState::from(u.clone()))
            }

            HospitalizationEvent::HospitalizationDeleted(u) => {
                info!("Applying HospitalizationDeleted event");
                let state: HospitalizationState = state.unwrap_or_default();
                Some(HospitalizationState {
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
        _state: &Option<HospitalizationState>,
        command: &HospitalizationCommand,
    ) -> Result<Vec<HospitalizationEvent>> {
        match command {
            HospitalizationCommand::CreateHospitalization(u) => {
                info!("Executing CreateHospitalization command");
                let hospitalization = hospitalization_domain::Create::parse(u)?;
                Ok(vec![HospitalizationEvent::HospitalizationCreated(
                    HospitalizationCreated::from(hospitalization),
                )])
            }
            HospitalizationCommand::UpdateHospitalization(u) => {
                info!("Executing UpdateHospitalization command");
                let hospitalization = hospitalization_domain::Update::parse(u)?;
                // TODO here update logic will come.
                Ok(vec![HospitalizationEvent::HospitalizationUpdated(
                    HospitalizationUpdated::from(hospitalization),
                )])
            }

            HospitalizationCommand::DeleteHospitalization(u) => {
                info!("Executing DeleteHospitalization command");
                let hospitalization = hospitalization_domain::Delete::parse(u)?;
                // TODO here update logic will come.
                Ok(vec![HospitalizationEvent::HospitalizationDeleted(
                    HospitalizationDeleted::from(hospitalization),
                )])
            }
        }
    }
}

pub const HOSPITALIZATION_AGGREGATE: HospitalizationAggregate = HospitalizationAggregate {};
