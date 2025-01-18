use anyhow::Result;
use cosmo_store_util::aggregate::Aggregate;
use log::info;

use super::{
    implantable_devices_commands::ImplantableDevicesCommand,
    implantable_devices_domain::{self, ImplantableDevicesState},
    implantable_devices_events::{
        ImplantableDevicesCreated, ImplantableDevicesDeleted, ImplantableDevicesEvent,
        ImplantableDevicesUpdated,
    },
};

#[derive(Debug, Clone)]
pub struct ImplantableDevicesAggregate {}

impl Aggregate<Option<ImplantableDevicesState>, ImplantableDevicesCommand, ImplantableDevicesEvent>
    for ImplantableDevicesAggregate
{
    fn init(&self) -> Option<ImplantableDevicesState> {
        None
    }

    fn apply(
        &self,
        state: Option<ImplantableDevicesState>,
        event: &ImplantableDevicesEvent,
    ) -> Option<ImplantableDevicesState> {
        match event {
            ImplantableDevicesEvent::ImplantableDevicesCreated(u) => {
                info!("Applying ImplantableDevicesCreated event");
                Some(ImplantableDevicesState::from(u.clone()))
            }

            ImplantableDevicesEvent::ImplantableDevicesUpdated(u) => {
                info!("Applying ImplantableDevicesUpdated event");
                Some(ImplantableDevicesState::from(u.clone()))
            }

            ImplantableDevicesEvent::ImplantableDevicesDeleted(u) => {
                info!("Applying ImplantableDevicesDeleted event");
                let state: ImplantableDevicesState = state.unwrap_or_default();
                Some(ImplantableDevicesState {
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
        _state: &Option<ImplantableDevicesState>,
        command: &ImplantableDevicesCommand,
    ) -> Result<Vec<ImplantableDevicesEvent>> {
        match command {
            ImplantableDevicesCommand::CreateImplantableDevices(u) => {
                info!("Executing CreateImplantableDevices command");
                let implantabledevices = implantable_devices_domain::Create::parse(u)?;
                Ok(vec![ImplantableDevicesEvent::ImplantableDevicesCreated(
                    ImplantableDevicesCreated::from(implantabledevices),
                )])
            }
            ImplantableDevicesCommand::UpdateImplantableDevices(u) => {
                info!("Executing UpdateImplantableDevices command");
                let implantabledevices = implantable_devices_domain::Update::parse(u)?;
                // TODO here update logic will come.
                Ok(vec![ImplantableDevicesEvent::ImplantableDevicesUpdated(
                    ImplantableDevicesUpdated::from(implantabledevices),
                )])
            }

            ImplantableDevicesCommand::DeleteImplantableDevices(u) => {
                info!("Executing DeleteImplantableDevices command");
                let implantabledevices = implantable_devices_domain::Delete::parse(u)?;
                // TODO here update logic will come.
                Ok(vec![ImplantableDevicesEvent::ImplantableDevicesDeleted(
                    ImplantableDevicesDeleted::from(implantabledevices),
                )])
            }
        }
    }
}

pub const IMPLANTABLEDEVICES_AGGREGATE: ImplantableDevicesAggregate =
    ImplantableDevicesAggregate {};
