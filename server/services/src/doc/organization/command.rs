use super::helper::process_organization_events;
use crate::{app_state::AppState, method::convention::ErrorData};
use anyhow::Result;
use auth::dto::authenticate_user::AuthenticatedUser;
use biscuit_auth::KeyPair;
use common::stream::Stream;
use cosmo_store::types::{event_read_range::EventsReadRange, expected_version::ExpectedVersion};
use cosmo_store_util::aggregate::{make_handler, Aggregate};
use organization::{
    domain::{
        organization_aggregate::ORGANIZATION_AGGREGATE,
        organization_commands::{
            CreateOrganization, DeleteOrganization, OrganizationCommand, OrganizationSelect,
            UpdateOrganization,
        },
    },
    dto::{
        organization_add::OrganizationAdd, organization_delete::OrganizationDelete,
        organization_update::OrganizationUpdate, select_organization::SelectOrganization,
    },
};
use serde_json::Value;
use sqlx::types::chrono::Utc;
use utils::biscuit::helper::{create_token, TokenData};
use utils::store_helper::organization_store;
use uuid::Uuid;
use validator::Validate;

pub async fn add_organization(
    params: OrganizationAdd,
    app_state: AppState,
    user_id: String,
) -> Result<Value, ErrorData> {
    log::info!("Adding organization...");

    let store = organization_store(&app_state.write_pool).await?;
    let stream_id = format!("Organization::{}", Uuid::new_v4().as_simple());
    let command = CreateOrganization {
        id: Uuid::new_v4().as_simple().to_string(),
        created_by: user_id.clone(),
        updated_by: user_id.clone(),
        created_at: Utc::now(),
        last_updated: Utc::now(),
        org_name: params.name,
        details: params.details,
        phone_number: params.phone_number,
        email: params.email,
        address: params.address,
    };

    let events = make_handler(
        &ORGANIZATION_AGGREGATE,
        &store,
        &OrganizationCommand::CreateOrganization(command.clone()),
        &stream_id,
        &EventsReadRange::AllEvents,
        &ExpectedVersion::Any,
    )
    .await?;

    let res = process_organization_events(app_state.read_pool, command.id, stream_id, events).await;

    match res {
        Ok(_) => {
            log::info!("Organization added successfully.");
            serde_json::to_value(String::from("Successful")).map_err(ErrorData::from)
        }
        Err(err) => {
            log::error!("Error adding organization: {:#?}", err);
            Err(ErrorData {
                message: String::from("ORGANIZATION_NOT_ADDED"),
                data: Value::Null,
                code: -32600,
            })
        }
    }
}

pub async fn update_organization(
    params: OrganizationUpdate,
    app_state: AppState,
    user_id: String,
) -> Result<Value, ErrorData> {
    log::info!("Updating organization...");

    let store = organization_store(&app_state.write_pool).await?;

    let organization_db = sqlx::query_as::<_, Stream>(
        "SELECT id,stream_id,version,(data->>'created_by') AS created_by, (data->>'created_at') AS created_at,data FROM organization_table_state WHERE id = ?",
    )
    .bind(params.id.clone())
    .fetch_optional(&app_state.read_pool.clone())
    .await?;

    let created_by = organization_db
        .as_ref()
        .map(|org| org.created_by.clone())
        .unwrap_or_default();
    let stream_id = organization_db
        .as_ref()
        .map(|org| org.stream_id.clone())
        .unwrap_or_default();
    let created_at = organization_db
        .as_ref()
        .map(|org| org.created_at)
        .unwrap_or_default();

    let command = UpdateOrganization {
        id: params.id,
        created_by,
        updated_by: user_id,
        created_at,
        last_updated: Utc::now(),
        org_name: params.name,
        details: params.details,
        phone_number: params.phone_number,
        email: params.email,
        address: params.address,
    };

    let events = make_handler(
        &ORGANIZATION_AGGREGATE,
        &store,
        &OrganizationCommand::UpdateOrganization(command.clone()),
        &stream_id,
        &EventsReadRange::AllEvents,
        &ExpectedVersion::Any,
    )
    .await?;

    let res = process_organization_events(app_state.read_pool, command.id, stream_id, events).await;

    match res {
        Ok(_) => {
            log::info!("Organization updated successfully.");
            serde_json::to_value(String::from("Successful")).map_err(ErrorData::from)
        }
        Err(err) => {
            log::error!("Error updating organization: {:#?}", err);
            Err(ErrorData {
                message: String::from("ORGANIZATION_NOT_UPDATED"),
                data: Value::Null,
                code: -32600,
            })
        }
    }
}

pub async fn delete_organization(
    params: OrganizationDelete,
    app_state: AppState,
    user_id: String,
) -> Result<Value, ErrorData> {
    log::info!("Deleting organization...");

    let store = organization_store(&app_state.write_pool).await?;
    let organization_db = sqlx::query_as::<_, Stream>(
        "SELECT id,stream_id,version,(data->>'created_by') AS created_by, (data->>'created_at') AS created_at,data FROM organization_table_state WHERE id = ?",
    )
    .bind(params.id.clone())
    .fetch_optional(&app_state.read_pool.clone())
    .await?;

    let created_by = organization_db
        .as_ref()
        .map(|org| org.created_by.clone())
        .unwrap_or_default();
    let stream_id = organization_db
        .as_ref()
        .map(|org| org.stream_id.clone())
        .unwrap_or_default();
    let created_at = organization_db
        .as_ref()
        .map(|org| org.created_at)
        .unwrap_or_default();

    let command = DeleteOrganization {
        id: params.id,
        created_by,
        updated_by: user_id,
        created_at,
        last_updated: Utc::now(),
    };

    let events = make_handler(
        &ORGANIZATION_AGGREGATE,
        &store,
        &OrganizationCommand::DeleteOrganization(command.clone()),
        &stream_id,
        &EventsReadRange::AllEvents,
        &ExpectedVersion::Any,
    )
    .await?;

    let res = process_organization_events(app_state.read_pool, command.id, stream_id, events).await;

    match res {
        Ok(_) => {
            log::info!("Organization deleted successfully.");
            serde_json::to_value(String::from("Successful")).map_err(ErrorData::from)
        }
        Err(err) => {
            log::error!("Error deleting organization: {:#?}", err);
            Err(ErrorData {
                message: String::from("ORGANIZATION_NOT_DELETED"),
                data: Value::Null,
                code: -32600,
            })
        }
    }
}

pub async fn select_organization(
    input: SelectOrganization,
    roles: Vec<String>,
    app_state: AppState,
    user_id: String,
    user_name: String,
) -> Result<Value, ErrorData> {
    log::info!("Selecting organization...");

    match input.validate() {
        Ok(_) => {
            let vec_of_refs: Vec<&str> = roles.iter().map(|s| s.as_str()).collect();

            let store = organization_store(&app_state.write_pool).await?;
            let stream_id = format!("Organization::{}", Uuid::new_v4().as_simple());
            let events = make_handler(
                &ORGANIZATION_AGGREGATE,
                &store,
                &OrganizationCommand::SelectOrganization(OrganizationSelect::from(input)),
                &stream_id,
                &EventsReadRange::AllEvents,
                &ExpectedVersion::Any,
            )
            .await?;
            let state = events.iter().fold(ORGANIZATION_AGGREGATE.init(), |a, b| {
                ORGANIZATION_AGGREGATE.apply(a, &b.data)
            });
            match state {
                Some(s) => {
                    let token = create_token(
                        &KeyPair::from(&app_state.private_key),
                        TokenData {
                            user_id: &user_id,
                            user_name: &user_name,
                            user_roles: vec_of_refs,
                            org_id: &s.id,
                            org_name: &s.name,
                            service_location_id: "",
                            service_location_name: "",
                        },
                    )?;

                    log::info!("Organization selected successfully.");

                    serde_json::to_value(AuthenticatedUser {
                        id: user_id.clone(),
                        token,
                        user_name,
                        selected_language: "EN".to_string(),
                        role: roles,
                        org_id: s.id,
                        org_name: s.name,
                        service_location_id: "".to_string(),
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
