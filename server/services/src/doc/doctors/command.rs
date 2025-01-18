use super::helper::process_doctor_events;
use crate::{app_state::AppState, doc::syncs::sync, method::convention::ErrorData};
use anyhow::Result;
use common::stream::Stream;
use cosmo_store::types::{event_read_range::EventsReadRange, expected_version::ExpectedVersion};
use cosmo_store_util::aggregate::make_handler;
use doctor::{
    domain::{
        doctor_aggregate::DOCTOR_AGGREGATE,
        doctor_commands::{CreateDoctor, DeleteDoctor, DoctorCommand, UpdateDoctor},
    },
    dto::{
        doctor_add::DoctorAdd, doctor_delete::DoctorDelete, doctor_type::DoctorType,
        doctor_update::DoctorUpdate,
    },
};
use log::{error, info};
use serde_json::Value;
use sqlx::Row;
use sqlx::{types::chrono::Utc, Sqlite};
use user::domain::user_domain::UserState;
use utils::store_helper::doctor_store;
use uuid::Uuid;

pub async fn add_doctors(
    params: DoctorAdd,
    app_state: AppState,
    organization_id: String,
    user_id: String,
) -> Result<Value, ErrorData> {
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

    info!("Starting add_doctors function...");

    let store = doctor_store(&app_state.write_pool, &organization_id).await?;
    let stream_id = format!("Doctor::{}", uuid::Uuid::new_v4().as_simple());

    let query = sqlx::query::<Sqlite>(
        "
    SELECT auth.mobile_number
    FROM auth
    INNER JOIN roles ON auth.id = roles.user_id
    WHERE roles.role = 'doctor'
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

    let command = CreateDoctor {
        id: Uuid::new_v4().as_simple().to_string(),
        user: user_state,
        org_id: organization_id.clone(),
        created_by: user_id.clone(),
        updated_by: user_id.clone(),
        created_at: Utc::now(),
        last_updated: Utc::now(),
        doctor_role: params.doctor_role,
        doctor_register_number: params.docter_register_number,
        doctor_department: params.doctor_department,
        doctor_speciality: params.doctor_speciality,
        emergency: params.emergency,
    };
    let events = make_handler(
        &DOCTOR_AGGREGATE,
        &store,
        &DoctorCommand::CreateDoctor(Box::new(command.clone())),
        &stream_id,
        &EventsReadRange::AllEvents,
        &ExpectedVersion::Any,
    )
    .await?;
    let res =
        process_doctor_events(app_state.read_pool.clone(), command.id, stream_id, events).await;

    let row = sync(
        params.last_updated_input,
        organization_id.clone(),
        app_state.read_pool.clone(),
    )
    .await?;

    match res {
        anyhow::Result::Ok(_) => {
            info!("Doctor events processed successfully.");
            serde_json::to_value(row).map_err(ErrorData::from)
        }
        Err(err) => {
            error!("Error processing doctor events: {:?}", err);
            Err(ErrorData {
                message: String::from("DOCTOR_NOT_ADDED"),
                data: Value::Null,
                code: -32600,
            })
        }
    }
}

pub async fn update_doctors(
    params: DoctorUpdate,
    app_state: AppState,
    organization_id: String,
    user_id: String,
) -> Result<Value, ErrorData> {
    let store = doctor_store(&app_state.write_pool, &organization_id).await?;

    let doctor_db = sqlx::query_as::<_, Stream>(
                "SELECT id,stream_id,version,(data->>'created_by') AS created_by, (data->>'created_at') AS created_at,data FROM doctor_table_state WHERE id = ?",
            )
            .bind(params.id.clone())
            .fetch_optional(&app_state.read_pool.clone())
            .await?;

    let created_by = doctor_db
        .as_ref()
        .map(|org| org.created_by.clone())
        .unwrap_or_default();
    let stream_id = doctor_db
        .as_ref()
        .map(|org| org.stream_id.clone())
        .unwrap_or_default();
    let created_at = doctor_db
        .as_ref()
        .map(|org| org.created_at)
        .unwrap_or_default();

    let data = doctor_db
        .as_ref()
        .map(|org| org.data.clone())
        .unwrap_or_default();

    let data_obj: serde_json::Value = serde_json::from_value(data.clone()).unwrap_or_default();

    let doctor_role: Vec<DoctorType> =
        serde_json::from_value(data_obj["doctor_role"].clone()).unwrap_or_default();
    let doctor_register_number: String =
        serde_json::from_value(data_obj["doctor_register_number"].clone()).unwrap_or_default();
    let doctor_department: String =
        serde_json::from_value(data_obj["doctor_department"].clone()).unwrap_or_default();
    let doctor_speciality: String =
        serde_json::from_value(data_obj["doctor_speciality"].clone()).unwrap_or_default();
    let emergency: bool = serde_json::from_value(data_obj["emergency"].clone()).unwrap_or_default();

    let json_data_result: Option<Value> =
        sqlx::query_scalar::<_, Option<Value>>("SELECT data FROM user_table_state WHERE id = ?")
            .bind(params.doctor.user_id.clone())
            .fetch_one(&app_state.read_pool.clone())
            .await?;

    let json_data = match json_data_result {
        Some(data) => data,
        None => serde_json::Value::Null,
    };

    let user_state: UserState = serde_json::from_value(json_data)
        .map_err(|e| anyhow::anyhow!("Failed to parse JSON: {}", e))?;

    if params.doctor.doctor_role == doctor_role
        && params.doctor.docter_register_number == doctor_register_number
        && params.doctor.doctor_department == doctor_department
        && params.doctor.doctor_speciality == doctor_speciality
        && params.doctor.emergency == emergency
    {
        return Err(
            anyhow::anyhow!("Doctor Details Not Updated. No changes made in the form.").into(),
        );
    }

    if params.doctor.doctor_role != doctor_role
        || params.doctor.docter_register_number != doctor_register_number
        || params.doctor.doctor_department != doctor_department
        || params.doctor.doctor_speciality != doctor_speciality
        || params.doctor.emergency != emergency
    {
        let command = UpdateDoctor {
            id: params.id,
            user: user_state,
            org_id: organization_id.clone(),
            created_by: created_by.clone(),
            updated_by: user_id.clone(),
            created_at,
            last_updated: Utc::now(),
            doctor_role: params.doctor.doctor_role,
            doctor_register_number: params.doctor.docter_register_number,
            doctor_department: params.doctor.doctor_department,
            doctor_speciality: params.doctor.doctor_speciality,
            emergency: params.doctor.emergency,
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

    let row = sync(
        params.doctor.last_updated_input,
        organization_id.clone(),
        app_state.read_pool.clone(),
    )
    .await?;

    if true {
        info!("Doctor updated successfully.");
        serde_json::to_value(row).map_err(ErrorData::from)
    } else {
        error!("Error updating doctor");
        Err(ErrorData {
            message: String::from("DOCTOR_NOT_UPDATED"),
            data: Value::Null,
            code: -32600,
        })
    }
}

pub async fn delete_doctors(
    params: DoctorDelete,
    app_state: AppState,
    organization_id: String,
    user_id: String,
) -> Result<Value, ErrorData> {
    let store = doctor_store(&app_state.write_pool, &organization_id).await?;
    let doctor_db = sqlx::query_as::<_, Stream>(
        "SELECT id,stream_id,version,(data->>'created_by') AS created_by, (data->>'created_at') AS created_at,data FROM doctor_table_state WHERE id = ?",
    )
    .bind(params.id.clone())
    .fetch_optional(&app_state.read_pool.clone())
    .await?;

    let created_by = doctor_db
        .as_ref()
        .map(|org| org.created_by.clone())
        .unwrap_or_default();
    let stream_id = doctor_db
        .as_ref()
        .map(|org| org.stream_id.clone())
        .unwrap_or_default();
    let created_at = doctor_db
        .as_ref()
        .map(|org| org.created_at)
        .unwrap_or_default();

    let command = DeleteDoctor {
        id: params.id,
        created_by,
        updated_by: user_id,
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

    let res =
        process_doctor_events(app_state.read_pool.clone(), command.id, stream_id, events).await;

    let row = sync(
        params.last_updated_input,
        organization_id.clone(),
        app_state.read_pool.clone(),
    )
    .await?;
    match res {
        anyhow::Result::Ok(_) => serde_json::to_value(row).map_err(ErrorData::from),
        Err(err) => {
            error!("{:#?}", err);
            Err(ErrorData {
                message: String::from("DOCTOR_NOT_DELETED"),
                data: Value::Null,
                code: -32600,
            })
        }
    }
}
