use super::{user_commands::UserCommand, user_domain::UserState, user_events::UserEvent};
use crate::domain::{
    user_domain,
    user_events::{
        UserAddressUpdated, UserContactUpdated, UserCreated, UserDeleted, UserDetailsUpdated,
        UserGovDetailsUpdated, UserPasswordUpdated,
    },
};
use anyhow::Result;
use cosmo_store_util::aggregate::Aggregate;
use log::info;

#[derive(Debug, Clone)]
pub struct UserAggregate {}

impl Aggregate<Option<UserState>, UserCommand, UserEvent> for UserAggregate {
    fn init(&self) -> Option<UserState> {
        None
    }

    fn apply(&self, state: Option<UserState>, event: &UserEvent) -> Option<UserState> {
        match event {
            UserEvent::UserCreated(u) => {
                info!("Applying UserCreated event");
                Some(UserState::from(*u.clone()))
            }

            UserEvent::UserPasswordUpdated(u) => match state {
                None => None,
                Some(state) => Some(UserState {
                    id: u.id.clone(),
                    user: state.user,
                    address: state.address,
                    phone: state.phone,
                    government_info: state.government_info,
                    password: u.password.clone(),
                    is_deleted: false,
                    org_id: state.org_id.clone(),
                    created_by: u.created_by.clone(),
                    updated_by: u.updated_by.clone(),
                    created_at: u.created_at,
                    last_updated: u.last_updated,
                }),
            },

            UserEvent::UserContactUpdated(u) => match state {
                None => None,
                Some(state) => Some(UserState {
                    id: state.id,
                    user: state.user,
                    address: state.address,
                    phone: u.phone.clone(),
                    government_info: state.government_info,
                    password: state.password.clone(),
                    is_deleted: false,
                    org_id: state.org_id.clone(),
                    created_by: u.created_by.clone(),
                    updated_by: u.updated_by.clone(),
                    created_at: u.created_at,
                    last_updated: u.last_updated,
                }),
            },

            UserEvent::UserAddressUpdated(u) => match state {
                None => None,
                Some(state) => Some(UserState {
                    id: state.id,
                    user: state.user,
                    address: u.address.clone(),
                    phone: state.phone.clone(),
                    government_info: state.government_info,
                    password: state.password.clone(),
                    org_id: state.org_id.clone(),
                    created_by: u.created_by.clone(),
                    updated_by: u.updated_by.clone(),
                    created_at: u.created_at,
                    last_updated: u.last_updated,
                    is_deleted: false,
                }),
            },

            UserEvent::UserGovDetailsUpdated(u) => match state {
                None => None,
                Some(state) => Some(UserState {
                    id: state.id,
                    user: state.user,
                    address: state.address.clone(),
                    phone: state.phone.clone(),
                    government_info: u.government_info.clone(),
                    password: state.password.clone(),
                    org_id: state.org_id.clone(),
                    created_by: u.created_by.clone(),
                    updated_by: u.updated_by.clone(),
                    created_at: u.created_at,
                    last_updated: u.last_updated,
                    is_deleted: false,
                }),
            },

            UserEvent::UserDetailsUpdated(u) => match state {
                None => None,
                Some(state) => Some(UserState {
                    id: state.id,
                    user: u.user.clone(),
                    address: state.address.clone(),
                    phone: state.phone.clone(),
                    government_info: state.government_info.clone(),
                    password: state.password.clone(),
                    org_id: state.org_id.clone(),
                    created_by: u.created_by.clone(),
                    updated_by: u.updated_by.clone(),
                    created_at: u.created_at,
                    last_updated: u.last_updated,
                    is_deleted: false,
                }),
            },

            UserEvent::UserDeleted(s) => {
                info!("Applying UserDeleted event");
                let state = state.unwrap_or_default();
                Some(UserState {
                    id: String::from(&s.id),
                    is_deleted: true,
                    ..state
                })
            }
        }
    }

    fn execute(&self, _state: &Option<UserState>, command: &UserCommand) -> Result<Vec<UserEvent>> {
        match command {
            UserCommand::CreateUser(u) => {
                info!("Executing CreateUser command");
                let input = user_domain::Create::parse(u)?;
                Ok(vec![UserEvent::UserCreated(Box::new(UserCreated::from(
                    input,
                )))])
            }

            UserCommand::UpdateUserContact(u) => {
                info!("Executing UpdateUserContact command");
                let input = user_domain::UserContactUpdate::parse(u)?;

                Ok(vec![UserEvent::UserContactUpdated(
                    UserContactUpdated::from(input),
                )])
            }

            UserCommand::UpdateUserPassword(u) => {
                info!("Executing UpdateUserPassword command");
                let input = user_domain::UserPasswordUpdate::parse(u)?;

                Ok(vec![UserEvent::UserPasswordUpdated(
                    UserPasswordUpdated::from(input),
                )])
            }
            UserCommand::DeleteUser(u) => {
                info!("Executing DeleteUser command");
                let input = user_domain::Delete::parse(u)?;

                Ok(vec![UserEvent::UserDeleted(UserDeleted::from(input))])
            }
            UserCommand::UpdateUserAddress(u) => {
                info!("Executing UpdateUserAddress command");
                let input = user_domain::UserAddressUpdate::parse(u)?;

                Ok(vec![UserEvent::UserAddressUpdated(
                    UserAddressUpdated::from(input),
                )])
            }
            UserCommand::UpdateUserGovDetails(u) => {
                info!("Executing UpdateUserGovDetails command");
                let input = user_domain::UserGovUpdate::parse(u)?;

                Ok(vec![UserEvent::UserGovDetailsUpdated(
                    UserGovDetailsUpdated::from(input),
                )])
            }
            UserCommand::UpdateUserDetails(u) => {
                info!("Executing UpdateUserDetails command");
                let input = user_domain::UserDetailsUpdate::parse(u)?;

                Ok(vec![UserEvent::UserDetailsUpdated(
                    UserDetailsUpdated::from(input),
                )])
            }
        }
    }
}

pub const USER_AGGREGATE: UserAggregate = UserAggregate {};
