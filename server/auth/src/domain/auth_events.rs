use super::auth_domain::LogInWithMobile;
use chrono::{DateTime, Utc};
use cosmo_store::types::event_write::EventWrite;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LoggedInWithMobile {
    pub mobile: String,
    pub password: String,
    pub time: DateTime<Utc>,
}

impl From<LogInWithMobile> for LoggedInWithMobile {
    fn from(s: LogInWithMobile) -> Self {
        LoggedInWithMobile {
            mobile: String::from(s.mobile.as_ref()),
            password: s.password,
            time: Utc::now(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum LogInEvent {
    LoggedInWithMobile(LoggedInWithMobile),
}

impl From<LogInEvent> for EventWrite<LogInEvent, LogInEvent> {
    fn from(u: LogInEvent) -> Self {
        EventWrite {
            id: Default::default(),
            correlation_id: None,
            causation_id: None,
            name: String::from("login_event"),
            data: u,
            metadata: None,
        }
    }
}
