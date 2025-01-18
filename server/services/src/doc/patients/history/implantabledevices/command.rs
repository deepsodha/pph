use super::helper::process_implantabledevices_events;
use crate::{
    app_state::AppState,
    doc::{patients::get_patient_id::patient_id_fetch, syncs::sync},
    method::convention::ErrorData,
};
use chrono::Utc;
use common::stream::Stream;
use cosmo_store::types::{event_read_range::EventsReadRange, expected_version::ExpectedVersion};
use cosmo_store_util::aggregate::make_handler;
use log::{debug, error, info};
use patient::{
    domain::history::implantabledevices::{
        implantable_devices_aggregate::IMPLANTABLEDEVICES_AGGREGATE,
        implantable_devices_commands::{
            CreateImplantableDevices, DeleteImplantableDevices, ImplantableDevicesCommand,
            UpdateImplantableDevices,
        },
    },
    dto::history::implantabledevices::{
        implantabledevices_add::ImplantableDevicesAdd,
        implantabledevices_delete::ImplantableDevicesDelete,
        implantabledevices_update::ImplantableDevicesUpdate,
    },
};
use serde_json::Value;
use utils::store_helper::implantabledevices_store;
use uuid::Uuid;

pub async fn add_implantable_devices(
    params: ImplantableDevicesAdd,
    app_state: AppState,
    organization_id: String,
    user_id: String,
) -> Result<Value, ErrorData> {
    let patient_id = patient_id_fetch(app_state.clone(), params.patient_id.clone()).await?;
    if patient_id {
        info!("Adding implantable devices...");

        let store = implantabledevices_store(&app_state.write_pool, &organization_id).await?;
        let stream_id = format!("ImplantableDevices::{}", uuid::Uuid::new_v4().as_simple());
        let command = CreateImplantableDevices {
            id: Uuid::new_v4().as_simple().to_string(),
            org_id: organization_id.clone(),
            patient_id: params.patient_id,
            created_by: user_id.clone(),
            updated_by: user_id.clone(),
            created_at: Utc::now(),
            last_updated: Utc::now(),
            status: params.status,
            comments: params.comments,
            udi: params.udi,
            udi_unknown: params.udi_unknown,
        };

        debug!("Creating implantable devices command: {:?}", &command);

        let events = make_handler(
            &IMPLANTABLEDEVICES_AGGREGATE,
            &store,
            &ImplantableDevicesCommand::CreateImplantableDevices(command.clone()),
            &stream_id,
            &EventsReadRange::AllEvents,
            &ExpectedVersion::Any,
        )
        .await?;

        debug!("Events created: {:?}", &events);

        let res = process_implantabledevices_events(
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
            anyhow::Result::Ok(_) => {
                info!("Implantable devices added successfully.");
                serde_json::to_value(row).map_err(ErrorData::from)
            }
            Err(err) => {
                error!("Error adding implantable devices: {:#?}", err);
                Err(ErrorData {
                    message: String::from("IMPLANTABLEDEVICES_NOT_ADDED"),
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

pub async fn update_implantable_devices(
    params: ImplantableDevicesUpdate,
    app_state: AppState,
    organization_id: String,
    user_id: String,
) -> Result<Value, ErrorData> {
    let patient_id = patient_id_fetch(app_state.clone(), params.input.patient_id.clone()).await?;
    if patient_id {
        info!("Updating implantable devices...");

        let store = implantabledevices_store(&app_state.write_pool, &organization_id).await?;
        let implantabledevices_db = sqlx::query_as::<_, Stream>(
        "SELECT id,stream_id,version,(data->>'created_by') AS created_by, (data->>'created_at') AS created_at,data FROM implantabledevices_table_state WHERE id = ?",
    )
    .bind(params.id.clone())
    .fetch_optional(&app_state.read_pool.clone())
    .await?;

        let created_by = implantabledevices_db
            .as_ref()
            .map(|org| org.created_by.clone())
            .unwrap_or_default();
        let stream_id = implantabledevices_db
            .as_ref()
            .map(|org| org.stream_id.clone())
            .unwrap_or_default();
        let created_at = implantabledevices_db
            .as_ref()
            .map(|org| org.created_at)
            .unwrap_or_default();
        let command = UpdateImplantableDevices {
            id: params.id,
            created_by,
            updated_by: user_id.clone(),
            created_at,
            last_updated: Utc::now(),
            org_id: organization_id.clone(),
            patient_id: params.input.patient_id,
            comments: params.input.comments,
            status: params.input.status,
            udi: params.input.udi,
            udi_unknown: params.input.udi_unknown,
        };

        debug!("Updating implantable devices command: {:?}", &command);

        let events = make_handler(
            &IMPLANTABLEDEVICES_AGGREGATE,
            &store,
            &ImplantableDevicesCommand::UpdateImplantableDevices(command.clone()),
            &stream_id,
            &EventsReadRange::AllEvents,
            &ExpectedVersion::Any,
        )
        .await?;

        debug!("Events created: {:?}", &events);

        let res = process_implantabledevices_events(
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
            anyhow::Result::Ok(_) => {
                info!("Implantable devices updated successfully.");
                serde_json::to_value(row).map_err(ErrorData::from)
            }
            Err(err) => {
                error!("Error updating implantable devices: {:#?}", err);
                Err(ErrorData {
                    message: String::from("IMPLATABLEDEVICES_NOT_UPDATED"),
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

pub async fn delete_implantable_devices(
    params: ImplantableDevicesDelete,
    app_state: AppState,
    organization_id: String,
    user_id: String,
) -> Result<Value, ErrorData> {
    let patient_id = patient_id_fetch(app_state.clone(), params.patient_id.clone()).await?;
    if patient_id {
        info!("Deleting implantable devices...");

        let store = implantabledevices_store(&app_state.write_pool, &organization_id).await?;
        let implantabledevices_db = sqlx::query_as::<_, Stream>(
        "SELECT id,stream_id,version,(data->>'created_by') AS created_by, (data->>'created_at') AS created_at,data FROM implantabledevices_table_state WHERE id = ?",
    )
    .bind(params.id.clone())
    .fetch_optional(&app_state.read_pool.clone())
    .await?;

        let created_by = implantabledevices_db
            .as_ref()
            .map(|org| org.created_by.clone())
            .unwrap_or_default();
        let stream_id = implantabledevices_db
            .as_ref()
            .map(|org| org.stream_id.clone())
            .unwrap_or_default();
        let created_at = implantabledevices_db
            .as_ref()
            .map(|org| org.created_at)
            .unwrap_or_default();

        let command = DeleteImplantableDevices {
            id: params.id,
            patient_id: params.patient_id,
            org_id: organization_id.clone(),
            created_by,
            updated_by: user_id.clone(),
            created_at,
            last_updated: Utc::now(),
        };

        debug!("Deleting implantable devices command: {:?}", &command);

        let events = make_handler(
            &IMPLANTABLEDEVICES_AGGREGATE,
            &store,
            &ImplantableDevicesCommand::DeleteImplantableDevices(command.clone()),
            &stream_id,
            &EventsReadRange::AllEvents,
            &ExpectedVersion::Any,
        )
        .await?;

        debug!("Events created: {:?}", &events);

        let res = process_implantabledevices_events(
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
            anyhow::Result::Ok(_) => {
                info!("Implantable devices deleted successfully.");
                serde_json::to_value(row).map_err(ErrorData::from)
            }
            Err(err) => {
                error!("Error deleting implantable devices: {:#?}", err);
                Err(ErrorData {
                    message: String::from("IMPLATABLEDEVICES_NOT_DELETED"),
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
