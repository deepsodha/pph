use super::{
    patient_commands::PatientCommand,
    patient_domain::{self, PatientState},
    patient_events::PatientAddressUpdated,
};
use crate::domain::patient_events::{
    PatientContactDetailsUpdated, PatientCreated, PatientDeleted, PatientEvent,
    PatientGovDetailsUpdated, PatientUpdated, PatientUserDetailsUpdated,
};
use anyhow::Result;
use cosmo_store_util::aggregate::Aggregate;
use log::info;

#[derive(Debug, Clone)]
pub struct PatientAggregate {}

impl Aggregate<Option<PatientState>, PatientCommand, PatientEvent> for PatientAggregate {
    fn init(&self) -> Option<PatientState> {
        None
    }

    fn apply(&self, state: Option<PatientState>, event: &PatientEvent) -> Option<PatientState> {
        match event {
            PatientEvent::PatientCreated(u) => {
                info!("Applying PatientCreated event");
                Some(PatientState::from(*u.clone()))
            }
            PatientEvent::PatientUpdated(u) => match state {
                None => None,
                Some(state) => {
                    info!("Applying PatientUpdated event");
                    Some(PatientState {
                        id: state.id.clone(),
                        org_id: u.org_id.clone(),
                        created_by: u.created_by.clone(),
                        updated_by: u.updated_by.clone(),
                        created_at: u.created_at,
                        last_updated: u.last_updated,
                        is_deleted: false,
                        user: state.user.clone(),
                        address: state.address.clone(),
                        phone: state.phone.clone(),
                        government_info: state.government_info.clone(),
                    })
                }
            },
            PatientEvent::PatientAddressUpdated(u) => match state {
                None => None,
                Some(state) => {
                    info!("Applying PatientAddressUpdated event");
                    Some(PatientState {
                        id: state.id.clone(),
                        org_id: state.org_id.clone(),
                        created_by: u.created_by.clone(),
                        updated_by: u.updated_by.clone(),
                        created_at: u.created_at,
                        last_updated: u.last_updated,
                        user: state.user.clone(),
                        address: u.address.clone(),
                        phone: state.phone.clone(),
                        government_info: state.government_info.clone(),
                        is_deleted: false,
                    })
                }
            },
            PatientEvent::PatientDeleted(s) => {
                info!("Applying PatientDeleted event");
                let state = state.unwrap_or_default();
                Some(PatientState {
                    org_id: String::from(&s.org_id),
                    id: String::from(&s.id),
                    is_deleted: true,
                    ..state // As we don't need to update all properties
                })
            }
            PatientEvent::PatientUserDetailsUpdated(u) => match state {
                None => None,
                Some(state) => {
                    info!("Applying PatientUserDetailsUpdated event");
                    Some(PatientState {
                        id: state.id.clone(),
                        org_id: state.org_id.clone(),
                        created_by: u.created_by.clone(),
                        updated_by: u.updated_by.clone(),
                        created_at: u.created_at,
                        last_updated: u.last_updated,
                        user: u.user.clone(),
                        address: state.address.clone(),
                        phone: state.phone.clone(),
                        government_info: state.government_info.clone(),
                        is_deleted: false,
                    })
                }
            },
            PatientEvent::PatientContactDetailsUpdated(u) => match state {
                None => None,
                Some(state) => {
                    info!("Applying PatientContactDetailsUpdated event");
                    Some(PatientState {
                        id: state.id.clone(),
                        org_id: state.org_id.clone(),
                        created_by: u.created_by.clone(),
                        updated_by: u.updated_by.clone(),
                        created_at: u.created_at,
                        last_updated: u.last_updated,
                        user: state.user.clone(),
                        address: state.address.clone(),
                        phone: u.phone.clone(),
                        government_info: state.government_info.clone(),
                        is_deleted: false,
                    })
                }
            },
            PatientEvent::PatientGovDetailsUpdated(u) => match state {
                None => None,
                Some(state) => {
                    info!("Applying PatientGovDetailsUpdated event");
                    Some(PatientState {
                        id: state.id.clone(),
                        org_id: state.org_id.clone(),
                        created_by: u.created_by.clone(),
                        updated_by: u.updated_by.clone(),
                        created_at: u.created_at,
                        last_updated: u.last_updated,
                        user: state.user.clone(),
                        address: state.address.clone(),
                        phone: state.phone.clone(),
                        government_info: u.government_info.clone(),
                        is_deleted: false,
                    })
                }
            },
        }
    }

    fn execute(
        &self,
        state: &Option<PatientState>,
        command: &PatientCommand,
    ) -> Result<Vec<PatientEvent>> {
        match command {
            PatientCommand::CreatePatient(u) => {
                info!("Executing CreatePatient command");
                let input = patient_domain::Create::parse(u)?;
                Ok(vec![PatientEvent::PatientCreated(Box::new(
                    PatientCreated::from(input),
                ))])
            }
            PatientCommand::UpdatePatient(u) => match state {
                None => Err(anyhow::anyhow!("Patient not found")),
                Some(_state) => {
                    info!("Executing UpdatePatient command");
                    let input = patient_domain::Update::parse(u)?;
                    Ok(vec![PatientEvent::PatientUpdated(PatientUpdated::from(
                        input,
                    ))])
                }
            },
            PatientCommand::UpdatePatientAddress(u) => match state {
                None => Err(anyhow::anyhow!("Patient not found")),
                Some(state) => {
                    if u.address == state.address {
                        return Err(anyhow::anyhow!("Patient address not updated"));
                    }
                    info!("Executing UpdatePatientAddress command");
                    let input = patient_domain::PatientAddress::parse(u)?;
                    Ok(vec![PatientEvent::PatientAddressUpdated(
                        PatientAddressUpdated::from(input),
                    )])
                }
            },
            PatientCommand::UpdatePatientUserDetails(u) => match state {
                None => Err(anyhow::anyhow!("Patient not found")),
                Some(state) => {
                    if u.user == state.user {
                        return Err(anyhow::anyhow!("Patient User not updated"));
                    }
                    info!("Executing UpdatePatientUser command");
                    let input = patient_domain::PatientUser::parse(u)?;
                    Ok(vec![PatientEvent::PatientUserDetailsUpdated(Box::new(
                        PatientUserDetailsUpdated::from(input),
                    ))])
                }
            },
            PatientCommand::UpdatePatientContactDetails(u) => match state {
                None => Err(anyhow::anyhow!("Patient not found")),
                Some(state) => {
                    if u.phone == state.phone {
                        return Err(anyhow::anyhow!("Patient Contact not updated"));
                    }
                    info!("Executing UpdatePatientContact command");
                    let input = patient_domain::PatientPhone::parse(u)?;
                    Ok(vec![PatientEvent::PatientContactDetailsUpdated(
                        PatientContactDetailsUpdated::from(input),
                    )])
                }
            },
            PatientCommand::UpdatePatientGovDetails(u) => match state {
                None => Err(anyhow::anyhow!("Patient not found")),
                Some(state) => {
                    if u.government_info == state.government_info {
                        return Err(anyhow::anyhow!("Patient GovInfo not updated"));
                    }
                    info!("Executing UpdatePatientGovInfo command");
                    let input = patient_domain::PatientGov::parse(u)?;
                    Ok(vec![PatientEvent::PatientGovDetailsUpdated(
                        PatientGovDetailsUpdated::from(input),
                    )])
                }
            },
            PatientCommand::DeletePatient(u) => {
                info!("Executing DeletePatient command");
                let input = patient_domain::Delete::parse(u)?;
                Ok(vec![PatientEvent::PatientDeleted(PatientDeleted::from(
                    input,
                ))])
            }
        }
    }
}

pub const PATIENT_AGGREGATE: PatientAggregate = PatientAggregate {};
