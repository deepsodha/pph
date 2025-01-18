use super::{
    addhistorical_commands::HistoricalCommand,
    addhistorical_domain::{self, HistoricalState},
    addhistorical_events::{
        HistoricalCreated, HistoricalDeleted, HistoricalEvent, HistoricalUpdated,
    },
};
use anyhow::Result;
use cosmo_store_util::aggregate::Aggregate;
use log::info;

#[derive(Debug, Clone)]
pub struct HistoricalAggregate {}

impl Aggregate<Option<HistoricalState>, HistoricalCommand, HistoricalEvent>
    for HistoricalAggregate
{
    fn init(&self) -> Option<HistoricalState> {
        None
    }

    fn apply(
        &self,
        state: Option<HistoricalState>,
        event: &HistoricalEvent,
    ) -> Option<HistoricalState> {
        match event {
            HistoricalEvent::HistoricalCreated(u) => {
                info!("Applying HistoricalCreated event");
                Some(HistoricalState::from(u.clone()))
            }

            HistoricalEvent::HistoricalUpdated(u) => {
                info!("Applying HistoricalUpdated event");
                Some(HistoricalState::from(u.clone()))
            }

            HistoricalEvent::HistoricalDeleted(u) => {
                info!("Applying HistoricalDeleted event");
                let state = state.unwrap_or_default();
                Some(HistoricalState {
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
        _state: &Option<HistoricalState>,
        command: &HistoricalCommand,
    ) -> Result<Vec<HistoricalEvent>> {
        match command {
            HistoricalCommand::CreateHistorical(u) => {
                info!("Executing CreateHistorical command");
                let history = addhistorical_domain::Create::parse(u)?;
                Ok(vec![HistoricalEvent::HistoricalCreated(
                    HistoricalCreated::from(history),
                )])
            }
            HistoricalCommand::UpdateHistorical(u) => {
                info!("Executing UpdateHistorical command");
                let history = addhistorical_domain::Update::parse(u)?;
                // TODO here update logic will come.
                Ok(vec![HistoricalEvent::HistoricalUpdated(
                    HistoricalUpdated::from(history),
                )])
            }
            HistoricalCommand::DeleteHistorical(u) => {
                info!("Executing DeleteHistorical command");
                let history = addhistorical_domain::Delete::parse(u)?;
                // TODO here update logic will come.
                Ok(vec![HistoricalEvent::HistoricalDeleted(
                    HistoricalDeleted::from(history),
                )])
            }
        }
    }
}

pub const HISTORICAL_AGGREGATE: HistoricalAggregate = HistoricalAggregate {};
