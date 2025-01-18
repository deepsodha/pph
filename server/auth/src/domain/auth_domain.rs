use super::{auth_commands::LoginWithMobileCommand, auth_events::LoggedInWithMobile};
use anyhow::Result;
use chrono::{DateTime, Utc};
use common::domain::mobile::ValidMobile;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogInState {
    pub mobile: String,
    pub password: String,
    pub time: DateTime<Utc>,
}

impl From<LoggedInWithMobile> for LogInState {
    fn from(u: LoggedInWithMobile) -> Self {
        LogInState {
            mobile: String::from(&u.mobile),
            password: String::from(&u.password),
            time: u.time,
        }
    }
}

#[derive(Debug, Clone)]
pub struct LogInWithMobile {
    pub mobile: ValidMobile,
    pub password: String,
}

impl LogInWithMobile {
    pub fn parse(a: LoginWithMobileCommand) -> Result<LogInWithMobile> {
        Ok(LogInWithMobile {
            mobile: ValidMobile::parse(&a.mobile)?,
            password: String::from(&a.password),
        })
    }
}
