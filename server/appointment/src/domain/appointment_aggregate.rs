use super::{
    appointment_commands::AppointmentCommands,
    appointment_domain::{self, AppointmentState},
    appointment_events::{
        AppointmentCreated, AppointmentDeleted, AppointmentEvent, AppointmentUpdated,
    },
};
use anyhow::Result;
use cosmo_store_util::aggregate::Aggregate;
use log::info;

#[derive(Debug, Clone)]
pub struct AppointmentAggregate {}

impl Aggregate<Option<AppointmentState>, AppointmentCommands, AppointmentEvent>
    for AppointmentAggregate
{
    fn init(&self) -> Option<AppointmentState> {
        None
    }

    fn apply(
        &self,
        state: Option<AppointmentState>,
        event: &AppointmentEvent,
    ) -> Option<AppointmentState> {
        match event {
            AppointmentEvent::AppointmentCreated(u) => {
                info!("Applying AppointmentCreated event");
                Some(AppointmentState::from(u.clone()))
            }
            AppointmentEvent::AppointmentUpdated(u) => {
                info!("Applying AppointmentUpdated event");
                Some(AppointmentState::from(u.clone()))
            }
            AppointmentEvent::AppointmentDeleted(u) => {
                info!("Applying AppointmentDeleted event");
                let state = state.unwrap_or_default();
                Some(AppointmentState {
                    org_id: String::from(&u.org_id),
                    id: String::from(&u.id),
                    is_deleted: true,
                    ..state // As we don't need to update all properties
                })
            }
        }
    }

    fn execute(
        &self,
        _state: &Option<AppointmentState>,
        command: &AppointmentCommands,
    ) -> Result<Vec<AppointmentEvent>> {
        match command {
            AppointmentCommands::CreateAppointment(u) => {
                info!("Executing CreateAppointment command");
                let data = appointment_domain::Create::parse(u)?;
                Ok(vec![AppointmentEvent::AppointmentCreated(
                    AppointmentCreated::from(data),
                )])
            }
            AppointmentCommands::UpdateAppointment(u) => {
                info!("Executing UpdateAppointment command");
                let data = appointment_domain::Update::parse(u)?;
                Ok(vec![AppointmentEvent::AppointmentUpdated(
                    AppointmentUpdated::from(data),
                )])
            }
            AppointmentCommands::DeleteAppointment(u) => {
                info!("Executing DeleteAppointment command");
                let data = appointment_domain::Delete::parse(u)?;
                Ok(vec![AppointmentEvent::AppointmentDeleted(
                    AppointmentDeleted::from(data),
                )])
            }
        }
    }
}

pub const APPOINTMENTS_AGGREGATE: AppointmentAggregate = AppointmentAggregate {};
