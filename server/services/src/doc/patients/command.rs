use super::helper::process_patient_events;
use crate::doc::syncs::sync;
use crate::{app_state::AppState, method::convention::ErrorData};
use anyhow::Result;
use common::dto::address::AddressInput;
use common::dto::contact::ContactInput;
use common::dto::gov_info::GovInfoInput;
use common::dto::user::UserInput;
use common::stream::Stream;
use cosmo_store::types::{event_read_range::EventsReadRange, expected_version::ExpectedVersion};
use cosmo_store_util::aggregate::make_handler;
use log::{error, info};
use patient::domain::patient_aggregate::PATIENT_AGGREGATE;
use patient::domain::patient_commands::{
    CreatePatient, DeletePatient, PatientCommand, UpdatePatientAddress,
    UpdatePatientContactDetails, UpdatePatientGovDetails, UpdatePatientUserDetails,
};
use patient::dto::patient::{
    patient_add::PatientAdd, patient_delete::PatientDelete, patient_update::PatientUpdate,
};
use serde_json::Value;
use sqlx::types::chrono::Utc;
use utils::store_helper::patient_store;
use uuid::Uuid;

pub async fn add_patient(
    params: PatientAdd,
    app_state: AppState,
    organization_id: String,
    user_id: String,
) -> Result<Value, ErrorData> {
    info!("Adding patient");

    let store = patient_store(&app_state.write_pool, &organization_id).await?;
    let stream_id = format!("Patient::{}", uuid::Uuid::new_v4().as_simple());
    let command = CreatePatient {
        id: Uuid::new_v4().as_simple().to_string(),
        org_id: organization_id.clone(),
        created_by: user_id.clone(),
        updated_by: user_id.clone(),
        created_at: Utc::now(),
        last_updated: Utc::now(),
        user: params.user,
        address: params.address,
        phone: params.phone,
        government_info: params.government_info,
    };

    info!("Creating command for adding patient");

    let events = make_handler(
        &PATIENT_AGGREGATE,
        &store,
        &PatientCommand::CreatePatient(Box::new(command.clone())),
        &stream_id,
        &EventsReadRange::AllEvents,
        &ExpectedVersion::Any,
    )
    .await?;

    info!("Handling events for adding patient");

    let res =
        process_patient_events(app_state.read_pool.clone(), command.id, stream_id, events).await;

    info!("Processing patient events");

    let row = sync(
        params.last_updated_input,
        organization_id.clone(),
        app_state.read_pool.clone(),
    )
    .await?;

    match res {
        Ok(_) => {
            info!("Patient added successfully");
            serde_json::to_value(row).map_err(ErrorData::from)
        }
        Err(err) => {
            error!("Error adding patient: {:#?}", err);

            let error_data = ErrorData {
                message: String::from("PATIENT_NOT_ADDED"),
                data: serde_json::json!({"error_message": err.to_string()}),
                code: -32600,
            };
            Err(error_data)
        }
    }
}

pub async fn update_patient(
    params: PatientUpdate,
    app_state: AppState,
    organization_id: String,
    user_id: String,
) -> Result<Value, ErrorData> {
    info!("Updating patient");

    let store = patient_store(&app_state.write_pool, &organization_id).await?;
    let patient_db = sqlx::query_as::<_, Stream>(
        "SELECT id,stream_id,version,(data->>'created_by') AS created_by, (data->>'created_at') AS created_at, data FROM patient_table_state WHERE id = ?",
    )
    .bind(params.id.clone())
    .fetch_optional(&app_state.read_pool.clone())
    .await?;

    let id = patient_db
        .as_ref()
        .map(|org| org.id.clone())
        .unwrap_or_default();
    let created_by = patient_db
        .as_ref()
        .map(|org| org.created_by.clone())
        .unwrap_or_default();
    let stream_id = patient_db
        .as_ref()
        .map(|org| org.stream_id.clone())
        .unwrap_or_default();
    let created_at = patient_db
        .as_ref()
        .map(|org| org.created_at)
        .unwrap_or_default();

    let data = patient_db
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

    if params.input.address == address
        && params.input.government_info == government_info
        && params.input.phone == phone
        && params.input.user == user
    {
        return Err(
            anyhow::anyhow!("Patient Details Not Updated. No changes made in the form.").into(),
        );
    }

    if params.input.address != address {
        info!("Updating patient address");
        let command_address = UpdatePatientAddress {
            id: id.clone(),
            address: params.input.address.clone(),
            created_by: created_by.clone(),
            updated_by: user_id.clone(),
            created_at,
            last_updated: Utc::now(),
        };

        let events_address = make_handler(
            &PATIENT_AGGREGATE,
            &store,
            &PatientCommand::UpdatePatientAddress(command_address.clone()),
            &stream_id,
            &EventsReadRange::AllEvents,
            &ExpectedVersion::Any,
        )
        .await?;

        let _ = process_patient_events(
            app_state.read_pool.clone(),
            command_address.id.clone(),
            stream_id.clone(),
            events_address,
        )
        .await;
    }

    if params.input.user != user {
        info!("Updating patient user");
        let command_user = UpdatePatientUserDetails {
            id: id.clone(),
            created_by: created_by.clone(),
            updated_by: user_id.clone(),
            created_at,
            last_updated: Utc::now(),
            user: params.input.user.clone(),
        };

        let events_user = make_handler(
            &PATIENT_AGGREGATE,
            &store,
            &PatientCommand::UpdatePatientUserDetails(Box::new(command_user.clone())),
            &stream_id,
            &EventsReadRange::AllEvents,
            &ExpectedVersion::Any,
        )
        .await?;

        let _ = process_patient_events(
            app_state.read_pool.clone(),
            command_user.id.clone(),
            stream_id.clone(),
            events_user,
        )
        .await;
    }

    if params.input.phone != phone {
        info!("Updating patient phone");
        let command_phone = UpdatePatientContactDetails {
            id: id.clone(),
            created_by: created_by.clone(),
            updated_by: user_id.clone(),
            created_at,
            last_updated: Utc::now(),
            phone: params.input.phone.clone(),
        };

        let events_phone = make_handler(
            &PATIENT_AGGREGATE,
            &store,
            &PatientCommand::UpdatePatientContactDetails(command_phone.clone()),
            &stream_id,
            &EventsReadRange::AllEvents,
            &ExpectedVersion::Any,
        )
        .await?;

        let _ = process_patient_events(
            app_state.read_pool.clone(),
            command_phone.id.clone(),
            stream_id.clone(),
            events_phone,
        )
        .await;
    }

    if params.input.government_info != government_info {
        info!("Updating patient government_info");
        let command_gov = UpdatePatientGovDetails {
            id: id.clone(),
            created_by: created_by.clone(),
            updated_by: user_id.clone(),
            created_at,
            last_updated: Utc::now(),
            government_info: params.input.government_info.clone(),
        };

        let events_gov = make_handler(
            &PATIENT_AGGREGATE,
            &store,
            &PatientCommand::UpdatePatientGovDetails(command_gov.clone()),
            &stream_id,
            &EventsReadRange::AllEvents,
            &ExpectedVersion::Any,
        )
        .await?;

        let _ = process_patient_events(
            app_state.read_pool.clone(),
            command_gov.id.clone(),
            stream_id.clone(),
            events_gov,
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
        info!("Patient updated successfully");
        serde_json::to_value(row.clone()).map_err(ErrorData::from)
    } else {
        error!("Patient not updated");
        Err(ErrorData {
            message: String::from("PATIENT_NOT_UPDATED"),
            data: Value::Null,
            code: -32600,
        })
    }
}

pub async fn delete_patient(
    params: PatientDelete,
    app_state: AppState,
    organization_id: String,
    user_id: String,
) -> Result<Value, ErrorData> {
    info!("Deleting patient");

    let store = patient_store(&app_state.write_pool, &organization_id).await?;
    let patient_db = sqlx::query_as::<_, Stream>(
        "SELECT id,stream_id,version,(data->>'created_by') AS created_by, (data->>'created_at') AS created_at, data FROM patient_table_state WHERE id = ?",
    )
    .bind(params.id.clone())
    .fetch_optional(&app_state.read_pool.clone())
    .await?;

    let created_by = patient_db
        .as_ref()
        .map(|org| org.created_by.clone())
        .unwrap_or_default();
    let stream_id = patient_db
        .as_ref()
        .map(|org| org.stream_id.clone())
        .unwrap_or_default();
    let created_at = patient_db
        .as_ref()
        .map(|org| org.created_at)
        .unwrap_or_default();
    let command = DeletePatient {
        id: params.id,
        created_by,
        updated_by: user_id.clone(),
        created_at,
        last_updated: Utc::now(),
        org_id: organization_id.clone(),
    };

    info!("Creating command for deleting patient");

    let events = make_handler(
        &PATIENT_AGGREGATE,
        &store,
        &PatientCommand::DeletePatient(command.clone()),
        &stream_id,
        &EventsReadRange::AllEvents,
        &ExpectedVersion::Any,
    )
    .await?;

    info!("Handling events for deleting patient");

    let res =
        process_patient_events(app_state.read_pool.clone(), command.id, stream_id, events).await;

    let row = sync(
        params.last_updated_input,
        organization_id.clone(),
        app_state.read_pool.clone(),
    )
    .await?;

    match res {
        Ok(_) => {
            info!("Patient deleted successfully");
            serde_json::to_value(row).map_err(ErrorData::from)
        }
        Err(err) => {
            error!("Error deleting patient: {:#?}", err);
            Err(ErrorData {
                message: String::from("PATIENT_NOT_DELETED"),
                data: Value::Null,
                code: -32600,
            })
        }
    }
}
