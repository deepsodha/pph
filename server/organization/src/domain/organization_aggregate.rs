use super::{
    organization_commands::OrganizationCommand,
    organization_domain::{self, OrganizationState},
    organization_events::{
        OrganizationCreated, OrganizationDeleted, OrganizationEvent, OrganizationSelected,
        OrganizationUpdated,
    },
};
use anyhow::Result;
use cosmo_store_util::aggregate::Aggregate;
use log::info;

#[derive(Debug, Clone)]
pub struct OrganizationAggregate {}

impl Aggregate<Option<OrganizationState>, OrganizationCommand, OrganizationEvent>
    for OrganizationAggregate
{
    fn init(&self) -> Option<OrganizationState> {
        None
    }

    fn apply(
        &self,
        state: Option<OrganizationState>,
        event: &OrganizationEvent,
    ) -> Option<OrganizationState> {
        match event {
            OrganizationEvent::OrganizationCreated(u) => {
                info!("Applying OrganizationCreated event");
                Some(OrganizationState::from(u.clone()))
            }
            OrganizationEvent::OrganizationUpdated(u) => {
                info!("Applying OrganizationUpdated event");
                Some(OrganizationState::from(u.clone()))
            }
            OrganizationEvent::OrganizationDeleted(s) => {
                info!("Applying OrganizationDeleted event");
                let state = state.unwrap_or_default();
                Some(OrganizationState {
                    id: String::from(&s.id),
                    is_deleted: true,
                    ..state
                })
            }
            OrganizationEvent::OrganizationSelected(s) => {
                info!("Applying OrganizationSelected event");
                let state = state.unwrap_or_default();
                Some(OrganizationState {
                    id: String::from(&s.id),
                    name: String::from(&s.name),
                    ..state
                })
            }
        }
    }

    fn execute(
        &self,
        _state: &Option<OrganizationState>,
        command: &OrganizationCommand,
    ) -> Result<Vec<OrganizationEvent>> {
        match command {
            OrganizationCommand::CreateOrganization(u) => {
                info!("Executing CreateOrganization command");
                let input = organization_domain::Create::parse(u)?;
                Ok(vec![OrganizationEvent::OrganizationCreated(
                    OrganizationCreated::from(input),
                )])
            }
            OrganizationCommand::UpdateOrganization(u) => {
                info!("Executing UpdateOrganization command");
                let input = organization_domain::Update::parse(u)?;
                // TODO: Add update logic here.
                Ok(vec![OrganizationEvent::OrganizationUpdated(
                    OrganizationUpdated::from(input),
                )])
            }
            OrganizationCommand::DeleteOrganization(u) => {
                info!("Executing DeleteOrganization command");
                let input = organization_domain::Delete::parse(u)?;
                // TODO: Add delete logic here.
                Ok(vec![OrganizationEvent::OrganizationDeleted(
                    OrganizationDeleted::from(input),
                )])
            }
            OrganizationCommand::SelectOrganization(u) => {
                info!("Executing SelectOrganization command");
                let input = organization_domain::Select::parse(u)?;
                Ok(vec![OrganizationEvent::OrganizationSelected(
                    OrganizationSelected::from(input),
                )])
            }
        }
    }
}

pub const ORGANIZATION_AGGREGATE: OrganizationAggregate = OrganizationAggregate {};
