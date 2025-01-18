use crate::{
    app_state::AppState,
    doc::{staff::helper::process_staff_events, syncs::sync},
    method::convention::ErrorData,
};
use chrono::Utc;
use common::stream::Stream;
use cosmo_store::types::{event_read_range::EventsReadRange, expected_version::ExpectedVersion};
use cosmo_store_util::aggregate::make_handler;
use serde_json::Value;
use sqlx::Row;
use sqlx::Sqlite;
use staff::{
    domain::{
        staff_aggregate::STAFF_AGGREGATE,
        staff_commands::{CreateStaff, DeleteStaff, StaffCommand, UpdateStaff},
    },
    dto::{
        staff_add::StaffAdd, staff_delete::StaffDelete, staff_type::StaffType,
        staff_update::StaffUpdate,
    },
};
use user::domain::user_domain::UserState;
use utils::store_helper::staff_store;
use uuid::Uuid;

pub async fn add_staff(
    params: StaffAdd,
    app_state: AppState,
    organization_id: String,
    user_id: String,
) -> Result<Value, ErrorData> {
    log::info!(
        "Adding staff for organization_id: {} and user_id: {}",
        organization_id,
        user_id
    );

    let json_data_result: Option<Value> =
        sqlx::query_scalar::<_, Option<Value>>("SELECT data FROM user_table_state WHERE id = ?")
            .bind(params.user_id.clone())
            .fetch_one(&app_state.read_pool.clone())
            .await?;

    let json_data = match json_data_result {
        Some(data) => data,
        None => serde_json::Value::Null,
    };

    let user_state: UserState = serde_json::from_value(json_data)
        .map_err(|e| anyhow::anyhow!("Failed to parse JSON: {}", e))?;

    let query = sqlx::query::<Sqlite>(
        "
    SELECT auth.mobile_number
    FROM auth
    INNER JOIN roles ON auth.id = roles.user_id
    WHERE roles.role = 'staff'
    ",
    );

    let mobile_number: Option<String> = query
        .fetch_optional(&app_state.read_pool.clone())
        .await?
        .map(|row| row.get("mobile_number"));

    if let Some(get_mobile_number) = mobile_number {
        if get_mobile_number == user_state.phone.number {
            return Err(ErrorData {
                message: String::from("User is already in use."),
                data: Value::Null,
                code: -32600,
            });
        }
    }

    let store = staff_store(&app_state.write_pool, &organization_id).await?;
    let stream_id = format!("Staff::{}", Uuid::new_v4().as_simple());
    let command = CreateStaff {
        id: Uuid::new_v4().as_simple().to_string(),
        org_id: organization_id.clone(),
        user: user_state.clone(),
        created_by: user_id.clone(),
        updated_by: user_id.clone(),
        created_at: Utc::now(),
        last_updated: Utc::now(),
        staff_department: params.staff_department,
        emergency: params.emergency,
    };
    let events = make_handler(
        &STAFF_AGGREGATE,
        &store,
        &StaffCommand::CreateStaff(Box::new(command.clone())),
        &stream_id,
        &EventsReadRange::AllEvents,
        &ExpectedVersion::Any,
    )
    .await?;
    let res = process_staff_events(
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
                "Staff added successfully for organization_id: {} and user_id: {}",
                organization_id,
                user_id
            );
            serde_json::to_value(row).map_err(ErrorData::from)
        }
        Err(err) => {
            log::error!(
                "Error adding Staff for organization_id: {} and user_id: {}: {:#?}",
                organization_id,
                user_id,
                err
            );
            Err(ErrorData {
                message: String::from("STAFF_NOT_ADDED"),
                data: Value::Null,
                code: -32600,
            })
        }
    }
}
pub async fn update_staff(
    params: StaffUpdate,
    app_state: AppState,
    organization_id: String,
    user_id: String,
) -> Result<Value, ErrorData> {
    log::info!(
        "Updating staff for organization_id: {} and user_id: {}",
        organization_id,
        user_id
    );

    let json_data_result: Option<Value> =
        sqlx::query_scalar::<_, Option<Value>>("SELECT data FROM user_table_state WHERE id = ?")
            .bind(params.input.user_id.clone())
            .fetch_one(&app_state.read_pool.clone())
            .await?;

    let json_data = match json_data_result {
        Some(data) => data,
        None => serde_json::Value::Null,
    };

    let user_state: UserState = serde_json::from_value(json_data)
        .map_err(|e| anyhow::anyhow!("Failed to parse JSON: {}", e))?;

    let store = staff_store(&app_state.write_pool, &organization_id).await?;
    let staff_db = sqlx::query_as::<_, Stream>(
            "SELECT id,stream_id,version,(data->>'created_by') AS created_by, (data->>'created_at') AS created_at,data FROM staff_table_state WHERE id = ?",
                )
                .bind(params.id.clone())
                .fetch_optional(&app_state.read_pool.clone())
                .await?;

    let created_by = staff_db
        .as_ref()
        .map(|org| org.created_by.clone())
        .unwrap_or_default();
    let stream_id = staff_db
        .as_ref()
        .map(|org| org.stream_id.clone())
        .unwrap_or_default();
    let created_at = staff_db
        .as_ref()
        .map(|org| org.created_at)
        .unwrap_or_default();

    let data = staff_db
        .as_ref()
        .map(|org| org.data.clone())
        .unwrap_or_default();
    let data_obj: serde_json::Value = serde_json::from_value(data.clone()).unwrap_or_default();

    let staff_department: Vec<StaffType> =
        serde_json::from_value(data_obj["staff_department"].clone()).unwrap_or_default();

    let emergency: bool = serde_json::from_value(data_obj["emergency"].clone()).unwrap_or_default();

    if params.input.staff_department == staff_department && params.input.emergency == emergency {
        return Err(
            anyhow::anyhow!("Staff Details Not Updated. No changes made in the form.").into(),
        );
    }

    if params.input.staff_department != staff_department || params.input.emergency != emergency {
        let command = UpdateStaff {
            id: params.id,
            org_id: organization_id.clone(),
            created_by,
            updated_by: user_id.clone(),
            created_at,
            last_updated: Utc::now(),
            staff_department: params.input.staff_department,
            user: user_state.clone(),
            emergency: params.input.emergency,
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

    let row = sync(
        params.input.last_updated_input.clone(),
        organization_id.clone(),
        app_state.read_pool.clone(),
    )
    .await?;

    if true {
        log::info!(
            "Staff updated successfully for organization_id: {} and user_id: {}",
            organization_id,
            user_id
        );
        serde_json::to_value(row).map_err(ErrorData::from)
    } else {
        log::error!(
            "Error updating Staff for organization_id: {} and user_id: {}",
            organization_id,
            user_id,
        );
        Err(ErrorData {
            message: String::from("STAFF_NOT_UPDATED"),
            data: Value::Null,
            code: -32600,
        })
    }
}

pub async fn delete_staff(
    params: StaffDelete,
    app_state: AppState,
    organization_id: String,
    user_id: String,
) -> Result<Value, ErrorData> {
    log::info!(
        "Deleting staff with id: {} for organization_id: {} and user_id: {}",
        params.id,
        organization_id,
        user_id
    );

    let store = staff_store(&app_state.write_pool, &organization_id).await?;
    let staff_db = sqlx::query_as::<_, Stream>(
        "SELECT id,stream_id,version,(data->>'created_by') AS created_by, (data->>'created_at') AS created_at,data FROM staff_table_state WHERE id = ?",
    )
    .bind(params.id.clone())
    .fetch_optional(&app_state.read_pool.clone())
    .await?;

    let created_by = staff_db
        .as_ref()
        .map(|org| org.created_by.clone())
        .unwrap_or_default();
    let stream_id = staff_db
        .as_ref()
        .map(|org| org.stream_id.clone())
        .unwrap_or_default();
    let created_at = staff_db
        .as_ref()
        .map(|org| org.created_at)
        .unwrap_or_default();
    let command = DeleteStaff {
        id: params.id.clone(),
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
    let res = process_staff_events(
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
            log::info!(
                "Staff with id: {} deleted successfully for organization_id: {} and user_id: {}",
                params.id.clone(),
                organization_id,
                user_id
            );
            serde_json::to_value(row).map_err(ErrorData::from)
        }
        Err(err) => {
            log::error!(
                "Error deleting Staff with id: {} for organization_id: {} and user_id: {}: {:#?}",
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
