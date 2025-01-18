use anyhow::Result;
use cosmo_store_util::aggregate::Aggregate;
use log::info;

use crate::domain::{service_location_domain, service_location_events::ServiceLocationSelected};

use super::{
    service_location_commands::ServiceLocationCommand,
    service_location_domain::ServiceLocationState,
    service_location_events::{
        ServiceLocationCreated, ServiceLocationDeleted, ServiceLocationEvent,
        ServiceLocationUpdated,
    },
};

#[derive(Debug, Clone)]
pub struct ServiceLocationAggregate {}

impl Aggregate<Option<ServiceLocationState>, ServiceLocationCommand, ServiceLocationEvent>
    for ServiceLocationAggregate
{
    fn init(&self) -> Option<ServiceLocationState> {
        None
    }

    fn apply(
        &self,
        state: Option<ServiceLocationState>,
        event: &ServiceLocationEvent,
    ) -> Option<ServiceLocationState> {
        match event {
            ServiceLocationEvent::ServiceLocationCreated(u) => {
                info!("Applying ServiceLocationCreated event");
                Some(ServiceLocationState::from(u.clone()))
            }

            ServiceLocationEvent::ServiceLocationUpdated(s) => {
                info!("Applying ServiceLocationUpdated event");
                Some(ServiceLocationState::from(s.clone()))
            }

            ServiceLocationEvent::ServiceLocationDeleted(s) => {
                info!("Applying ServiceLocationDeleted event");
                let state = state.unwrap_or_default();
                Some(ServiceLocationState {
                    id: String::from(&s.id),
                    org_id: String::from(&s.org_id),
                    is_deleted: true,
                    ..state // As we don't need to update all properties
                })
            }
            ServiceLocationEvent::ServiceLocationSelected(s) => {
                info!("Applying ServiceLocationSelected event");
                let state = state.unwrap_or_default();
                Some(ServiceLocationState {
                    id: String::from(&s.id),
                    service_location_name: String::from(&s.name),
                    ..state
                })
            }
        }
    }

    fn execute(
        &self,
        _state: &Option<ServiceLocationState>,
        command: &ServiceLocationCommand,
    ) -> Result<Vec<ServiceLocationEvent>> {
        match command {
            ServiceLocationCommand::CreateSerivceLocation(u) => {
                info!("Executing CreateSerivceLocation command");
                let input = service_location_domain::Create::parse(u)?;
                Ok(vec![ServiceLocationEvent::ServiceLocationCreated(
                    ServiceLocationCreated::from(input),
                )])
            }
            ServiceLocationCommand::UpdateServiceLocation(u) => {
                info!("Executing UpdateServiceLocation command");
                let input = service_location_domain::Update::parse(u)?;
                // TODO here update logic will come.
                Ok(vec![ServiceLocationEvent::ServiceLocationUpdated(
                    ServiceLocationUpdated::from(input),
                )])
            }

            ServiceLocationCommand::DeleteServiceLocation(u) => {
                info!("Executing DeleteServiceLocation command");
                let input = service_location_domain::Delete::parse(u)?;
                // TODO here update logic will come.
                Ok(vec![ServiceLocationEvent::ServiceLocationDeleted(
                    ServiceLocationDeleted::from(input),
                )])
            }

            ServiceLocationCommand::ServiceLocationSelect(u) => {
                info!("Executing SelectServiceLocation command");
                let input = service_location_domain::Select::parse(u)?;
                Ok(vec![ServiceLocationEvent::ServiceLocationSelected(
                    ServiceLocationSelected::from(input),
                )])
            }
        }
    }
}

pub const SERVICELOCATION_AGGREGATE: ServiceLocationAggregate = ServiceLocationAggregate {};
