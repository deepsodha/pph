use anyhow::Result;
use cosmo_store_util::aggregate::Aggregate;
use log::info;

use crate::domain::notes::{note_domain, note_events::NoteCreated};

use super::{note_command::NoteCommand, note_domain::NoteState, note_events::NoteEvent};

#[derive(Debug, Clone)]
pub struct NoteAggregate {}

impl Aggregate<Option<NoteState>, NoteCommand, NoteEvent> for NoteAggregate {
    fn init(&self) -> Option<NoteState> {
        None
    }

    fn apply(&self, _state: Option<NoteState>, event: &NoteEvent) -> Option<NoteState> {
        match event {
            NoteEvent::NoteCreated(u) => {
                info!("Applying NoteCreated event");
                Some(NoteState::from(u.clone()))
            }
        }
    }

    fn execute(&self, _state: &Option<NoteState>, command: &NoteCommand) -> Result<Vec<NoteEvent>> {
        match command {
            NoteCommand::CreateNote(u) => {
                info!("Executing CreateNote command");
                let note = note_domain::Note::parse(u)?;
                Ok(vec![NoteEvent::NoteCreated(NoteCreated::from(note))])
            }
        }
    }
}

pub const NOTE_AGGREGATE: NoteAggregate = NoteAggregate {};
