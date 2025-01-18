use super::{
    doctor_commands::DoctorCommand,
    doctor_domain::{self, DoctorState},
    doctor_events::{DoctorCreated, DoctorDeleted, DoctorEvent, DoctorUpdated},
};
use anyhow::Result;
use cosmo_store_util::aggregate::Aggregate;
use log::info;

#[derive(Debug, Clone)]
pub struct DoctorAggregate {}

impl Aggregate<Option<DoctorState>, DoctorCommand, DoctorEvent> for DoctorAggregate {
    fn init(&self) -> Option<DoctorState> {
        None
    }

    fn apply(&self, state: Option<DoctorState>, event: &DoctorEvent) -> Option<DoctorState> {
        match event {
            DoctorEvent::DoctorCreated(u) => {
                info!("Applying DoctorCreated event");
                Some(DoctorState::from(*u.clone()))
            }
            DoctorEvent::DoctorUpdated(u) => match state {
                None => None,
                Some(state) => {
                    info!("Applying DoctorUpdated event");
                    Some(DoctorState {
                        id: state.id.clone(),
                        user: u.user.clone(),
                        org_id: u.org_id.clone(),
                        created_by: u.created_by.clone(),
                        updated_by: u.updated_by.clone(),
                        created_at: u.created_at,
                        last_updated: u.last_updated,
                        is_deleted: false,
                        doctor_role: u.doctor_role.clone(),
                        doctor_register_number: u.doctor_register_number.clone(),
                        doctor_department: u.doctor_department.clone(),
                        doctor_speciality: u.doctor_speciality.clone(),
                        emergency: u.emergency,
                    })
                }
            },
            DoctorEvent::DoctorDeleted(s) => {
                info!("Applying DoctorDeleted event");
                let state = state.unwrap_or_default();
                Some(DoctorState {
                    id: String::from(&s.id),
                    org_id: String::from(&s.org_id),
                    is_deleted: true,
                    ..state // As we don't need to update all properties
                })
            }
        }
    }

    fn execute(
        &self,
        state: &Option<DoctorState>,
        command: &DoctorCommand,
    ) -> Result<Vec<DoctorEvent>> {
        match command {
            DoctorCommand::CreateDoctor(u) => {
                info!("Executing CreateDoctor command");
                let input = doctor_domain::Create::parse(u)?;
                Ok(vec![DoctorEvent::DoctorCreated(Box::new(
                    DoctorCreated::from(input),
                ))])
            }
            DoctorCommand::UpdateDoctor(u) => match state {
                None => Err(anyhow::anyhow!("Doctor not found")),
                Some(_state) => {
                    info!("Executing UpdateDoctor command");
                    let input = doctor_domain::Update::parse(u)?;
                    Ok(vec![DoctorEvent::DoctorUpdated(Box::new(
                        DoctorUpdated::from(input),
                    ))])
                }
            },
            DoctorCommand::DeleteDoctor(u) => {
                info!("Executing DeleteDoctor command");
                let input = doctor_domain::Delete::parse(u)?;
                Ok(vec![DoctorEvent::DoctorDeleted(DoctorDeleted::from(input))])
            }
        }
    }
}

pub const DOCTOR_AGGREGATE: DoctorAggregate = DoctorAggregate {};
