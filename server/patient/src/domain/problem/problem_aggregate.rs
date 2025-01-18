use super::{
    problem_commands::ProblemCommand,
    problem_domain::{self, ProblemState},
    problem_events::{ProblemCreated, ProblemDeleted, ProblemEvent, ProblemUpdated},
};
use anyhow::Result;
use cosmo_store_util::aggregate::Aggregate;
use log::info;

#[derive(Debug, Clone)]
pub struct ProblemAggregate {}

impl Aggregate<Option<ProblemState>, ProblemCommand, ProblemEvent> for ProblemAggregate {
    fn init(&self) -> Option<ProblemState> {
        None
    }

    fn apply(&self, state: Option<ProblemState>, event: &ProblemEvent) -> Option<ProblemState> {
        match event {
            ProblemEvent::ProblemCreated(u) => {
                info!("Applying ProblemCreated event");
                Some(ProblemState::from(u.clone()))
            }
            ProblemEvent::ProblemUpdated(u) => {
                info!("Applying ProblemUpdated event");
                Some(ProblemState::from(u.clone()))
            }
            ProblemEvent::ProblemDeleted(u) => {
                info!("Applying ProblemDeleted event");
                let state = state.unwrap_or_default();
                Some(ProblemState {
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
        _state: &Option<ProblemState>,
        command: &ProblemCommand,
    ) -> Result<Vec<ProblemEvent>> {
        match command {
            ProblemCommand::CreateProblem(u) => {
                info!("Executing CreateProblem command");
                let problem = problem_domain::Create::parse(u)?;
                Ok(vec![ProblemEvent::ProblemCreated(ProblemCreated::from(
                    problem,
                ))])
            }
            ProblemCommand::UpdateProblem(u) => {
                info!("Executing UpdateProblem command");
                let problem = problem_domain::Update::parse(u)?;
                // TODO here update logic will come.
                Ok(vec![ProblemEvent::ProblemUpdated(ProblemUpdated::from(
                    problem,
                ))])
            }
            ProblemCommand::DeleteProblem(u) => {
                info!("Executing DeleteProblem command");
                let problem = problem_domain::Delete::parse(u)?;
                // TODO here update logic will come.
                Ok(vec![ProblemEvent::ProblemDeleted(ProblemDeleted::from(
                    problem,
                ))])
            }
        }
    }
}

pub const PROBLEM_AGGREGATE: ProblemAggregate = ProblemAggregate {};
