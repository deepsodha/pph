use crate::{
    app_state::AppState,
    doc::{organization::helper::process_organization_events, syncs::sync},
    method::convention::ErrorData,
};
use anyhow::Result;
use chrono::Utc;
use common::stream::Stream;
use common::{
    domain::password::ValidPassword,
    dto::{
        contact::ContactInput, gov_info::GovInfoInput, user::UserInput, user_role::SystemAdminRole,
    },
};
use cosmo_store::types::{event_read_range::EventsReadRange, expected_version::ExpectedVersion};
use cosmo_store_util::aggregate::make_handler;
use log::{error, info};
use organization::domain::{
    organization_aggregate::ORGANIZATION_AGGREGATE,
    organization_commands::{
        CreateOrganization, DeleteOrganization, OrganizationCommand, UpdateOrganization,
    },
};
use serde_json::Value;
use system_admin::dto::{
    system_admin_delete::SystemAdminDelete, system_admin_update::SystemAdminUpdate,
};
use system_admin::{
    domain::{
        systemadmin_aggregate::SYSTEMADMIN_AGGREGATE,
        systemadmin_commands::{
            CreateSystemAdmin, DeleteSystemAdmin, SystemAdminCommands, UpdateSystemAdmin,
            UpdateSystemAdminContactDetails, UpdateSystemAdminGovDetails,
            UpdateSystemAdminUserDetails,
        },
    },
    dto::system_admin_add::SystemAdminAdd,
};
use utils::nanoid_helper::generate_nano_id;
use utils::{
    password_helper::{hash_password, verify_password},
    store_helper::{organization_store, systemadmin_store},
};
use uuid::Uuid;

use super::helper::process_systemadmin_events;
pub async fn add_systemadmin(
    params: SystemAdminAdd,
    app_state: AppState,
    user_id: String,
) -> Result<Value, ErrorData> {
    log::info!("Adding system admin with user_id: {}", user_id);

    let is_valid_password = ValidPassword::parse(&params.password).is_ok();

    if is_valid_password {
        let hash = hash_password(&params.password)
            .map_err(|e| ErrorData::new(-32603, &format!("Password hashing failed: {}", e)))?;
        let is_valid = verify_password(&hash, &params.confirm_password).unwrap_or(false);

        if is_valid {
            log::info!("Adding organization...");

            let org_store = organization_store(&app_state.write_pool).await?;
            let org_stream_id = format!("Organization::{}", Uuid::new_v4().as_simple());
            let org_command = CreateOrganization {
                id: generate_nano_id(),
                created_by: user_id.clone(),
                updated_by: user_id.clone(),
                created_at: Utc::now(),
                last_updated: Utc::now(),
                org_name: params.org_details.name,
                details: params.org_details.details,
                phone_number: params.org_details.phone_number,
                email: params.org_details.email,
                address: params.org_details.address,
            };

            let org_events = make_handler(
                &ORGANIZATION_AGGREGATE,
                &org_store,
                &OrganizationCommand::CreateOrganization(org_command.clone()),
                &org_stream_id,
                &EventsReadRange::AllEvents,
                &ExpectedVersion::Any,
            )
            .await?;

            let org_res = process_organization_events(
                app_state.read_pool.clone(),
                org_command.id.clone(),
                org_stream_id,
                org_events,
            )
            .await;

            match org_res {
                Ok(_) => {
                    let store = systemadmin_store(&app_state.write_pool).await?;
                    let stream_id = format!("SystemAdmin::{}", Uuid::new_v4().as_simple());
                    let command = CreateSystemAdmin {
                        id: generate_nano_id(),
                        org_id: org_command.id.clone(),
                        created_by: user_id.clone(),
                        updated_by: user_id.clone(),
                        created_at: Utc::now(),
                        last_updated: Utc::now(),
                        roles: params.roles,
                        password: hash,
                        user: params.user,
                        government_info: params.government_info,
                        phone: params.phone,
                    };

                    let events = make_handler(
                        &SYSTEMADMIN_AGGREGATE,
                        &store,
                        &SystemAdminCommands::CreateSystemAdmin(Box::new(command.clone())),
                        &stream_id,
                        &EventsReadRange::AllEvents,
                        &ExpectedVersion::Any,
                    )
                    .await?;

                    let res = process_systemadmin_events(
                        app_state.read_pool.clone(),
                        command.id.clone(),
                        org_command.id.clone(),
                        org_command.org_name.clone(),
                        stream_id.clone(),
                        events,
                    )
                    .await;

                    let row = sync(
                        params.last_updated_input,
                        org_command.id.clone(),
                        app_state.read_pool.clone(),
                    )
                    .await?;

                    match res {
                        Ok(_) => {
                            log::info!("System admin added successfully with id: {}", command.id);
                            serde_json::to_value(row).map_err(ErrorData::from)
                        }
                        Err(err) => {
                            log::error!("Failed to add system admin: {:#?}", err);
                            Err(ErrorData {
                                message: String::from("SYSTEMADMIN_NOT_ADDED"),
                                data: Value::Null,
                                code: -32600,
                            })
                        }
                    }
                }
                Err(err) => {
                    log::error!("Failed to add system admin: {:#?}", err);
                    Err(ErrorData {
                        message: String::from("SYSTEMADMIN_NOT_ADDED"),
                        data: Value::Null,
                        code: -32600,
                    })
                }
            }
        } else {
            log::warn!(
                "Password and Confirm Password do not match for user: {}",
                user_id
            );
            Err(ErrorData {
                message: String::from("Password and Confirm Password do not match"),
                data: Value::Null,
                code: -32600,
            })
        }
    } else {
        log::warn!("Invalid password provided for user: {}", user_id);
        Err(ErrorData {
            message: String::from("Password is not valid. It must contain at least one numeric, one alphabetic, and one special character and be at least 6 characters long"),
            data: Value::Null,
            code: -32600,
        })
    }
}

pub async fn update_systemadmin(
    params: SystemAdminUpdate,
    app_state: AppState,
    user_id: String,
) -> Result<Value, ErrorData> {
    let org_store = organization_store(&app_state.write_pool).await?;

    let organization_db = sqlx::query_as::<_, Stream>(
            "SELECT id,stream_id,version,(data->>'created_by') AS created_by, (data->>'created_at') AS created_at,data FROM organization_table_state WHERE id = ?",
        )
        .bind(params.org_id.clone())
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

    let org_command = UpdateOrganization {
        id: params.org_id.clone(),
        created_by,
        updated_by: user_id.clone(),
        created_at,
        last_updated: Utc::now(),
        org_name: params.input.org_details.name,
        details: params.input.org_details.details,
        phone_number: params.input.org_details.phone_number,
        email: params.input.org_details.email,
        address: params.input.org_details.address,
    };

    let org_events = make_handler(
        &ORGANIZATION_AGGREGATE,
        &org_store,
        &OrganizationCommand::UpdateOrganization(org_command.clone()),
        &stream_id,
        &EventsReadRange::AllEvents,
        &ExpectedVersion::Any,
    )
    .await?;

    let _ = process_organization_events(
        app_state.read_pool.clone(),
        org_command.id.clone(),
        stream_id,
        org_events,
    )
    .await;

    log::info!("Organization updated successfully.");

    log::info!("Updating system admin with id: {}", params.id);

    let is_valid_password = ValidPassword::parse(&params.input.password).is_ok();
    if is_valid_password {
        let hash = hash_password(&params.input.password)
            .map_err(|e| ErrorData::new(-32603, &format!("Password hashing failed: {}", e)))?;
        let is_valid = verify_password(&hash, &params.input.confirm_password).unwrap_or(false);

        if is_valid {
            let store = systemadmin_store(&app_state.write_pool).await?;
            let systemadmin_db = sqlx::query_as::<_, Stream>(
                    "SELECT id,stream_id,version,(data->>'created_by') AS created_by, (data->>'created_at') AS created_at,data FROM systemadmin_table_state WHERE id = ?",
                        )
                        .bind(params.id.clone())
                        .fetch_optional(&app_state.read_pool.clone())
                        .await?;

            let id = systemadmin_db
                .as_ref()
                .map(|org| org.id.clone())
                .unwrap_or_default();
            let created_by = systemadmin_db
                .as_ref()
                .map(|org| org.created_by.clone())
                .unwrap_or_default();
            let stream_id = systemadmin_db
                .as_ref()
                .map(|org| org.stream_id.clone())
                .unwrap_or_default();
            let created_at = systemadmin_db
                .as_ref()
                .map(|org| org.created_at)
                .unwrap_or_default();

            let data = systemadmin_db
                .as_ref()
                .map(|org| org.data.clone())
                .unwrap_or_default();
            let data_obj: serde_json::Value =
                serde_json::from_value(data.clone()).unwrap_or_default();

            let user: UserInput =
                serde_json::from_value(data_obj["user"].clone()).unwrap_or_default();
            let government_info: GovInfoInput =
                serde_json::from_value(data_obj["government_info"].clone()).unwrap_or_default();
            let phone: ContactInput =
                serde_json::from_value(data_obj["phone"].clone()).unwrap_or_default();
            let roles: Vec<SystemAdminRole> =
                serde_json::from_value(data_obj["roles"].clone()).unwrap_or_default();
            let password: String =
                serde_json::from_value(data_obj["password"].clone()).unwrap_or_default();

            let condition = verify_password(&password, &params.input.password).unwrap_or(false);

            if params.input.user != user {
                info!("Updating SystemAdmin user");
                let command_user = UpdateSystemAdminUserDetails {
                    id: id.clone(),
                    created_by: created_by.clone(),
                    updated_by: user_id.clone(),
                    created_at,
                    last_updated: Utc::now(),
                    user: params.input.user.clone(),
                };

                let events_user = make_handler(
                    &SYSTEMADMIN_AGGREGATE,
                    &store,
                    &SystemAdminCommands::UpdateSystemAdminUserDetails(Box::new(
                        command_user.clone(),
                    )),
                    &stream_id,
                    &EventsReadRange::AllEvents,
                    &ExpectedVersion::Any,
                )
                .await?;

                let _ = process_systemadmin_events(
                    app_state.read_pool.clone(),
                    command_user.id.clone(),
                    org_command.id.to_string(),
                    org_command.org_name.clone(),
                    stream_id.clone(),
                    events_user,
                )
                .await;
            }

            if params.input.phone != phone {
                info!("Updating SystemAdmin phone");
                let command_phone = UpdateSystemAdminContactDetails {
                    id: id.clone(),
                    created_by: created_by.clone(),
                    updated_by: user_id.clone(),
                    created_at,
                    last_updated: Utc::now(),
                    phone: params.input.phone.clone(),
                };

                let events_phone = make_handler(
                    &SYSTEMADMIN_AGGREGATE,
                    &store,
                    &SystemAdminCommands::UpdateSystemAdminContactDetails(command_phone.clone()),
                    &stream_id,
                    &EventsReadRange::AllEvents,
                    &ExpectedVersion::Any,
                )
                .await?;

                let _ = process_systemadmin_events(
                    app_state.read_pool.clone(),
                    command_phone.id.clone(),
                    org_command.id.to_string(),
                    org_command.org_name.clone(),
                    stream_id.clone(),
                    events_phone,
                )
                .await;
            }

            if params.input.government_info != government_info {
                info!("Updating sytemadmin government_info");
                let command_gov = UpdateSystemAdminGovDetails {
                    id: id.clone(),
                    created_by: created_by.clone(),
                    updated_by: user_id.clone(),
                    created_at,
                    last_updated: Utc::now(),
                    government_info: params.input.government_info.clone(),
                };

                let events_gov = make_handler(
                    &SYSTEMADMIN_AGGREGATE,
                    &store,
                    &SystemAdminCommands::UpdateSystemAdminGovDetails(command_gov.clone()),
                    &stream_id,
                    &EventsReadRange::AllEvents,
                    &ExpectedVersion::Any,
                )
                .await?;

                let _ = process_systemadmin_events(
                    app_state.read_pool.clone(),
                    command_gov.id.clone(),
                    org_command.id.to_string(),
                    org_command.org_name.clone(),
                    stream_id.clone(),
                    events_gov,
                )
                .await;
            }

            if params.input.roles != roles || !condition {
                let command = UpdateSystemAdmin {
                    id: params.id.clone(),
                    org_id: org_command.id.clone(),
                    created_by,
                    updated_by: user_id.clone(),
                    created_at,
                    last_updated: Utc::now(),
                    roles: params.input.roles.clone(),
                    password: hash,
                };

                let events = make_handler(
                    &SYSTEMADMIN_AGGREGATE,
                    &store,
                    &SystemAdminCommands::UpdateSystemAdmin(command.clone()),
                    &stream_id,
                    &EventsReadRange::AllEvents,
                    &ExpectedVersion::Any,
                )
                .await?;
                let _ = process_systemadmin_events(
                    app_state.read_pool.clone(),
                    command.id.clone(),
                    org_command.id.to_string(),
                    org_command.org_name.clone(),
                    stream_id.clone(),
                    events,
                )
                .await;
            }

            let row = sync(
                params.input.last_updated_input,
                org_command.id.to_string(),
                app_state.read_pool.clone(),
            )
            .await?;

            if true {
                log::info!("System admin updated successfully with id: {}", params.id);
                serde_json::to_value(row).map_err(ErrorData::from)
            } else {
                log::error!("Failed to update system admin");
                Err(ErrorData {
                    message: String::from("SYSTEMADMIN_NOT_UPDATED"),
                    data: Value::Null,
                    code: -32600,
                })
            }
        } else {
            log::warn!(
                "Password and Confirm Password do not match for system admin: {}",
                params.id
            );
            Err(ErrorData {
                message: String::from("Password and Confirm Password do not match"),
                data: Value::Null,
                code: -32600,
            })
        }
    } else {
        log::warn!("Invalid password provided for system admin: {}", params.id);
        Err(ErrorData {
                message: String::from("Password is not valid. It must contain at least one numeric, one alphabetic, and one special character and be at least 6 characters long"),
                data: Value::Null,
                code: -32600,
            })
    }
}

pub async fn delete_systemadmin(
    params: SystemAdminDelete,
    app_state: AppState,
    user_id: String,
) -> Result<Value, ErrorData> {
    log::info!("Deleting organization...");

    let org_store = organization_store(&app_state.write_pool).await?;
    let organization_db = sqlx::query_as::<_, Stream>(
        "SELECT id,stream_id,version,(data->>'created_by') AS created_by, (data->>'created_at') AS created_at,data FROM organization_table_state WHERE id = ?",
    )
    .bind(params.org_id.clone())
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

    let org_command = DeleteOrganization {
        id: params.org_id.clone(),
        created_by,
        updated_by: user_id.clone(),
        created_at,
        last_updated: Utc::now(),
    };

    let org_events = make_handler(
        &ORGANIZATION_AGGREGATE,
        &org_store,
        &OrganizationCommand::DeleteOrganization(org_command.clone()),
        &stream_id,
        &EventsReadRange::AllEvents,
        &ExpectedVersion::Any,
    )
    .await?;

    let _ = process_organization_events(
        app_state.read_pool.clone(),
        org_command.id.clone(),
        stream_id,
        org_events,
    )
    .await;

    log::info!("Organization deleted successfully.");
    let store = systemadmin_store(&app_state.write_pool).await?;
    let systemadmin_db = sqlx::query_as::<_, Stream>(
        "SELECT id,stream_id,version,(data->>'created_by') AS created_by, (data->>'created_at') AS created_at,data FROM systemadmin_table_state WHERE id = ?",
    )
    .bind(params.id.clone())
    .fetch_optional(&app_state.read_pool.clone())
    .await?;

    let created_by = systemadmin_db
        .as_ref()
        .map(|org| org.created_by.clone())
        .unwrap_or_default();
    let stream_id = systemadmin_db
        .as_ref()
        .map(|org| org.stream_id.clone())
        .unwrap_or_default();
    let created_at = systemadmin_db
        .as_ref()
        .map(|org| org.created_at)
        .unwrap_or_default();
    let command = DeleteSystemAdmin {
        id: params.id,
        created_by,
        updated_by: user_id,
        created_at,
        last_updated: Utc::now(),
        org_id: org_command.id.clone(),
    };

    let events = make_handler(
        &SYSTEMADMIN_AGGREGATE,
        &store,
        &SystemAdminCommands::DeleteSystemAdmin(command.clone()),
        &stream_id,
        &EventsReadRange::AllEvents,
        &ExpectedVersion::Any,
    )
    .await?;
    let res = process_systemadmin_events(
        app_state.read_pool.clone(),
        command.id,
        org_command.id.to_string(),
        "".to_string(),
        stream_id,
        events,
    )
    .await;

    let row = sync(
        params.last_updated_input,
        org_command.id.to_string(),
        app_state.read_pool.clone(),
    )
    .await?;

    match res {
        anyhow::Result::Ok(_) => serde_json::to_value(row).map_err(ErrorData::from),
        Err(err) => {
            error!("{:#?}", err);
            Err(ErrorData {
                message: String::from("SYSTEMADMIN_NOT_DELETED"),
                data: Value::Null,
                code: -32600,
            })
        }
    }
}
