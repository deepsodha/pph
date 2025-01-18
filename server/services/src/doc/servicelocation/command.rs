use super::helper::process_servicelocation_events;
use crate::{app_state::AppState, doc::syncs::sync, method::convention::ErrorData};
use anyhow::Result;
use auth::dto::authenticate_user::AuthenticatedUser;
use biscuit_auth::KeyPair;
use chrono::NaiveTime;
use common::stream::Stream;
use cosmo_store::types::{event_read_range::EventsReadRange, expected_version::ExpectedVersion};
use cosmo_store_util::aggregate::make_handler;
use cosmo_store_util::aggregate::Aggregate;
use serde_json::Value;
use servicelocation::{
    domain::{
        service_location_aggregate::SERVICELOCATION_AGGREGATE,
        service_location_commands::{
            CreateSerivceLocation, DeleteServiceLocation, ServiceLocationCommand,
            ServiceLocationSelect, UpdateServiceLocation,
        },
    },
    dto::{
        select_servicelocation::SelectServiceLocation, servicelocation_add::ServiceLocationAdd,
        servicelocation_delete::ServiceLocationDelete,
        servicelocation_update::ServiceLocationUpdate,
    },
};
use sqlx::types::chrono::Utc;
use utils::biscuit::helper::TokenData;
use utils::{biscuit::helper::create_token, store_helper::servicelocation_store};
use uuid::Uuid;
use validator::Validate;

pub async fn add_servicelocation(
    params: ServiceLocationAdd,
    app_state: AppState,
    organization_id: String,
    user_id: String,
) -> Result<Value, ErrorData> {
    match parse_time_range(&params.start_time, &params.end_time) {
        Ok((utc_start_time, utc_end_time)) => {
            log::info!(
                "Adding service location for organization_id: {} and user_id: {}",
                organization_id,
                user_id
            );

            let store = servicelocation_store(&app_state.write_pool, &organization_id).await?;
            let stream_id = format!("ServiceLocation::{}", Uuid::new_v4().as_simple());
            let command = CreateSerivceLocation {
                id: Uuid::new_v4().as_simple().to_string(),
                org_id: organization_id.clone(),
                start_time: utc_start_time,
                end_time: utc_end_time,
                service_location_name: params.service_location_name,
                created_by: user_id.clone(),
                updated_by: user_id.clone(),
                created_at: Utc::now(),
                last_updated: Utc::now(),
                address: params.address,
            };
            let events = make_handler(
                &SERVICELOCATION_AGGREGATE,
                &store,
                &ServiceLocationCommand::CreateSerivceLocation(command.clone()),
                &stream_id,
                &EventsReadRange::AllEvents,
                &ExpectedVersion::Any,
            )
            .await?;
            let res = process_servicelocation_events(
                app_state.read_pool.clone(),
                command.id,
                stream_id.clone(),
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
                    log::info!(
                        "Service location added successfully for organization_id: {} and user_id: {}",
                        organization_id,
                        user_id
                    );
                    serde_json::to_value(row).map_err(ErrorData::from)
                }
                Err(err) => {
                    log::error!(
                        "Error adding service location for organization_id: {} and user_id: {}: {:#?}",
                        organization_id,
                        user_id,
                        err
                    );
                    Err(ErrorData {
                        message: String::from("SERVICELOCATION_NOT_ADDED"),
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
}

pub async fn update_servicelocation(
    params: ServiceLocationUpdate,
    app_state: AppState,
    organization_id: String,
    user_id: String,
) -> Result<Value, ErrorData> {
    match parse_time_range(&params.input.start_time, &params.input.end_time) {
        Ok((utc_start_time, utc_end_time)) => {
            log::info!(
                "Updating service location for organization_id: {} and user_id: {}",
                organization_id,
                user_id
            );

            let store = servicelocation_store(&app_state.write_pool, &organization_id).await?;
            let servicelocation_db = sqlx::query_as::<_, Stream>(
                "SELECT id,stream_id,version,(data->>'created_by') AS created_by, (data->>'created_at') AS created_at,data FROM servicelocation_table_state WHERE id = ?",
            )
            .bind(params.id.clone())
            .fetch_optional(&app_state.read_pool.clone())
            .await?;

            let created_by = servicelocation_db
                .as_ref()
                .map(|org| org.created_by.clone())
                .unwrap_or_default();
            let stream_id = servicelocation_db
                .as_ref()
                .map(|org| org.stream_id.clone())
                .unwrap_or_default();
            let created_at = servicelocation_db
                .as_ref()
                .map(|org| org.created_at)
                .unwrap_or_default();
            let command = UpdateServiceLocation {
                id: params.id,
                org_id: organization_id.clone(),
                start_time: utc_start_time,
                end_time: utc_end_time,
                created_by,
                updated_by: user_id.clone(),
                created_at,
                last_updated: Utc::now(),
                address: params.input.address,
                service_location_name: params.input.service_location_name,
            };
            let events = make_handler(
                &SERVICELOCATION_AGGREGATE,
                &store,
                &ServiceLocationCommand::UpdateServiceLocation(command.clone()),
                &stream_id,
                &EventsReadRange::AllEvents,
                &ExpectedVersion::Any,
            )
            .await?;
            let res = process_servicelocation_events(
                app_state.read_pool.clone(),
                command.id.clone(),
                stream_id.clone(),
                events,
            )
            .await;

            let row = sync(
                params.input.last_updated_input.clone(),
                organization_id.clone(),
                app_state.read_pool.clone(),
            )
            .await?;

            match res {
                Ok(_) => {
                    log::info!(
                        "Service location updated successfully for organization_id: {} and user_id: {}",
                        organization_id,
                        user_id
                    );
                    serde_json::to_value(row).map_err(ErrorData::from)
                }
                Err(err) => {
                    log::error!(
                        "Error updating service location for organization_id: {} and user_id: {}: {:#?}",
                        organization_id,
                        user_id,
                        err
                    );
                    Err(ErrorData {
                        message: String::from("SERVICELOCATION_NOT_UPDATED"),
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
}

pub async fn delete_servicelocation(
    params: ServiceLocationDelete,
    app_state: AppState,
    organization_id: String,
    user_id: String,
) -> Result<Value, ErrorData> {
    log::info!(
        "Deleting service location with id: {} for organization_id: {} and user_id: {}",
        params.id,
        organization_id,
        user_id
    );

    let store = servicelocation_store(&app_state.write_pool, &organization_id).await?;
    let servicelocation_db = sqlx::query_as::<_, Stream>(
        "SELECT id,stream_id,version,(data->>'created_by') AS created_by, (data->>'created_at') AS created_at,data FROM servicelocation_table_state WHERE id = ?",
    )
    .bind(params.id.clone())
    .fetch_optional(&app_state.read_pool.clone())
    .await?;

    let created_by = servicelocation_db
        .as_ref()
        .map(|org| org.created_by.clone())
        .unwrap_or_default();
    let stream_id = servicelocation_db
        .as_ref()
        .map(|org| org.stream_id.clone())
        .unwrap_or_default();
    let created_at = servicelocation_db
        .as_ref()
        .map(|org| org.created_at)
        .unwrap_or_default();
    let command = DeleteServiceLocation {
        id: params.id.clone(),
        org_id: organization_id.clone(),
        created_by,
        updated_by: user_id.clone(),
        created_at,
        last_updated: Utc::now(),
    };

    let events = make_handler(
        &SERVICELOCATION_AGGREGATE,
        &store,
        &ServiceLocationCommand::DeleteServiceLocation(command.clone()),
        &stream_id,
        &EventsReadRange::AllEvents,
        &ExpectedVersion::Any,
    )
    .await?;
    let res = process_servicelocation_events(
        app_state.read_pool.clone(),
        command.id.clone(),
        stream_id.clone(),
        events,
    )
    .await;

    let row = sync(
        params.last_updated_input.clone(),
        organization_id.clone(),
        app_state.read_pool.clone(),
    )
    .await?;

    match res {
        Ok(_) => {
            log::info!("Service location with id: {} deleted successfully for organization_id: {} and user_id: {}", params.id.clone(), organization_id, user_id);
            serde_json::to_value(row).map_err(ErrorData::from)
        }
        Err(err) => {
            log::error!("Error deleting service location with id: {} for organization_id: {} and user_id: {}: {:#?}", params.id, organization_id, user_id, err);
            Err(ErrorData {
                message: String::from("SERVICELOCATION_NOT_DELETED"),
                data: Value::Null,
                code: -32600,
            })
        }
    }
}

pub async fn select_servicelocation(
    input: SelectServiceLocation,
    roles: Vec<String>,
    app_state: AppState,
    user_id: String,
    org_id: String,
    org_name: String,
    user_name: String,
) -> Result<Value, ErrorData> {
    log::info!("Selecting ServiceLocation...");

    match input.validate() {
        Ok(_) => {
            let vec_of_refs: Vec<&str> = roles.iter().map(|s| s.as_str()).collect();

            let store = servicelocation_store(&app_state.write_pool, &org_id).await?;
            let stream_id = format!("ServiceLocation::{}", Uuid::new_v4().as_simple());
            let events = make_handler(
                &SERVICELOCATION_AGGREGATE,
                &store,
                &ServiceLocationCommand::ServiceLocationSelect(ServiceLocationSelect::from(input)),
                &stream_id,
                &EventsReadRange::AllEvents,
                &ExpectedVersion::Any,
            )
            .await?;
            let state = events
                .iter()
                .fold(SERVICELOCATION_AGGREGATE.init(), |a, b| {
                    SERVICELOCATION_AGGREGATE.apply(a, &b.data)
                });
            match state {
                Some(s) => {
                    let token = create_token(
                        &KeyPair::from(&app_state.private_key),
                        TokenData {
                            user_roles: vec_of_refs,
                            user_id: &user_id,
                            org_id: &org_id,
                            org_name: &org_name,
                            user_name: &user_name,
                            service_location_id: &s.id,
                            service_location_name: &s.service_location_name,
                        },
                    )?;

                    log::info!("SerivceLocation selected successfully.");

                    serde_json::to_value(AuthenticatedUser {
                        id: user_id.clone(),
                        token,
                        user_name,
                        selected_language: "EN".to_string(),
                        role: roles,
                        org_id,
                        org_name,
                        service_location_id: s.id,
                    })
                    .map_err(ErrorData::from)
                }
                None => {
                    log::error!("State Not Found");
                    Err(ErrorData {
                        message: String::from("State Not Found"),
                        data: Value::Null,
                        code: -32600,
                    })
                }
            }
        }
        Err(error) => {
            log::error!("Validation error: {}", error);
            Err(ErrorData {
                message: error.to_string(),
                data: Value::Null,
                code: -32600,
            })
        }
    }
}

fn parse_time_range(start_time: &str, end_time: &str) -> Result<(NaiveTime, NaiveTime), String> {
    let start_time_obj = match NaiveTime::parse_from_str(start_time, "%H:%M:%S") {
        Ok(time) => time,
        Err(_) => return Err("Invalid start time format".to_string()),
    };

    let end_time_obj = match NaiveTime::parse_from_str(end_time, "%H:%M:%S") {
        Ok(time) => time,
        Err(_) => return Err("Invalid end time format".to_string()),
    };

    Ok((start_time_obj, end_time_obj))
}
