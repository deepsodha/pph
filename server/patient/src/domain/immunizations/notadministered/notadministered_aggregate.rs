use super::{
    notadministered_commands::NotAdministerCommand,
    notadministered_domain::{self, NotAdministeredState},
    notadministered_events::{
        NotAdministerEvent, NotAdministeredCreated, NotAdministeredDeleted, NotAdministeredUpdated,
    },
};
use anyhow::Result;
use cosmo_store_util::aggregate::Aggregate;
use log::info;

#[derive(Debug, Clone)]
pub struct NotAdministerAggregate {}

impl Aggregate<Option<NotAdministeredState>, NotAdministerCommand, NotAdministerEvent>
    for NotAdministerAggregate
{
    fn init(&self) -> Option<NotAdministeredState> {
        None
    }

    fn apply(
        &self,
        state: Option<NotAdministeredState>,
        event: &NotAdministerEvent,
    ) -> Option<NotAdministeredState> {
        match event {
            NotAdministerEvent::NotAdministeredCreated(u) => {
                info!("Applying NotAdministeredCreated event");
                Some(NotAdministeredState::from(u.clone()))
            }

            NotAdministerEvent::NotAdministeredUpdated(u) => {
                info!("Applying NotAdministeredUpdated event");
                Some(NotAdministeredState::from(u.clone()))
            }

            NotAdministerEvent::NotAdministeredDeleted(u) => {
                info!("Applying NotAdministeredDeleted event");
                let state = state.unwrap_or_default();
                Some(NotAdministeredState {
                    id: String::from(&u.id),
                    patient_id: String::from(&u.patient_id),
                    org_id: String::from(&u.org_id),
                    is_deleted: true,
                    ..state // As we don't need to update all properties
                })
            }
        }
    }

    fn execute(
        &self,
        _state: &Option<NotAdministeredState>,
        command: &NotAdministerCommand,
    ) -> Result<Vec<NotAdministerEvent>> {
        match command {
            NotAdministerCommand::CreateNotAdministered(u) => {
                info!("Executing CreateNotAdministered command");
                let ndm = notadministered_domain::Create::parse(u)?;
                Ok(vec![NotAdministerEvent::NotAdministeredCreated(
                    NotAdministeredCreated::from(ndm),
                )])
            }
            NotAdministerCommand::UpdateNotAdministered(u) => {
                info!("Executing UpdateNotAdministered command");
                let ndm = notadministered_domain::Update::parse(u)?;
                // TODO here update logic will come.
                Ok(vec![NotAdministerEvent::NotAdministeredUpdated(
                    NotAdministeredUpdated::from(ndm),
                )])
            }

            NotAdministerCommand::DeleteNotAdministered(u) => {
                info!("Executing DeleteNotAdministered command");
                let ndm = notadministered_domain::Delete::parse(u)?;
                // TODO here update logic will come.
                Ok(vec![NotAdministerEvent::NotAdministeredDeleted(
                    NotAdministeredDeleted::from(ndm),
                )])
            }
        }
    }
}

pub const NOT_ADMINISTER_AGGREGATE: NotAdministerAggregate = NotAdministerAggregate {};
