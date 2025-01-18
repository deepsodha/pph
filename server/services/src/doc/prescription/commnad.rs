use crate::{app_state::AppState, doc::syncs::sync, method::convention::ErrorData};
use anyhow::Result;
use common::stream::Stream;
use cosmo_store::types::{event_read_range::EventsReadRange, expected_version::ExpectedVersion};
use cosmo_store_util::aggregate::make_handler;
use prescription::{
    domain::{
        prescription_aggregate::PRESCRIPTION_AGGREGATE,
        prescription_commands::{
            CreatePrescription, DeletePrescription, PrescriptionCommand, UpdatedPrescription,
        },
    },
    dto::{
        prescription_add::PrescriptionAdd, prescription_delete::PrescriptionDelete,
        prescription_update::PrescriptionUpdate,
    },
};
use serde_json::Value;
use sqlx::types::chrono::Utc;
use utils::store_helper::prescription_store;
use uuid::Uuid;

use super::helper::process_prescription_events;
pub async fn add_prescription(
    params: PrescriptionAdd,
    app_state: AppState,
    organization_id: String,
    user_id: String,
) -> Result<Value, ErrorData> {
    log::info!(
        "Adding prescription for organization_id: {} and user_id: {}",
        organization_id,
        user_id
    );

    let store = prescription_store(&app_state.write_pool, &organization_id).await?;
    let stream_id = format!("Prescription::{}", Uuid::new_v4().as_simple());
    let command = CreatePrescription {
        id: Uuid::new_v4().as_simple().to_string(),
        patient_id: params.patient_id,
        doctor_id: user_id.clone(),
        created_by: user_id.clone(),
        updated_by: user_id.clone(),
        created_at: Utc::now(),
        last_updated: Utc::now(),
        org_id: organization_id.clone(),
        patient_name: params.patient_name,
        presecription_name: params.presecription_name,
        instruction: params.instruction,
        date: params.date,
        drug_name: params.drug_name,
    };

    let events = make_handler(
        &PRESCRIPTION_AGGREGATE,
        &store,
        &PrescriptionCommand::CreatePrescription(command.clone()),
        &stream_id,
        &EventsReadRange::AllEvents,
        &ExpectedVersion::Any,
    )
    .await?;

    let res =
        process_prescription_events(app_state.read_pool.clone(), command.id, stream_id, events)
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
                "Prescription added successfully for organization_id: {} and user_id: {}",
                organization_id,
                user_id
            );
            serde_json::to_value(row).map_err(ErrorData::from)
        }
        Err(err) => {
            log::error!(
                "Error adding prescription for organization_id: {} and user_id: {}: {:#?}",
                organization_id,
                user_id,
                err
            );
            Err(ErrorData {
                message: String::from("PRESCRIPTION_NOT_ADDED"),
                data: Value::Null,
                code: -32600,
            })
        }
    }
}

pub async fn update_prescription(
    params: PrescriptionUpdate,
    app_state: AppState,
    organization_id: String,
    user_id: String,
) -> Result<Value, ErrorData> {
    log::info!(
        "Updating prescription for organization_id: {} and user_id: {}",
        organization_id,
        user_id.clone()
    );

    let store = prescription_store(&app_state.write_pool, &organization_id).await?;
    let prescription_db = sqlx::query_as::<_, Stream>(
        "SELECT id,stream_id,version,(data->>'created_by') AS created_by, (data->>'created_at') AS created_at,data FROM prescription_table_state WHERE id = ?",
    )
    .bind(params.id.clone())
    .fetch_optional(&app_state.read_pool.clone())
    .await?;

    let created_by = prescription_db
        .as_ref()
        .map(|org| org.created_by.clone())
        .unwrap_or_default();
    let stream_id = prescription_db
        .as_ref()
        .map(|org| org.stream_id.clone())
        .unwrap_or_default();
    let created_at = prescription_db
        .as_ref()
        .map(|org| org.created_at)
        .unwrap_or_default();
    let command = UpdatedPrescription {
        id: params.id,
        org_id: organization_id.clone(),
        patient_id: params.input.patient_id,
        doctor_id: user_id.clone(),
        created_by,
        updated_by: user_id.clone(),
        created_at,
        last_updated: Utc::now(),
        patient_name: params.input.patient_name,
        presecription_name: params.input.presecription_name,
        instruction: params.input.instruction,
        date: params.input.date,
        drug_name: params.input.drug_name,
    };
    let events = make_handler(
        &PRESCRIPTION_AGGREGATE,
        &store,
        &PrescriptionCommand::UpdatedPrescription(command.clone()),
        &stream_id,
        &EventsReadRange::AllEvents,
        &ExpectedVersion::Any,
    )
    .await?;
    let res =
        process_prescription_events(app_state.read_pool.clone(), command.id, stream_id, events)
            .await;

    let row = sync(
        params.input.last_updated_input,
        organization_id.clone(),
        app_state.read_pool.clone(),
    )
    .await?;

    match res {
        Ok(_) => {
            log::info!(
                "Prescription updated successfully for organization_id: {} and user_id: {}",
                organization_id,
                user_id
            );
            serde_json::to_value(row).map_err(ErrorData::from)
        }
        Err(err) => {
            log::error!(
                "Error updating prescription for organization_id: {} and user_id: {}: {:#?}",
                organization_id,
                user_id,
                err
            );
            Err(ErrorData {
                message: String::from("PRESCRIPTION_NOT_UPDATED"),
                data: Value::Null,
                code: -32600,
            })
        }
    }
}

pub async fn delete_prescription(
    params: PrescriptionDelete,
    app_state: AppState,
    organization_id: String,
    user_id: String,
) -> Result<Value, ErrorData> {
    log::info!(
        "Deleting prescription for organization_id: {} and user_id: {}",
        organization_id,
        user_id.clone()
    );

    let store = prescription_store(&app_state.write_pool, &organization_id).await?;
    let prescription_db = sqlx::query_as::<_, Stream>(
        "SELECT id,stream_id,version,(data->>'created_by') AS created_by, (data->>'created_at') AS created_at,data FROM prescription_table_state WHERE id = ?",
    )
    .bind(params.id.clone())
    .fetch_optional(&app_state.read_pool.clone())
    .await?;

    let created_by = prescription_db
        .as_ref()
        .map(|org| org.created_by.clone())
        .unwrap_or_default();
    let stream_id = prescription_db
        .as_ref()
        .map(|org| org.stream_id.clone())
        .unwrap_or_default();
    let created_at = prescription_db
        .as_ref()
        .map(|org| org.created_at)
        .unwrap_or_default();
    let command = DeletePrescription {
        id: params.id,
        patient_id: params.patient_id,
        doctor_id: user_id.clone(),
        created_by,
        updated_by: user_id.clone(),
        created_at,
        last_updated: Utc::now(),
        org_id: organization_id.clone(),
    };

    let events = make_handler(
        &PRESCRIPTION_AGGREGATE,
        &store,
        &PrescriptionCommand::DeletePrescription(command.clone()),
        &stream_id,
        &EventsReadRange::AllEvents,
        &ExpectedVersion::Any,
    )
    .await?;
    let res =
        process_prescription_events(app_state.read_pool.clone(), command.id, stream_id, events)
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
                "Prescription deleted successfully for organization_id: {} and user_id: {}",
                organization_id,
                user_id
            );
            serde_json::to_value(row).map_err(ErrorData::from)
        }
        Err(err) => {
            log::error!(
                "Error deleting prescription for organization_id: {} and user_id: {}: {:#?}",
                organization_id,
                user_id,
                err
            );
            Err(ErrorData {
                message: String::from("PRESCRIPTION_NOT_DELETED"),
                data: Value::Null,
                code: -32600,
            })
        }
    }
}
