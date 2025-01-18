use crate::{
    app_state::AppState,
    doc::{
        patients::get_patient_id::{doctor_id_fetch, patient_id_fetch},
        syncs::sync,
    },
    method::convention::ErrorData,
};
use anyhow::Result;
use appointment::{
    domain::{
        appointment_aggregate::APPOINTMENTS_AGGREGATE,
        appointment_commands::{
            AppointmentCommands, CreateAppointment, DeleteAppointment, UpdateAppointment,
        },
    },
    dto::{
        appointment_add::AppointmentAdd, appointment_delete::AppointmentDelete,
        appointment_update::AppointmentUpdate,
    },
};

use common::stream::Stream;
use cosmo_store::types::{event_read_range::EventsReadRange, expected_version::ExpectedVersion};
use cosmo_store_util::aggregate::make_handler;
use log::{error, info};
use serde_json::Value;
use sqlx::types::chrono::Utc;
use utils::store_helper::appointment_store;
use uuid::{self, Uuid};

use super::helper::process_appointment_events;

pub async fn add_appointments(
    params: AppointmentAdd,
    app_state: AppState,
    organization_id: String,
    user_id: String,
    service_location_id: String,
) -> Result<Value, ErrorData> {
    let patient_id = patient_id_fetch(app_state.clone(), params.patient_id.clone()).await?;

    if patient_id {
        let doctor_id = doctor_id_fetch(app_state.clone(), params.doctor_id.clone()).await?;

        if doctor_id {
            info!("Starting add_appointments function...");

            let store = appointment_store(&app_state.write_pool, &organization_id).await?;
            let stream_id = format!("Appointment::{}", uuid::Uuid::new_v4().as_simple());
            let command = CreateAppointment {
                id: Uuid::new_v4().as_simple().to_string(),
                org_id: organization_id.clone(),
                service_location_id,
                created_by: user_id.clone(),
                updated_by: user_id.clone(),
                created_at: Utc::now(),
                last_updated: Utc::now(),
                patient_id: params.patient_id,
                doctor_id: params.doctor_id,
                doctor_name: params.doctor_name,
                patient_name: params.patient_name,
                visit: params.visit,
                date: params.date,
                appointment_duration: params.appointment_duration,
                choose_appointment: params.choose_appointment,
                note: params.note,
                room_and_equipment_no: params.room_and_equipment_no,
                staff_id: params.staff_id,
                staff_name: params.staff_name,
            };
            let events = make_handler(
                &APPOINTMENTS_AGGREGATE,
                &store,
                &AppointmentCommands::CreateAppointment(command.clone()),
                &stream_id,
                &EventsReadRange::AllEvents,
                &ExpectedVersion::Any,
            )
            .await?;

            let res = process_appointment_events(
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

            if let Err(err) = res {
                error!("Error processing appointment events: {:?}", err);
                return Err(ErrorData {
                    message: String::from("APPOINTMENT_NOT_ADDED"),
                    data: Value::Null,
                    code: -32600,
                });
            }

            info!("add_appointments function completed successfully");
            serde_json::to_value(row).map_err(ErrorData::from)
        } else {
            Err(ErrorData {
                message: String::from("DOCTOR_NOT_FOUND"),
                data: Value::Null,
                code: -32600,
            })
        }
    } else {
        Err(ErrorData {
            message: String::from("PATIENT_NOT_FOUND"),
            data: Value::Null,
            code: -32600,
        })
    }
}

pub async fn update_appointments(
    params: AppointmentUpdate,
    app_state: AppState,
    organization_id: String,
    user_id: String,
    service_location_id: String,
) -> Result<Value, ErrorData> {
    info!("Starting update_appointments function...");

    let store = appointment_store(&app_state.write_pool, &organization_id).await?;
    let appointment_db = sqlx::query_as::<_, Stream>(
        "SELECT id,stream_id,version,(data->>'created_by') AS created_by, (data->>'created_at') AS created_at, data FROM appointment_table_state WHERE id = ?",
    )
    .bind(params.id.clone())
    .fetch_optional(&app_state.read_pool.clone())
    .await?;

    let created_by = appointment_db
        .as_ref()
        .map(|org| org.created_by.clone())
        .unwrap_or_default();
    let stream_id = appointment_db
        .as_ref()
        .map(|org| org.stream_id.clone())
        .unwrap_or_default();
    let created_at = appointment_db
        .as_ref()
        .map(|org| org.created_at)
        .unwrap_or_default();
    let command = UpdateAppointment {
        id: params.id,
        org_id: organization_id.clone(),
        service_location_id,
        created_by,
        updated_by: user_id,
        created_at,
        last_updated: Utc::now(),
        patient_id: params.appointment.patient_id,
        doctor_id: params.appointment.doctor_id,
        doctor_name: params.appointment.doctor_name,
        patient_name: params.appointment.patient_name,
        visit: params.appointment.visit,
        date: params.appointment.date,
        appointment_duration: params.appointment.appointment_duration,
        choose_appointment: params.appointment.choose_appointment,
        note: params.appointment.note,
        room_and_equipment_no: params.appointment.room_and_equipment_no,
        staff_id: params.appointment.staff_id,
        staff_name: params.appointment.staff_name,
    };
    let events = make_handler(
        &APPOINTMENTS_AGGREGATE,
        &store,
        &AppointmentCommands::UpdateAppointment(command.clone()),
        &stream_id,
        &EventsReadRange::AllEvents,
        &ExpectedVersion::Any,
    )
    .await?;

    let res =
        process_appointment_events(app_state.read_pool.clone(), command.id, stream_id, events)
            .await;

    let row = sync(
        params.appointment.last_updated_input,
        organization_id.clone(),
        app_state.read_pool.clone(),
    )
    .await?;

    if let Err(err) = res {
        error!("Error processing appointment events: {:?}", err);
        return Err(ErrorData {
            message: String::from("APPOINTMENT_NOT_UPDATED"),
            data: Value::Null,
            code: -32600,
        });
    }

    info!("update_appointments function completed successfully");
    serde_json::to_value(row).map_err(ErrorData::from)
}

pub async fn delete_appointments(
    params: AppointmentDelete,
    app_state: AppState,
    organization_id: String,
    user_id: String,
    service_location_id: String,
) -> Result<Value, ErrorData> {
    info!("Starting delete_appointments function...");

    let store = appointment_store(&app_state.write_pool, &organization_id).await?;
    let appointment_db = sqlx::query_as::<_, Stream>(
        "SELECT id,stream_id,version,(data->>'created_by') AS created_by, (data->>'created_at') AS created_at, data FROM appointment_table_state WHERE id = ?",
    )
    .bind(params.id.clone())
    .fetch_optional(&app_state.read_pool.clone())
    .await?;

    let created_by = appointment_db
        .as_ref()
        .map(|org| org.created_by.clone())
        .unwrap_or_default();
    let stream_id = appointment_db
        .as_ref()
        .map(|org| org.stream_id.clone())
        .unwrap_or_default();
    let created_at = appointment_db
        .as_ref()
        .map(|org| org.created_at)
        .unwrap_or_default();
    let command = DeleteAppointment {
        id: params.id,
        patient_id: params.patient_id,
        service_location_id,
        doctor_id: params.doctor_id,
        org_id: organization_id.clone(),
        created_by,
        updated_by: user_id,
        created_at,
        last_updated: Utc::now(),
    };
    let events = make_handler(
        &APPOINTMENTS_AGGREGATE,
        &store,
        &AppointmentCommands::DeleteAppointment(command.clone()),
        &stream_id,
        &EventsReadRange::AllEvents,
        &ExpectedVersion::Any,
    )
    .await?;

    let res =
        process_appointment_events(app_state.read_pool.clone(), command.id, stream_id, events)
            .await;

    let row = sync(
        params.last_updated_input,
        organization_id.clone(),
        app_state.read_pool.clone(),
    )
    .await?;

    if let Err(err) = res {
        error!("Error processing appointment events: {:?}", err);
        return Err(ErrorData {
            message: String::from("APPOINTMENT_NOT_DELETED"),
            data: Value::Null,
            code: -32600,
        });
    }

    info!("delete_appointments function completed successfully");
    serde_json::to_value(row).map_err(ErrorData::from)
}
