use anyhow::Result;
use cosmo_store_util::aggregate::Aggregate;
use log::info;

use super::{
    social_history_commands::SocialHistoryCommand,
    social_history_domain::{self, SocialHistoryState},
    social_history_events::{
        SocialHistoryCreated, SocialHistoryDeleted, SocialHistoryEvent, SocialHistoryUpdated,
    },
};

#[derive(Debug, Clone)]
pub struct SocialHistoryAggregate {}

impl Aggregate<Option<SocialHistoryState>, SocialHistoryCommand, SocialHistoryEvent>
    for SocialHistoryAggregate
{
    fn init(&self) -> Option<SocialHistoryState> {
        None
    }

    fn apply(
        &self,
        state: Option<SocialHistoryState>,
        event: &SocialHistoryEvent,
    ) -> Option<SocialHistoryState> {
        match event {
            SocialHistoryEvent::SocialHistoryCreated(u) => {
                info!("Applying SocialHistoryCreated event");
                Some(SocialHistoryState::from(u.clone()))
            }

            SocialHistoryEvent::SocialHistoryUpdated(u) => {
                info!("Applying SocialHistoryUpdated event");
                Some(SocialHistoryState::from(u.clone()))
            }

            SocialHistoryEvent::SocialHistoryDeleted(u) => {
                info!("Applying SocialHistoryDeleted event");
                let state: SocialHistoryState = state.unwrap_or_default();
                Some(SocialHistoryState {
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
        _state: &Option<SocialHistoryState>,
        command: &SocialHistoryCommand,
    ) -> Result<Vec<SocialHistoryEvent>> {
        match command {
            SocialHistoryCommand::CreateSocialHistory(u) => {
                info!("Executing CreateSocialHistory command");
                let socialhistory = social_history_domain::Create::parse(u)?;
                Ok(vec![SocialHistoryEvent::SocialHistoryCreated(
                    SocialHistoryCreated::from(socialhistory),
                )])
            }
            SocialHistoryCommand::UpdateSocialHistory(u) => {
                info!("Executing UpdateSocialHistory command");
                let socialhistory = social_history_domain::Update::parse(u)?;
                Ok(vec![SocialHistoryEvent::SocialHistoryUpdated(
                    SocialHistoryUpdated::from(socialhistory),
                )])
            }

            SocialHistoryCommand::DeleteSocialHistory(u) => {
                info!("Executing DeleteSocialHistory command");
                let socialhistory = social_history_domain::Delete::parse(u)?;
                Ok(vec![SocialHistoryEvent::SocialHistoryDeleted(
                    SocialHistoryDeleted::from(socialhistory),
                )])
            }
        }
    }
}

pub const SOCIALHISTORY_AGGREGATE: SocialHistoryAggregate = SocialHistoryAggregate {};
