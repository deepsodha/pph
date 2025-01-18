use super::{
    medication_commands::MedicationsCommand,
    medication_domain::{self, MedicationsState},
    medication_events::{
        MedicationCreated, MedicationDeleted, MedicationUpdated, MedicationsEvent,
    },
};
use anyhow::Result;
use cosmo_store_util::aggregate::Aggregate;
use log::info;

#[derive(Debug, Clone)]
pub struct MedicationsAggregate {}

impl Aggregate<Option<MedicationsState>, MedicationsCommand, MedicationsEvent>
    for MedicationsAggregate
{
    fn init(&self) -> Option<MedicationsState> {
        None
    }

    fn apply(
        &self,
        state: Option<MedicationsState>,
        event: &MedicationsEvent,
    ) -> Option<MedicationsState> {
        match event {
            MedicationsEvent::MedicationCreated(u) => {
                info!("Applying MedicationCreated event");
                Some(MedicationsState::from(u.clone()))
            }

            MedicationsEvent::MedicationUpdated(u) => {
                info!("Applying MedicationUpdated event");
                Some(MedicationsState::from(u.clone()))
            }

            MedicationsEvent::MedicationDeleted(u) => {
                info!("Applying MedicationDeleted event");
                let state = state.unwrap_or_default();
                Some(MedicationsState {
                    id: String::from(&u.id),
                    org_id: String::from(&u.org_id),
                    patient_id: String::from(&u.patient_id),
                    is_deleted: true,
                    ..state // As we don't need to update all properties
                })
            }
        }
    }

    fn execute(
        &self,
        _state: &Option<MedicationsState>,
        command: &MedicationsCommand,
    ) -> Result<Vec<MedicationsEvent>> {
        match command {
            MedicationsCommand::CreateMedication(u) => {
                info!("Executing CreateMedication command");
                let input = medication_domain::Create::parse(u)?;
                Ok(vec![MedicationsEvent::MedicationCreated(
                    MedicationCreated::from(input),
                )])
            }
            MedicationsCommand::UpdateMedication(u) => {
                info!("Executing UpdateMedication command");
                let input = medication_domain::Update::parse(u)?;
                Ok(vec![MedicationsEvent::MedicationUpdated(
                    MedicationUpdated::from(input),
                )])
            }

            MedicationsCommand::DeleteMedication(u) => {
                info!("Executing DeleteMedication command");
                let input = medication_domain::Delete::parse(u)?;
                Ok(vec![MedicationsEvent::MedicationDeleted(
                    MedicationDeleted::from(input),
                )])
            }
        }
    }
}

pub const MEDICATIONS_AGGREGATE: MedicationsAggregate = MedicationsAggregate {};
