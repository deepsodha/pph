use crate::doc::doctors::helper::process_doctor_events;
use crate::doc::staff::helper::process_staff_events;
use crate::doc::user::helper::process_user_events;
use crate::{app_state::AppState, doc::syncs::sync, method::convention::ErrorData};
use anyhow::Error;
use chrono::Utc;
use common::dto::last_updated_input::LastUpdatedInput;
use common::stream::Stream;
use common::{
    domain::password::ValidPassword,
    dto::{address::AddressInput, contact::ContactInput, gov_info::GovInfoInput, user::UserInput},
};
use cosmo_store::types::{event_read_range::EventsReadRange, expected_version::ExpectedVersion};
use cosmo_store_util::aggregate::make_handler;
use doctor::domain::doctor_aggregate::DOCTOR_AGGREGATE;
use doctor::domain::doctor_commands::{DeleteDoctor, DoctorCommand, UpdateDoctor};
use doctor::dto::doctor_type::DoctorType;
use log::{error, info};
use serde_json::Value;
use staff::domain::staff_aggregate::STAFF_AGGREGATE;
use staff::domain::staff_commands::{DeleteStaff, StaffCommand, UpdateStaff};
use staff::dto::staff_type::StaffType;
use user::domain::user_aggregate::USER_AGGREGATE;
use user::domain::user_commands::{
    CreateUser, DeleteUser, UpdateUserAddress, UpdateUserContact, UpdateUserDetails,
    UpdateUserGovDetails, UpdateUserPassword, UserCommand,
};
use user::domain::user_domain::UserState;
use user::dto::user_add::UserAdd;
use user::dto::user_delete::UserDelete;
use user::dto::user_update::UserUpdate;
use utils::password_helper::{hash_password, verify_password};
use utils::store_helper::{doctor_store, staff_store, user_store};
use uuid::Uuid;

pub async fn add_user(
    params: UserAdd,
    app_state: AppState,
    organization_id: String,
    user_id: String,
) -> Result<Value, ErrorData> {
    log::info!(
        "Adding user for organization_id: {} and user_id: {}",
        organization_id,
        user_id
    );
    let is_valid_password = ValidPassword::parse(&params.password).is_ok();
    if is_valid_password {
        let hash = hash_password(&params.password)
            .map_err(|e| ErrorData::new(-32603, &format!("Password hashing failed: {}", e)))?;
        let is_valid = verify_password(&hash, &params.confirm_password).unwrap_or(false);
        if is_valid {
            let store = user_store(&app_state.write_pool, &organization_id).await?;
            let stream_id = format!("User::{}", Uuid::new_v4().as_simple());
            let command = CreateUser {
                id: Uuid::new_v4().as_simple().to_string(),
                org_id: organization_id.clone(),
                created_by: user_id.clone(),
                updated_by: user_id.clone(),
                created_at: Utc::now(),
                last_updated: Utc::now(),
                phone: params.phone,
                password: hash,
                user: params.user,
                address: params.address,
                government_info: params.goverment_info,
            };
            let events = make_handler(
                &USER_AGGREGATE,
                &store,
                &UserCommand::CreateUser(Box::new(command.clone())),
                &stream_id,
                &EventsReadRange::AllEvents,
                &ExpectedVersion::Any,
            )
            .await?;
            let res = process_user_events(
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
                        "User added successfully for organization_id: {} and user_id: {}",
                        organization_id,
                        user_id
                    );
                    serde_json::to_value(row).map_err(ErrorData::from)
                }
                Err(err) => {
                    log::error!(
                        "Error adding User for organization_id: {} and user_id: {}: {:#?}",
                        organization_id,
                        user_id,
                        err
                    );
                    Err(ErrorData {
                        message: String::from("USER_NOT_ADDED"),
                        data: Value::Null,
                        code: -32600,
                    })
                }
            }
        } else {
            error!("Password and Confirm Password do not match.");
            Err(ErrorData {
                message: String::from("Password and Confirm Password do not match "),
                data: Value::Null,
                code: -32600,
            })
        }
    } else {
        error!("Password is not valid.");
        Err(ErrorData {
                    message: String::from("Password is not valid. It must contain at least one numeric, one alphabetic, and one special character and be at least 6 characters long"),
                    data: Value::Null,
                    code: -32600,
                })
    }
}

pub async fn update_user_password(
    app_state: AppState,
    organization_id: String,
    user_id: String,
    password: String,
    confirm_password: String,
    last_updated_input: LastUpdatedInput,
) -> Result<Value, ErrorData> {
    let is_valid_password = ValidPassword::parse(&password).is_ok();
    let store = user_store(&app_state.write_pool, &organization_id).await?;
    if is_valid_password {
        let hash = hash_password(&password)
            .map_err(|e| ErrorData::new(-32603, &format!("Password hashing failed: {}", e)))?;
        let is_valid = verify_password(&hash, &confirm_password).unwrap_or(false);

        if is_valid {
            let user_db = sqlx::query_as::<_, Stream>(
                "SELECT id,stream_id,version,(data->>'created_by') AS created_by, (data->>'created_at') AS created_at,data FROM user_table_state WHERE id = ?",
                    )
                    .bind(user_id.clone())
                    .fetch_optional(&app_state.read_pool.clone())
                    .await?;

            let created_by = user_db
                .as_ref()
                .map(|org| org.created_by.clone())
                .unwrap_or_default();
            let stream_id = user_db
                .as_ref()
                .map(|org| org.stream_id.clone())
                .unwrap_or_default();
            let created_at = user_db
                .as_ref()
                .map(|org| org.created_at)
                .unwrap_or_default();

            let command_password = UpdateUserPassword {
                id: user_id.clone(),
                created_by: created_by.clone(),
                updated_by: user_id.clone(),
                created_at,
                last_updated: Utc::now(),
                password: hash.clone(),
            };

            let events_password = make_handler(
                &USER_AGGREGATE,
                &store,
                &UserCommand::UpdateUserPassword(command_password.clone()),
                &stream_id,
                &EventsReadRange::AllEvents,
                &ExpectedVersion::Any,
            )
            .await?;

            let _ = process_user_events(
                app_state.read_pool.clone(),
                command_password.id.clone(),
                stream_id.clone(),
                events_password,
            )
            .await;

            let row = sync(
                last_updated_input.clone(),
                organization_id.clone(),
                app_state.read_pool.clone(),
            )
            .await?;

            if true {
                log::info!(
                    "User updated successfully for organization_id: {} and user_id: {}",
                    organization_id,
                    user_id
                );
                serde_json::to_value(row).map_err(ErrorData::from)
            } else {
                log::error!(
                    "Error updating User for organization_id: {} and user_id: {}",
                    organization_id,
                    user_id,
                );
                Err(ErrorData {
                    message: String::from("USER_NOT_UPDATED"),
                    data: Value::Null,
                    code: -32600,
                })
            }
        } else {
            error!("Password and Confirm Password do not match.");
            Err(ErrorData {
                message: String::from("Password and Confirm Password do not match "),
                data: Value::Null,
                code: -32600,
            })
        }
    } else {
        error!("Password is not valid.");
        Err(ErrorData {
                message: String::from("Password is not valid. It must contain at least one numeric, one alphabetic, and one special character and be at least 6 characters long"),
                data: Value::Null,
                code: -32600,
            })
    }
}
pub async fn update_user(
    params: UserUpdate,
    app_state: AppState,
    organization_id: String,
    user_id: String,
) -> Result<Value, ErrorData> {
    log::info!(
        "Updating user for organization_id: {} and user_id: {}",
        organization_id,
        user_id
    );

    let store = user_store(&app_state.write_pool, &organization_id).await?;
    let user_db = sqlx::query_as::<_, Stream>(
            "SELECT id,stream_id,version,(data->>'created_by') AS created_by, (data->>'created_at') AS created_at,data FROM user_table_state WHERE id = ?",
                )
                .bind(params.id.clone())
                .fetch_optional(&app_state.read_pool.clone())
                .await?;

    let id = user_db
        .as_ref()
        .map(|org| org.id.clone())
        .unwrap_or_default();
    let created_by = user_db
        .as_ref()
        .map(|org| org.created_by.clone())
        .unwrap_or_default();
    let stream_id = user_db
        .as_ref()
        .map(|org| org.stream_id.clone())
        .unwrap_or_default();
    let created_at = user_db
        .as_ref()
        .map(|org| org.created_at)
        .unwrap_or_default();

    let data = user_db
        .as_ref()
        .map(|org| org.data.clone())
        .unwrap_or_default();
    let data_obj: serde_json::Value = serde_json::from_value(data.clone()).unwrap_or_default();

    let address: AddressInput =
        serde_json::from_value(data_obj["address"].clone()).unwrap_or_default();
    let user: UserInput = serde_json::from_value(data_obj["user"].clone()).unwrap_or_default();
    let government_info: GovInfoInput =
        serde_json::from_value(data_obj["government_info"].clone()).unwrap_or_default();
    let phone: ContactInput = serde_json::from_value(data_obj["phone"].clone()).unwrap_or_default();

    if params.address == address
        && params.goverment_info == government_info
        && params.phone == phone
        && params.user == user
    {
        return Err(
            anyhow::anyhow!("User Details Not Updated. No changes made in the form.").into(),
        );
    }

    if params.address != address {
        info!("Updating user address");
        let command_address = UpdateUserAddress {
            id: id.clone(),
            address: params.address.clone(),
            created_by: created_by.clone(),
            updated_by: user_id.clone(),
            created_at,
            last_updated: Utc::now(),
        };

        let events_address = make_handler(
            &USER_AGGREGATE,
            &store,
            &UserCommand::UpdateUserAddress(command_address.clone()),
            &stream_id,
            &EventsReadRange::AllEvents,
            &ExpectedVersion::Any,
        )
        .await?;

        let _ = process_user_events(
            app_state.read_pool.clone(),
            command_address.id.clone(),
            stream_id.clone(),
            events_address,
        )
        .await;
    }

    if params.user != user {
        info!("Updating user details");
        let command_user = UpdateUserDetails {
            id: id.clone(),
            created_by: created_by.clone(),
            updated_by: user_id.clone(),
            created_at,
            last_updated: Utc::now(),
            user: params.user.clone(),
        };

        let events_user = make_handler(
            &USER_AGGREGATE,
            &store,
            &UserCommand::UpdateUserDetails(command_user.clone()),
            &stream_id,
            &EventsReadRange::AllEvents,
            &ExpectedVersion::Any,
        )
        .await?;

        let _ = process_user_events(
            app_state.read_pool.clone(),
            command_user.id.clone(),
            stream_id.clone(),
            events_user,
        )
        .await;
    }

    if params.goverment_info != government_info {
        info!("Updating user government_info");
        let command_gov = UpdateUserGovDetails {
            id: id.clone(),
            created_by: created_by.clone(),
            updated_by: user_id.clone(),
            created_at,
            last_updated: Utc::now(),
            government_info: params.goverment_info.clone(),
        };

        let events_gov = make_handler(
            &USER_AGGREGATE,
            &store,
            &UserCommand::UpdateUserGovDetails(command_gov.clone()),
            &stream_id,
            &EventsReadRange::AllEvents,
            &ExpectedVersion::Any,
        )
        .await?;

        let _ = process_user_events(
            app_state.read_pool.clone(),
            command_gov.id.clone(),
            stream_id.clone(),
            events_gov,
        )
        .await;
    }

    if params.phone != phone {
        info!("Updating user contact details");
        let command_contact = UpdateUserContact {
            id: id.clone(),
            created_by: created_by.clone(),
            updated_by: user_id.clone(),
            created_at,
            last_updated: Utc::now(),
            phone: params.phone.clone(),
        };

        let events_contact = make_handler(
            &USER_AGGREGATE,
            &store,
            &UserCommand::UpdateUserContact(command_contact.clone()),
            &stream_id,
            &EventsReadRange::AllEvents,
            &ExpectedVersion::Any,
        )
        .await?;

        let _ = process_user_events(
            app_state.read_pool.clone(),
            command_contact.id.clone(),
            stream_id.clone(),
            events_contact,
        )
        .await;
    }

    user_doctor_update(
        &app_state.clone(),
        organization_id.clone(),
        user_id.clone(),
        params.clone(),
    )
    .await?;
    user_staff_update(
        &app_state.clone(),
        organization_id.clone(),
        user_id.clone(),
        params.clone(),
    )
    .await?;
    let row = sync(
        params.last_updated_input.clone(),
        organization_id.clone(),
        app_state.read_pool.clone(),
    )
    .await?;

    if true {
        log::info!(
            "User updated successfully for organization_id: {} and user_id: {}",
            organization_id,
            user_id
        );
        serde_json::to_value(row).map_err(ErrorData::from)
    } else {
        log::error!(
            "Error updating User for organization_id: {} and user_id: {}",
            organization_id,
            user_id,
        );
        Err(ErrorData {
            message: String::from("USER_NOT_UPDATED"),
            data: Value::Null,
            code: -32600,
        })
    }
}

pub async fn delete_user(
    params: UserDelete,
    app_state: AppState,
    organization_id: String,
    user_id: String,
) -> Result<Value, ErrorData> {
    log::info!(
        "Deleting user with id: {} for organization_id: {} and user_id: {}",
        params.id,
        organization_id,
        user_id
    );

    let store = user_store(&app_state.write_pool, &organization_id).await?;
    let user_db = sqlx::query_as::<_, Stream>(
        "SELECT id,stream_id,version,(data->>'created_by') AS created_by, (data->>'created_at') AS created_at,data FROM user_table_state WHERE id = ?",
    )
    .bind(params.id.clone())
    .fetch_optional(&app_state.read_pool.clone())
    .await?;

    let created_by = user_db
        .as_ref()
        .map(|org| org.created_by.clone())
        .unwrap_or_default();
    let stream_id = user_db
        .as_ref()
        .map(|org| org.stream_id.clone())
        .unwrap_or_default();
    let created_at = user_db
        .as_ref()
        .map(|org| org.created_at)
        .unwrap_or_default();
    let command = DeleteUser {
        id: params.id.clone(),
        org_id: organization_id.clone(),
        created_by,
        updated_by: user_id.clone(),
        created_at,
        last_updated: Utc::now(),
    };

    let events = make_handler(
        &USER_AGGREGATE,
        &store,
        &UserCommand::DeleteUser(command.clone()),
        &stream_id,
        &EventsReadRange::AllEvents,
        &ExpectedVersion::Any,
    )
    .await?;
    let res = process_user_events(
        app_state.read_pool.clone(),
        command.id.clone(),
        stream_id.clone(),
        events,
    )
    .await;

    user_doctor_delete(
        &app_state.clone(),
        organization_id.clone(),
        user_id.clone(),
        params.clone(),
    )
    .await?;
    user_staff_delete(
        &app_state.clone(),
        organization_id.clone(),
        user_id.clone(),
        params.clone(),
    )
    .await?;

    let row = sync(
        params.last_updated_input.clone(),
        organization_id.clone(),
        app_state.read_pool.clone(),
    )
    .await?;

    match res {
        Ok(_) => {
            log::info!(
                "User with id: {} deleted successfully for organization_id: {} and user_id: {}",
                params.id.clone(),
                organization_id,
                user_id
            );
            serde_json::to_value(row).map_err(ErrorData::from)
        }
        Err(err) => {
            log::error!(
                "Error deleting User with id: {} for organization_id: {} and user_id: {}: {:#?}",
                params.id,
                organization_id,
                user_id,
                err
            );
            Err(ErrorData {
                message: String::from("STAFF_NOT_DELETED"),
                data: Value::Null,
                code: -32600,
            })
        }
    }
}

pub async fn user_doctor_delete(
    app_state: &AppState,
    organization_id: String,
    user_id: String,
    params: UserDelete,
) -> Result<(), Error> {
    let store = doctor_store(&app_state.write_pool, &organization_id).await?;

    let doctor_db = sqlx::query_as::<_, Stream>(
        "SELECT  id, stream_id, version, (data->>'created_by') AS created_by, (data->>'created_at') AS created_at, data FROM doctor_table_state WHERE data->>'user'->>'id'=$1"
    )
    .bind(params.id.clone())
    .fetch_optional(&app_state.read_pool.clone())
    .await?;

    let id = doctor_db
        .as_ref()
        .map(|doctor| doctor.id.clone())
        .unwrap_or_default();
    let created_by = doctor_db
        .as_ref()
        .map(|doctor| doctor.created_by.clone())
        .unwrap_or_default();
    let stream_id = doctor_db
        .as_ref()
        .map(|doctor| doctor.stream_id.clone())
        .unwrap_or_default();
    let created_at = doctor_db
        .as_ref()
        .map(|doctor| doctor.created_at)
        .unwrap_or_default();

    let data = doctor_db
        .as_ref()
        .map(|doctor| doctor.data.clone())
        .unwrap_or_default();

    let data_obj: serde_json::Value = serde_json::from_value(data.clone()).unwrap_or_default();
    let user: UserState = serde_json::from_value(data_obj["user"].clone()).unwrap_or_default();

    if params.id == user.id {
        let command = DeleteDoctor {
            id: id.clone(),
            created_by,
            updated_by: user_id.clone(),
            created_at,
            last_updated: Utc::now(),
            org_id: organization_id.clone(),
        };

        let events = make_handler(
            &DOCTOR_AGGREGATE,
            &store,
            &DoctorCommand::DeleteDoctor(command.clone()),
            &stream_id,
            &EventsReadRange::AllEvents,
            &ExpectedVersion::Any,
        )
        .await?;

        let _ =
            process_doctor_events(app_state.read_pool.clone(), command.id, stream_id, events).await;
    }

    Ok(())
}

pub async fn user_doctor_update(
    app_state: &AppState,
    organization_id: String,
    user_id: String,
    params: UserUpdate,
) -> Result<(), Error> {
    let store = doctor_store(&app_state.write_pool, &organization_id).await?;

    let doctor_db = sqlx::query_as::<_, Stream>(
        "SELECT  id, stream_id, version, (data->>'created_by') AS created_by, (data->>'created_at') AS created_at, data FROM doctor_table_state WHERE data->>'user'->>'id'=$1"
    )
    .bind(params.id.clone())
    .fetch_optional(&app_state.read_pool.clone())
    .await?;

    let id = doctor_db
        .as_ref()
        .map(|doctor| doctor.id.clone())
        .unwrap_or_default();
    let created_by = doctor_db
        .as_ref()
        .map(|doctor| doctor.created_by.clone())
        .unwrap_or_default();
    let stream_id = doctor_db
        .as_ref()
        .map(|doctor| doctor.stream_id.clone())
        .unwrap_or_default();
    let created_at = doctor_db
        .as_ref()
        .map(|doctor| doctor.created_at)
        .unwrap_or_default();

    let data = doctor_db
        .as_ref()
        .map(|doctor| doctor.data.clone())
        .unwrap_or_default();

    let data_obj: serde_json::Value = serde_json::from_value(data.clone()).unwrap_or_default();
    let user: UserState = serde_json::from_value(data_obj["user"].clone()).unwrap_or_default();
    let doctor_role: Vec<DoctorType> =
        serde_json::from_value(data_obj["doctor_role"].clone()).unwrap_or_default();
    let doctor_register_number: String =
        serde_json::from_value(data_obj["doctor_register_number"].clone()).unwrap_or_default();
    let doctor_department: String =
        serde_json::from_value(data_obj["doctor_department"].clone()).unwrap_or_default();
    let doctor_speciality: String =
        serde_json::from_value(data_obj["doctor_speciality"].clone()).unwrap_or_default();
    let emergency: bool = serde_json::from_value(data_obj["emergency"].clone()).unwrap_or_default();

    if params.id == user.id {
        let command = UpdateDoctor {
            id,
            user,
            org_id: organization_id.clone(),
            created_by: created_by.clone(),
            updated_by: user_id.clone(),
            created_at,
            last_updated: Utc::now(),
            doctor_role,
            doctor_register_number,
            doctor_department,
            doctor_speciality,
            emergency,
        };

        let events = make_handler(
            &DOCTOR_AGGREGATE,
            &store,
            &DoctorCommand::UpdateDoctor(Box::new(command.clone())),
            &stream_id.clone(),
            &EventsReadRange::AllEvents,
            &ExpectedVersion::Any,
        )
        .await?;
        let _ = process_doctor_events(
            app_state.read_pool.clone(),
            command.id,
            stream_id.clone(),
            events,
        )
        .await;
    }
    Ok(())
}
pub async fn user_staff_delete(
    app_state: &AppState,
    organization_id: String,
    user_id: String,
    params: UserDelete,
) -> Result<(), Error> {
    let store = staff_store(&app_state.write_pool, &organization_id).await?;

    let staff_db = sqlx::query_as::<_, Stream>(
        "SELECT  id, stream_id, version, (data->>'created_by') AS created_by, (data->>'created_at') AS created_at, data FROM staff_table_state WHERE data->>'user'->>'id'=$1"
    )
    .bind(params.id.clone())
    .fetch_optional(&app_state.read_pool.clone())
    .await?;

    let id = staff_db
        .as_ref()
        .map(|staff| staff.id.clone())
        .unwrap_or_default();
    let created_by = staff_db
        .as_ref()
        .map(|staff| staff.created_by.clone())
        .unwrap_or_default();
    let stream_id = staff_db
        .as_ref()
        .map(|staff| staff.stream_id.clone())
        .unwrap_or_default();
    let created_at = staff_db
        .as_ref()
        .map(|staff| staff.created_at)
        .unwrap_or_default();

    let data = staff_db
        .as_ref()
        .map(|staff| staff.data.clone())
        .unwrap_or_default();

    let data_obj: serde_json::Value = serde_json::from_value(data.clone()).unwrap_or_default();
    let user: UserState = serde_json::from_value(data_obj["user"].clone()).unwrap_or_default();

    if params.id == user.id {
        let command = DeleteStaff {
            id: id.clone(),
            org_id: organization_id.clone(),
            created_by,
            updated_by: user_id.clone(),
            created_at,
            last_updated: Utc::now(),
        };

        let events = make_handler(
            &STAFF_AGGREGATE,
            &store,
            &StaffCommand::DeleteStaff(command.clone()),
            &stream_id,
            &EventsReadRange::AllEvents,
            &ExpectedVersion::Any,
        )
        .await?;
        let _ = process_staff_events(
            app_state.read_pool.clone(),
            command.id.clone(),
            stream_id.clone(),
            events,
        )
        .await;
    }

    Ok(())
}

pub async fn user_staff_update(
    app_state: &AppState,
    organization_id: String,
    user_id: String,
    params: UserUpdate,
) -> Result<(), Error> {
    let store = staff_store(&app_state.write_pool, &organization_id).await?;

    let staff_db = sqlx::query_as::<_, Stream>(
        "SELECT  id, stream_id, version, (data->>'created_by') AS created_by, (data->>'created_at') AS created_at, data FROM staff_table_state WHERE data->>'user'->>'id'=$1"
    )
    .bind(params.id.clone())
    .fetch_optional(&app_state.read_pool.clone())
    .await?;

    let id = staff_db
        .as_ref()
        .map(|staff| staff.id.clone())
        .unwrap_or_default();
    let created_by = staff_db
        .as_ref()
        .map(|staff| staff.created_by.clone())
        .unwrap_or_default();
    let stream_id = staff_db
        .as_ref()
        .map(|staff| staff.stream_id.clone())
        .unwrap_or_default();
    let created_at = staff_db
        .as_ref()
        .map(|staff| staff.created_at)
        .unwrap_or_default();

    let data = staff_db
        .as_ref()
        .map(|staff| staff.data.clone())
        .unwrap_or_default();

    let data_obj: serde_json::Value = serde_json::from_value(data.clone()).unwrap_or_default();
    let user: UserState = serde_json::from_value(data_obj["user"].clone()).unwrap_or_default();

    let staff_department: Vec<StaffType> =
        serde_json::from_value(data_obj["staff_department"].clone()).unwrap_or_default();

    let emergency: bool = serde_json::from_value(data_obj["emergency"].clone()).unwrap_or_default();

    if params.id == user.id {
        let command = UpdateStaff {
            id,
            org_id: organization_id.clone(),
            created_by,
            updated_by: user_id.clone(),
            created_at,
            last_updated: Utc::now(),
            staff_department,
            user,
            emergency,
        };
        let events = make_handler(
            &STAFF_AGGREGATE,
            &store,
            &StaffCommand::UpdateStaff(Box::new(command.clone())),
            &stream_id,
            &EventsReadRange::AllEvents,
            &ExpectedVersion::Any,
        )
        .await?;
        let _ = process_staff_events(
            app_state.read_pool.clone(),
            command.id.clone(),
            stream_id.clone(),
            events,
        )
        .await;
    }

    Ok(())
}
