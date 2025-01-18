use super::{
    administer_commands::AdministerCommand,
    administer_domain::{self, AdministerState},
    administer_events::{AdministerCreated, AdministerDeleted, AdministerEvent, AdministerUpdated},
};
use anyhow::Result;
use cosmo_store_util::aggregate::Aggregate;
use log::info;

#[derive(Debug, Clone)]
pub struct AdministerAggregate {}

impl Aggregate<Option<AdministerState>, AdministerCommand, AdministerEvent>
    for AdministerAggregate
{
    fn init(&self) -> Option<AdministerState> {
        None
    }

    fn apply(
        &self,
        state: Option<AdministerState>,
        event: &AdministerEvent,
    ) -> Option<AdministerState> {
        match event {
            AdministerEvent::AdministerCreated(u) => {
                info!("Applying AdministerCreated event");
                Some(AdministerState::from(u.clone()))
            }

            AdministerEvent::AdministerUpdated(u) => {
                info!("Applying AdministerUpdated event");
                Some(AdministerState::from(u.clone()))
            }

            AdministerEvent::AdministerDeleted(u) => {
                info!("Applying AdministerDeleted event");
                let state = state.unwrap_or_default();
                Some(AdministerState {
                    id: String::from(&u.id),
                    org_id: String::from(&u.org_id),
                    patient_id: String::from(&u.patient_id),
                    is_deleted: true,
                    ..state // As we don't need to delete all properties
                })
            }
        }
    }

    fn execute(
        &self,
        _state: &Option<AdministerState>,
        command: &AdministerCommand,
    ) -> Result<Vec<AdministerEvent>> {
        match command {
            AdministerCommand::CreateAdminister(u) => {
                info!("Executing CreateAdminister command");
                let adm = administer_domain::Create::parse(u)?;
                Ok(vec![AdministerEvent::AdministerCreated(
                    AdministerCreated::from(adm),
                )])
            }
            AdministerCommand::UpdateAdminister(u) => {
                info!("Executing UpdateAdminister command");
                let adm = administer_domain::Update::parse(u)?;
                // TODO here update logic will come.
                Ok(vec![AdministerEvent::AdministerUpdated(
                    AdministerUpdated::from(adm),
                )])
            }

            AdministerCommand::DeleteAdminister(u) => {
                info!("Executing DeleteAdminister command");
                let adm = administer_domain::Delete::parse(u)?;
                // TODO here update logic will come.
                Ok(vec![AdministerEvent::AdministerDeleted(
                    AdministerDeleted::from(adm),
                )])
            }
        }
    }
}

pub const ADMINISTER_AGGREGATE: AdministerAggregate = AdministerAggregate {};
