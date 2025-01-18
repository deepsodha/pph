use crate::doc::patients::get_patient_id::patient_id_fetch;
use crate::doc::syncs::sync;
use crate::{app_state::AppState, method::convention::ErrorData};
use chrono::NaiveDate;
use common::stream::Stream;
use cosmo_store_util::aggregate::make_handler;
use csv::ReaderBuilder;
use organization::dto::csv::AllergiesRecord;
use patient::domain::allergies::allergies_aggregate::ALLERGIES_AGGREGATE;
use patient::domain::allergies::allergies_commands::{
    AllergiesCommand, CreateAllergies, DeleteAllergies, UpdateAllergies,
};
use std::collections::HashSet;

use patient::dto::allergies::allergy_add::AllergyAdd;

use super::helper::process_allergies_events;
use anyhow::Result;
use cosmo_store::types::{event_read_range::EventsReadRange, expected_version::ExpectedVersion};
use patient::dto::allergies::allergy_delete::AllergyDelete;
use patient::dto::allergies::allergy_update::AllergyUpdate;
use serde_json::Value;
use sqlx::types::chrono::Utc;
use std::error::Error;
use utils::store_helper::patient_store;
use uuid::Uuid;

use log::{error, info};

pub async fn add_allergy(
    params: AllergyAdd,
    app_state: AppState,
    organization_id: String,
    user_id: String,
) -> Result<Value, ErrorData> {
    let patient_id = patient_id_fetch(app_state.clone(), params.patient_id.clone()).await?;

    if patient_id {
        match parse_date(&params.input_date) {
            Ok(input_date_utc) => {
                info!("Start adding allergy entry");

                let store = patient_store(&app_state.write_pool, &organization_id).await?;
                let stream_id = format!("Allergies::{}", Uuid::new_v4().as_simple());
                let command = CreateAllergies {
                    id: Uuid::new_v4().as_simple().to_string(),
                    patient_id: params.patient_id,
                    created_by: user_id.clone(),
                    updated_by: user_id.clone(),
                    created_at: Utc::now(),
                    last_updated: Utc::now(),
                    org_id: organization_id.clone(),
                    allergen: params.allergen,
                    reaction: params.reaction,
                    allergy_severities: params.allergy_severities,
                    input_date: input_date_utc,
                    status: params.status,
                    comments: params.comments,
                };
                let events = make_handler(
                    &ALLERGIES_AGGREGATE,
                    &store,
                    &AllergiesCommand::CreateAllergies(command.clone()),
                    &stream_id,
                    &EventsReadRange::AllEvents,
                    &ExpectedVersion::Any,
                )
                .await?;
                let res = process_allergies_events(
                    app_state.read_pool.clone(),
                    command.id,
                    stream_id,
                    events,
                )
                .await;

                let row = sync(
                    params.last_updated_input,
                    organization_id.clone(),
                    app_state.read_pool.clone(),
                )
                .await?;

                match res {
                    Ok(_) => {
                        info!("Allergy entry added successfully");
                        serde_json::to_value(row).map_err(ErrorData::from)
                    }
                    Err(err) => {
                        error!("{:#?}", err);
                        Err(ErrorData {
                            message: String::from("ALLERYIES_NOT_ADDED"),
                            data: Value::Null,
                            code: -32600,
                        })
                    }
                }
            }

            Err(error) => Err(ErrorData {
                message: error.to_string(),
                data: Value::Null,
                code: -32600,
            }),
        }
    } else {
        Err(ErrorData {
            message: String::from("PATIENT_NOT_FOUND"),
            data: Value::Null,
            code: -32600,
        })
    }
}

pub async fn update_allergy(
    params: AllergyUpdate,
    app_state: AppState,
    organization_id: String,
    user_id: String,
) -> Result<Value, ErrorData> {
    info!("Start updating allergy entry");

    let patient_id = patient_id_fetch(app_state.clone(), params.input.patient_id.clone()).await?;
    if patient_id {
        match parse_date(&params.input.input_date) {
            Ok(input_date_utc) => {
                let store = patient_store(&app_state.write_pool, &organization_id).await?;
                let allergies_db = sqlx::query_as::<_, Stream>(
        "SELECT id,stream_id,version,(data->>'created_by') AS created_by, (data->>'created_at') AS created_at,data FROM allergies_table_state WHERE id = ?",
    )
    .bind(params.id.clone())
    .fetch_optional(&app_state.read_pool.clone())
    .await?;

                let created_by = allergies_db
                    .as_ref()
                    .map(|org| org.created_by.clone())
                    .unwrap_or_default();
                let stream_id = allergies_db
                    .as_ref()
                    .map(|org| org.stream_id.clone())
                    .unwrap_or_default();
                let created_at = allergies_db
                    .as_ref()
                    .map(|org| org.created_at)
                    .unwrap_or_default();
                let command = UpdateAllergies {
                    id: params.id,
                    created_by,
                    updated_by: user_id,
                    created_at,
                    last_updated: Utc::now(),
                    patient_id: params.input.patient_id,
                    org_id: organization_id.clone(),
                    allergen: params.input.allergen,
                    reaction: params.input.reaction,
                    allergy_severities: params.input.allergy_severities,
                    input_date: input_date_utc,
                    status: params.input.status,
                    comments: params.input.comments,
                };
                let events = make_handler(
                    &ALLERGIES_AGGREGATE,
                    &store,
                    &AllergiesCommand::UpdateAllergies(command.clone()),
                    &stream_id,
                    &EventsReadRange::AllEvents,
                    &ExpectedVersion::Any,
                )
                .await?;
                let res = process_allergies_events(
                    app_state.read_pool.clone(),
                    command.id,
                    stream_id,
                    events,
                )
                .await;

                let row = sync(
                    params.input.last_updated_input,
                    organization_id.clone(),
                    app_state.read_pool.clone(),
                )
                .await?;

                match res {
                    Ok(_) => {
                        info!("Allergy entry updated successfully");
                        serde_json::to_value(row).map_err(ErrorData::from)
                    }
                    Err(err) => {
                        error!("{:#?}", err);
                        Err(ErrorData {
                            message: String::from("ALLERGIES_NOT_UPDATED"),
                            data: Value::Null,
                            code: -32600,
                        })
                    }
                }
            }
            Err(error) => Err(ErrorData {
                message: error.to_string(),
                data: Value::Null,
                code: -32600,
            }),
        }
    } else {
        Err(ErrorData {
            message: String::from("PATIENT_NOT_FOUND"),
            data: Value::Null,
            code: -32600,
        })
    }
}

pub async fn delete_allergy(
    params: AllergyDelete,
    app_state: AppState,
    organization_id: String,
    user_id: String,
) -> Result<Value, ErrorData> {
    let patient_id = patient_id_fetch(app_state.clone(), params.patient_id.clone()).await?;

    if patient_id {
        info!("Start deleting allergy entry");

        let store = patient_store(&app_state.write_pool, &organization_id).await?;
        let allergies_db = sqlx::query_as::<_, Stream>(
        "SELECT id,stream_id,version,(data->>'created_by') AS created_by, (data->>'created_at') AS created_at,data FROM allergies_table_state WHERE id = ?",
    )
    .bind(params.id.clone())
    .fetch_optional(&app_state.read_pool.clone())
    .await?;

        let created_by = allergies_db
            .as_ref()
            .map(|org| org.created_by.clone())
            .unwrap_or_default();
        let stream_id = allergies_db
            .as_ref()
            .map(|org| org.stream_id.clone())
            .unwrap_or_default();
        let created_at = allergies_db
            .as_ref()
            .map(|org| org.created_at)
            .unwrap_or_default();

        let command = DeleteAllergies {
            id: params.id,
            patient_id: params.patient_id,
            org_id: organization_id.clone(),
            created_by,
            updated_by: user_id,
            created_at,
            last_updated: Utc::now(),
        };
        let events = make_handler(
            &ALLERGIES_AGGREGATE,
            &store,
            &AllergiesCommand::DeleteAllergies(command.clone()),
            &stream_id,
            &EventsReadRange::AllEvents,
            &ExpectedVersion::Any,
        )
        .await?;

        let res =
            process_allergies_events(app_state.read_pool.clone(), command.id, stream_id, events)
                .await;

        let row = sync(
            params.last_updated_input,
            organization_id.clone(),
            app_state.read_pool.clone(),
        )
        .await?;

        match res {
            Ok(_) => {
                info!("Allergy entry deleted successfully");
                serde_json::to_value(row).map_err(ErrorData::from)
            }
            Err(err) => {
                error!("{:#?}", err);
                Err(ErrorData {
                    message: String::from("ALLERGIES_NOT_DELETED"),
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

pub async fn get_all_allergies_handler() -> Result<Value, ErrorData> {
    log::info!("Handling get_all_allergies request");

    let path = "statics/data/allergy.csv";
    let records = read_allergies_csv(path).unwrap();

    let countries = get_all_allergies(&records);

    log::info!("get_all_allergies request handled successfully");
    serde_json::to_value(countries).map_err(ErrorData::from)
}

pub fn get_all_allergies(records: &[AllergiesRecord]) -> Vec<&str> {
    let unique_countries: HashSet<&str> = records
        .iter()
        .map(|record| record.allergy.as_str())
        .collect();

    let countries = unique_countries.into_iter().collect::<Vec<&str>>();

    log::info!("Retrieved all countries");

    countries
}

pub fn read_allergies_csv(file_path: &str) -> Result<Vec<AllergiesRecord>, Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new().from_path(file_path)?;
    let records = rdr
        .records()
        .filter_map(|record| {
            record.map_or_else(
                |e| {
                    log::error!("Error reading CSV record: {}", e);
                    None
                },
                Some,
            )
        })
        .map(|row| AllergiesRecord {
            allergy: row[0].to_string(),
        })
        .collect::<Vec<_>>();

    log::info!("CSV file read successfully");

    Ok(records)
}

fn parse_date(date: &str) -> Result<NaiveDate, String> {
    match NaiveDate::parse_from_str(date, "%Y-%m-%d") {
        Ok(date_obj) => Ok(date_obj),
        Err(_) => Err("Parse Error".to_string()),
    }
}
