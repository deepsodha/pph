use crate::app_state::AppState;
use crate::doc::patients::get_patient_id::patient_id_fetch;
use crate::doc::syncs::sync;
use crate::method::convention::ErrorData;
use anyhow::Result;
use common::stream::Stream;
use cosmo_store::types::{event_read_range::EventsReadRange, expected_version::ExpectedVersion};
use cosmo_store_util::aggregate::make_handler;
use log::{error, info};
use patient::domain::problem::problem_commands::DeleteProblem;
use patient::domain::problem::{
    problem_aggregate::PROBLEM_AGGREGATE,
    problem_commands::{CreateProblem, ProblemCommand, UpdateProblem},
};
use patient::dto::problem::{
    problem_add::ProblemsAdd, problem_delete::ProblemsDelete, problem_update::ProblemsUpdate,
};
use serde_json::Value;
use sqlx::types::chrono::Utc;
use utils::store_helper::patient_store;
use uuid::Uuid;

use super::helper::process_problems_events;

pub async fn add_problem(
    params: ProblemsAdd,
    app_state: AppState,
    organization_id: String,
    user_id: String,
) -> Result<Value, ErrorData> {
    let patient_id = patient_id_fetch(app_state.clone(), params.patient_id.clone()).await?;
    if patient_id {
        info!("Adding problem...");

        let store = patient_store(&app_state.write_pool, &organization_id).await?;
        let stream_id = format!("Problems::{}", Uuid::new_v4().as_simple());
        let command = CreateProblem {
            id: Uuid::new_v4().as_simple().to_string(),
            org_id: organization_id.clone(),
            patient_id: params.patient_id,
            created_by: user_id.clone(),
            updated_by: user_id.clone(),
            created_at: Utc::now(),
            last_updated: Utc::now(),
            status: params.status,
            issue: params.issue,
            icd_10_problem: params.icd_10_problem,
            issue_type: params.issue_type,
            start_date: params.start_date,
            end_date: params.end_date,
            comment: params.comment,
        };
        let events = make_handler(
            &PROBLEM_AGGREGATE,
            &store,
            &ProblemCommand::CreateProblem(command.clone()),
            &stream_id,
            &EventsReadRange::AllEvents,
            &ExpectedVersion::Any,
        )
        .await?;
        let res =
            process_problems_events(app_state.read_pool.clone(), command.id, stream_id, events)
                .await;

        let row = sync(
            params.last_updated_input,
            organization_id.clone(),
            app_state.read_pool.clone(),
        )
        .await?;

        match res {
            anyhow::Result::Ok(_) => {
                info!("Problem added successfully.");
                serde_json::to_value(row).map_err(ErrorData::from)
            }
            Err(err) => {
                error!("{:#?}", err);
                error!("Failed to add problem.");
                Err(ErrorData {
                    message: String::from("PROBLEM_NOT_ADDED"),
                    data: Value::Null,
                    code: -32600,
                })
            }
        }
    } else {
        Err(ErrorData {
            message: String::from("PATIENT_NOT_FOUND"),
            data: Value::Null,
            code: -32600,
        })
    }
}

pub async fn update_problem(
    params: ProblemsUpdate,
    app_state: AppState,
    organization_id: String,
    user_id: String,
) -> Result<Value, ErrorData> {
    let patient_id = patient_id_fetch(app_state.clone(), params.input.patient_id.clone()).await?;
    if patient_id {
        info!("Updating problem...");

        let store = patient_store(&app_state.write_pool, &organization_id).await?;
        let problem_db = sqlx::query_as::<_, Stream>(
            "SELECT id,stream_id,version,(data->>'created_by') AS created_by, (data->>'created_at') AS created_at,data FROM problem_table_state WHERE id = ?",
        )
        .bind(params.id.clone())
        .fetch_optional(&app_state.read_pool.clone())
        .await?;

        let created_by = problem_db
            .as_ref()
            .map(|org| org.created_by.clone())
            .unwrap_or_default();
        let stream_id = problem_db
            .as_ref()
            .map(|org| org.stream_id.clone())
            .unwrap_or_default();
        let created_at = problem_db
            .as_ref()
            .map(|org| org.created_at)
            .unwrap_or_default();
        let command = UpdateProblem {
            id: params.id,
            created_by,
            updated_by: user_id,
            created_at,
            last_updated: Utc::now(),
            org_id: organization_id.clone(),
            patient_id: params.input.patient_id,
            status: params.input.status,
            issue: params.input.issue,
            icd_10_problem: params.input.icd_10_problem,
            issue_type: params.input.issue_type,
            start_date: params.input.start_date,
            end_date: params.input.end_date,
            comment: params.input.comment,
        };
        let events = make_handler(
            &PROBLEM_AGGREGATE,
            &store,
            &ProblemCommand::UpdateProblem(command.clone()),
            &stream_id,
            &EventsReadRange::AllEvents,
            &ExpectedVersion::Any,
        )
        .await?;
        let res =
            process_problems_events(app_state.read_pool.clone(), command.id, stream_id, events)
                .await;

        let row = sync(
            params.input.last_updated_input,
            organization_id.clone(),
            app_state.read_pool.clone(),
        )
        .await?;

        match res {
            anyhow::Result::Ok(_) => {
                info!("Problem updated successfully.");
                serde_json::to_value(row).map_err(ErrorData::from)
            }
            Err(err) => {
                error!("{:#?}", err);
                error!("Failed to update problem.");
                Err(ErrorData {
                    message: String::from("PROBLEM_NOT_UPDATED"),
                    data: Value::Null,
                    code: -32600,
                })
            }
        }
    } else {
        Err(ErrorData {
            message: String::from("PATIENT_NOT_FOUND"),
            data: Value::Null,
            code: -32600,
        })
    }
}

pub async fn delete_problem(
    params: ProblemsDelete,
    app_state: AppState,
    organization_id: String,
    user_id: String,
) -> Result<Value, ErrorData> {
    let patient_id = patient_id_fetch(app_state.clone(), params.patient_id.clone()).await?;
    if patient_id {
        info!("Deleting problem...");

        let store = patient_store(&app_state.write_pool, &organization_id).await?;
        let problem_db = sqlx::query_as::<_, Stream>(
        "SELECT id,stream_id,version,(data->>'created_by') AS created_by, (data->>'created_at') AS created_at,data FROM problem_table_state WHERE id = ?",
    )
    .bind(params.id.clone())
    .fetch_optional(&app_state.read_pool.clone())
    .await?;

        let created_by = problem_db
            .as_ref()
            .map(|org| org.created_by.clone())
            .unwrap_or_default();
        let stream_id = problem_db
            .as_ref()
            .map(|org| org.stream_id.clone())
            .unwrap_or_default();
        let created_at = problem_db
            .as_ref()
            .map(|org| org.created_at)
            .unwrap_or_default();
        let command = DeleteProblem {
            id: params.id,
            org_id: organization_id.clone(),
            patient_id: params.patient_id,
            created_by,
            updated_by: user_id,
            created_at,
            last_updated: Utc::now(),
        };
        let events = make_handler(
            &PROBLEM_AGGREGATE,
            &store,
            &ProblemCommand::DeleteProblem(command.clone()),
            &stream_id,
            &EventsReadRange::AllEvents,
            &ExpectedVersion::Any,
        )
        .await?;
        let res =
            process_problems_events(app_state.read_pool.clone(), command.id, stream_id, events)
                .await;

        let row = sync(
            params.last_updated_input,
            organization_id.clone(),
            app_state.read_pool.clone(),
        )
        .await?;

        match res {
            anyhow::Result::Ok(_) => {
                info!("Problem deleted successfully.");
                serde_json::to_value(row).map_err(ErrorData::from)
            }
            Err(err) => {
                error!("{:#?}", err);
                error!("Failed to delete problem.");
                Err(ErrorData {
                    message: String::from("PROBLEM_NOT_DELETED"),
                    data: Value::Null,
                    code: -32600,
                })
            }
        }
    } else {
        Err(ErrorData {
            message: String::from("PATIENT_NOT_FOUND"),
            data: Value::Null,
            code: -32600,
        })
    }
}
