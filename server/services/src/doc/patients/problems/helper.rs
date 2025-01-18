use anyhow::Result;
use cosmo_store::{common::i64_event_version::EventVersion, types::event_read::EventRead};
use cosmo_store_util::aggregate::Aggregate;
use db_services::services::problem::upsert_problem;
use log::error;
use patient::domain::problem::{
    problem_aggregate::PROBLEM_AGGREGATE, problem_domain::ProblemState,
    problem_events::ProblemEvent,
};
use sqlx::{Pool, Sqlite};

use crate::doc::syncs::DataTable;
pub async fn process_problems_events(
    read_pool: Pool<Sqlite>,
    problem_id: String,
    stream_id: String,
    read_events: Vec<EventRead<ProblemEvent, ProblemEvent, EventVersion>>,
) -> Result<()> {
    let problem_db =
        sqlx::query_as::<_, DataTable>("SELECT * from  problem_table_state WHERE id = ? LIMIT 1")
            .bind(problem_id.clone())
            .fetch_optional(&read_pool)
            .await?;

    let problem_state: Option<ProblemState> =
        problem_db.and_then(|data_table| serde_json::from_value(data_table.data).ok());

    let problem_updated_state = read_events
        .iter()
        .fold(problem_state, |a, b| PROBLEM_AGGREGATE.apply(a, &b.data));

    match problem_updated_state {
        Some(p) => {
            upsert_problem(
                read_pool,
                p,
                read_events
                    .last()
                    .map_or_else(|| 0, |event| event.version.0),
                stream_id,
            )
            .await?;
        }
        None => {
            error!("Problems with ID: {} not found", problem_id);
        }
    }
    Ok(())
}
