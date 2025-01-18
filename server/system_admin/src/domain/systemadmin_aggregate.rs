use anyhow::Result;
use cosmo_store_util::aggregate::Aggregate;
use log::info;

use crate::domain::systemadmin_events::{
    SystemAdminContactDetailsUpdated, SystemAdminGovDetailsUpdated, SystemAdminUserDetailsUpdated,
};

use super::{
    systemadmin_commands::SystemAdminCommands,
    systemadmin_domain::{self, SystemAdminState},
    systemadmin_events::{
        SystemAdminCreated, SystemAdminDeleted, SystemAdminEvent, SystemAdminUpdated,
    },
};

#[derive(Debug, Clone)]
pub struct SystemAdminAggregate {}

impl Aggregate<Option<SystemAdminState>, SystemAdminCommands, SystemAdminEvent>
    for SystemAdminAggregate
{
    fn init(&self) -> Option<SystemAdminState> {
        None
    }

    fn apply(
        &self,
        state: Option<SystemAdminState>,
        event: &SystemAdminEvent,
    ) -> Option<SystemAdminState> {
        match event {
            SystemAdminEvent::SystemAdminCreated(u) => {
                info!("Applying SystemAdminCreated event");
                Some(SystemAdminState::from(*u.clone()))
            }

            SystemAdminEvent::SystemAdminUpdated(u) => match state {
                None => None,
                Some(state) => {
                    info!("Applying SystemAdminUpdated event");
                    Some(SystemAdminState {
                        id: state.id.clone(),
                        org_id: u.org_id.clone(),
                        created_by: u.created_by.clone(),
                        updated_by: u.updated_by.clone(),
                        created_at: u.created_at,
                        last_updated: u.last_updated,
                        is_deleted: false,
                        user: state.user.clone(),
                        phone: state.phone.clone(),
                        government_info: state.government_info.clone(),
                        roles: u.roles.clone(),
                        password: u.password.clone(),
                    })
                }
            },

            SystemAdminEvent::SystemAdminDeleted(s) => {
                info!("Applying SystemAdminDeleted event");
                let state = state.unwrap_or_default();
                Some(SystemAdminState {
                    id: String::from(&s.id),
                    is_deleted: true,
                    ..state // As we don't need to update all properties
                })
            }
            SystemAdminEvent::SystemAdminUserDetailsUpdated(u) => match state {
                None => None,
                Some(state) => {
                    info!("Applying SystemAdminUserDetailsUpdated event");
                    Some(SystemAdminState {
                        id: state.id.clone(),
                        org_id: state.org_id.clone(),
                        created_by: u.created_by.clone(),
                        updated_by: u.updated_by.clone(),
                        created_at: u.created_at,
                        last_updated: u.last_updated,
                        user: u.user.clone(),
                        phone: state.phone.clone(),
                        government_info: state.government_info.clone(),
                        is_deleted: false,
                        roles: state.roles.clone(),
                        password: state.password.clone(),
                    })
                }
            },
            SystemAdminEvent::SystemAdminContactDetailsUpdated(u) => match state {
                None => None,
                Some(state) => {
                    info!("Applying SystemAdminContactDetailsUpdated event");
                    Some(SystemAdminState {
                        id: state.id.clone(),
                        org_id: state.org_id.clone(),
                        created_by: u.created_by.clone(),
                        updated_by: u.updated_by.clone(),
                        created_at: u.created_at,
                        last_updated: u.last_updated,
                        user: state.user.clone(),
                        phone: u.phone.clone(),
                        government_info: state.government_info.clone(),
                        is_deleted: false,
                        roles: state.roles.clone(),
                        password: state.password.clone(),
                    })
                }
            },
            SystemAdminEvent::SystemAdminGovDetailsUpdated(u) => match state {
                None => None,
                Some(state) => {
                    info!("Applying SystemAdminGovDetailsUpdated event");
                    Some(SystemAdminState {
                        id: state.id.clone(),
                        org_id: state.org_id.clone(),
                        created_by: u.created_by.clone(),
                        updated_by: u.updated_by.clone(),
                        created_at: u.created_at,
                        last_updated: u.last_updated,
                        user: state.user.clone(),
                        phone: state.phone.clone(),
                        government_info: u.government_info.clone(),
                        is_deleted: false,
                        roles: state.roles.clone(),
                        password: state.password.clone(),
                    })
                }
            },
        }
    }

    fn execute(
        &self,
        state: &Option<SystemAdminState>,
        command: &SystemAdminCommands,
    ) -> Result<Vec<SystemAdminEvent>> {
        match command {
            SystemAdminCommands::CreateSystemAdmin(u) => {
                info!("Executing CreateSystemAdmin command");
                let input = systemadmin_domain::Create::parse(u)?;
                Ok(vec![SystemAdminEvent::SystemAdminCreated(Box::new(
                    SystemAdminCreated::from(input),
                ))])
            }
            SystemAdminCommands::UpdateSystemAdmin(u) => match state {
                None => Err(anyhow::anyhow!("SystemAdmin not found")),
                Some(_state) => {
                    info!("Executing UpdateSystemAdmin command");
                    let input = systemadmin_domain::Update::parse(u)?;
                    Ok(vec![SystemAdminEvent::SystemAdminUpdated(
                        SystemAdminUpdated::from(input),
                    )])
                }
            },

            SystemAdminCommands::DeleteSystemAdmin(u) => {
                info!("Executing DeleteSystemAdmin command");
                let input = systemadmin_domain::Delete::parse(u)?;
                // TODO here update logic will come.
                Ok(vec![SystemAdminEvent::SystemAdminDeleted(
                    SystemAdminDeleted::from(input),
                )])
            }
            SystemAdminCommands::UpdateSystemAdminUserDetails(u) => match state {
                None => Err(anyhow::anyhow!("SystemAdmin not found")),
                Some(state) => {
                    if u.user == state.user {
                        return Err(anyhow::anyhow!("SystemAdmin User not updated"));
                    }
                    info!("Executing UpdateSystemAdminUser command");
                    let input = systemadmin_domain::SystemAdminUser::parse(u)?;
                    Ok(vec![SystemAdminEvent::SystemAdminUserDetailsUpdated(
                        Box::new(SystemAdminUserDetailsUpdated::from(input)),
                    )])
                }
            },
            SystemAdminCommands::UpdateSystemAdminContactDetails(u) => match state {
                None => Err(anyhow::anyhow!("SystemAdmin not found")),
                Some(state) => {
                    if u.phone == state.phone {
                        return Err(anyhow::anyhow!("SystemAdmin Contact not updated"));
                    }
                    info!("Executing UpdateSystemAdminContact command");
                    let input = systemadmin_domain::SystemAdminPhone::parse(u)?;
                    Ok(vec![SystemAdminEvent::SystemAdminContactDetailsUpdated(
                        SystemAdminContactDetailsUpdated::from(input),
                    )])
                }
            },
            SystemAdminCommands::UpdateSystemAdminGovDetails(u) => match state {
                None => Err(anyhow::anyhow!("SystemAdmin not found")),
                Some(state) => {
                    if u.government_info == state.government_info {
                        return Err(anyhow::anyhow!("SystemAdmin GovInfo not updated"));
                    }
                    info!("Executing UpdateSystemAdminGovInfo command");
                    let input = systemadmin_domain::SystemAdminGov::parse(u)?;
                    Ok(vec![SystemAdminEvent::SystemAdminGovDetailsUpdated(
                        SystemAdminGovDetailsUpdated::from(input),
                    )])
                }
            },
        }
    }
}

pub const SYSTEMADMIN_AGGREGATE: SystemAdminAggregate = SystemAdminAggregate {};
