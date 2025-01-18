use anyhow::Result;
use cosmo_store_util::aggregate::Aggregate;
use log::info;

use super::{
    obandpregnancy_commands::OBandPregnancyCommand,
    obandpregnancy_domain::{self, OBandPregnancyState},
    obandpregnancy_events::{
        OBandPregnancyCreated, OBandPregnancyDeleted, OBandPregnancyEvent, OBandPregnancyUpdated,
    },
};

#[derive(Debug, Clone)]
pub struct OBAndPregnancyAggregate {}

impl Aggregate<Option<OBandPregnancyState>, OBandPregnancyCommand, OBandPregnancyEvent>
    for OBAndPregnancyAggregate
{
    fn init(&self) -> Option<OBandPregnancyState> {
        None
    }

    fn apply(
        &self,
        state: Option<OBandPregnancyState>,
        event: &OBandPregnancyEvent,
    ) -> Option<OBandPregnancyState> {
        match event {
            OBandPregnancyEvent::OBandPregnancyCreated(u) => {
                info!("Applying OBandPregnancyCreated event");
                Some(OBandPregnancyState::from(u.clone()))
            }

            OBandPregnancyEvent::OBandPregnancyUpdated(u) => {
                info!("Applying OBandPregnancyUpdated event");
                Some(OBandPregnancyState::from(u.clone()))
            }

            OBandPregnancyEvent::OBandPregnancyDeleted(u) => {
                info!("Applying OBandPregnancyDeleted event");
                let state: OBandPregnancyState = state.unwrap_or_default();
                Some(OBandPregnancyState {
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
        _state: &Option<OBandPregnancyState>,
        command: &OBandPregnancyCommand,
    ) -> Result<Vec<OBandPregnancyEvent>> {
        match command {
            OBandPregnancyCommand::CreateOBandPregnancy(u) => {
                info!("Executing CreateOBandPregnancy command");
                let obandpregnancy = obandpregnancy_domain::Create::parse(u)?;
                Ok(vec![OBandPregnancyEvent::OBandPregnancyCreated(
                    OBandPregnancyCreated::from(obandpregnancy),
                )])
            }
            OBandPregnancyCommand::UpdateOBandPregnancy(u) => {
                info!("Executing UpdateOBandPregnancy command");
                let obandpregnancy = obandpregnancy_domain::Update::parse(u)?;
                Ok(vec![OBandPregnancyEvent::OBandPregnancyUpdated(
                    OBandPregnancyUpdated::from(obandpregnancy),
                )])
            }

            OBandPregnancyCommand::DeleteOBandPregnancy(u) => {
                info!("Executing DeleteOBandPregnancy command");
                let obandpregnancy = obandpregnancy_domain::Delete::parse(u)?;
                Ok(vec![OBandPregnancyEvent::OBandPregnancyDeleted(
                    OBandPregnancyDeleted::from(obandpregnancy),
                )])
            }
        }
    }
}

pub const OBANDPREGNANCY_AGGREGATE: OBAndPregnancyAggregate = OBAndPregnancyAggregate {};
