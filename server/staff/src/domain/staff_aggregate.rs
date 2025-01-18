use anyhow::Result;
use cosmo_store_util::aggregate::Aggregate;
use log::info;

use crate::domain::{
    staff_domain,
    staff_events::{StaffCreated, StaffDeleted, StaffUpdated},
};

use super::{staff_commands::StaffCommand, staff_domain::StaffState, staff_events::StaffEvent};

#[derive(Debug, Clone)]
pub struct StaffAggregate {}

impl Aggregate<Option<StaffState>, StaffCommand, StaffEvent> for StaffAggregate {
    fn init(&self) -> Option<StaffState> {
        None
    }

    fn apply(&self, state: Option<StaffState>, event: &StaffEvent) -> Option<StaffState> {
        match event {
            StaffEvent::StaffCreated(u) => {
                info!("Applying StaffCreated event");
                Some(StaffState::from(*u.clone()))
            }
            StaffEvent::StaffUpdated(u) => match state {
                None => None,
                Some(state) => {
                    info!("Applying StaffUpdated event");
                    Some(StaffState {
                        id: state.id.clone(),
                        org_id: u.org_id.clone(),
                        created_by: u.created_by.clone(),
                        updated_by: u.updated_by.clone(),
                        created_at: u.created_at,
                        last_updated: u.last_updated,
                        is_deleted: false,
                        user: u.user.clone(),
                        staff_department: u.staff_department.clone(),
                        emergency: u.emergency,
                    })
                }
            },
            StaffEvent::StaffDeleted(s) => {
                info!("Applying StaffDeleted event");
                let state = state.unwrap_or_default();
                Some(StaffState {
                    id: String::from(&s.id),
                    org_id: String::from(&s.org_id),
                    is_deleted: true,
                    ..state // As we don't need to update all properties
                })
            }
        }
    }

    fn execute(
        &self,
        _state: &Option<StaffState>,
        command: &StaffCommand,
    ) -> Result<Vec<StaffEvent>> {
        match command {
            StaffCommand::CreateStaff(u) => {
                info!("Executing CreateStaff command");
                let input = staff_domain::Create::parse(u)?;
                Ok(vec![StaffEvent::StaffCreated(Box::new(
                    StaffCreated::from(input),
                ))])
            }
            StaffCommand::UpdateStaff(u) => {
                info!("Executing UpdateStaff command");
                let input = staff_domain::Update::parse(u)?;
                // TODO: Add update logic here.
                Ok(vec![StaffEvent::StaffUpdated(Box::new(
                    StaffUpdated::from(input),
                ))])
            }
            StaffCommand::DeleteStaff(u) => {
                info!("Executing DeleteStaff command");
                let input = staff_domain::Delete::parse(u)?;
                // TODO: Add delete logic here.
                Ok(vec![StaffEvent::StaffDeleted(StaffDeleted::from(input))])
            }
        }
    }
}

pub const STAFF_AGGREGATE: StaffAggregate = StaffAggregate {};
