use anyhow::Result;
use cosmo_store_util::aggregate::Aggregate;
use log::info;

use super::{
    auth_commands::LoginCommand,
    auth_domain::{self, LogInState},
    auth_events::{LogInEvent, LoggedInWithMobile},
};

#[derive(Debug, Clone)]
pub struct LoginAggregate {}

impl Aggregate<Option<LogInState>, LoginCommand, LogInEvent> for LoginAggregate {
    fn init(&self) -> Option<LogInState> {
        None
    }

    fn apply(&self, _state: Option<LogInState>, event: &LogInEvent) -> Option<LogInState> {
        match event {
            LogInEvent::LoggedInWithMobile(u) => {
                info!("Applying LoggedInWithMobile event");
                Some(LogInState::from(u.clone()))
            }
        }
    }

    fn execute(
        &self,
        _state: &Option<LogInState>,
        command: &LoginCommand,
    ) -> Result<Vec<LogInEvent>> {
        match command {
            LoginCommand::LoginWithMobileCommand(u) => {
                info!("Executing LoginWithMobileCommand");
                let input = auth_domain::LogInWithMobile::parse(u.clone())?;
                Ok(vec![LogInEvent::LoggedInWithMobile(
                    LoggedInWithMobile::from(input),
                )])
            }
        }
    }
}

pub const LOGIN_AGGREGATE: LoginAggregate = LoginAggregate {};
