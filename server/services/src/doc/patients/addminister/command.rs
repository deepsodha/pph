use crate::doc::patients::get_patient_id::patient_id_fetch;
use crate::doc::syncs::sync;
use crate::{app_state::AppState, method::convention::ErrorData};
use anyhow::Result;
use common::stream::Stream;
use cosmo_store::types::{event_read_range::EventsReadRange, expected_version::ExpectedVersion};
use cosmo_store_util::aggregate::make_handler;
use patient::domain::immunizations::administer::administer_commands::{
    DeleteAdminister, UpdateAdminister,
};
use patient::domain::immunizations::administer::{
    administer_aggregate::ADMINISTER_AGGREGATE,
    administer_commands::{AdministerCommand, CreateAdminister},
};
use patient::dto::immunization::administer_types::{
    AdministerAdd, AdministerDelete, AdministerUpdate,
};
use serde_json::Value;
use sqlx::types::chrono::Utc;
use utils::store_helper::patient_store;
use uuid::Uuid;

use super::helper::process_administer_events;
use log::{error, info};

pub async fn add_administer(
    params: AdministerAdd,
    app_state: AppState,
    organization_id: String,
    user_id: String,
) -> Result<Value, ErrorData> {
    let patient_id = patient_id_fetch(app_state.clone(), params.patient_id.clone()).await?;

    if patient_id {
        info!("Start adding administer entry");

        let store = patient_store(&app_state.write_pool, &organization_id).await?;
        let stream_id = format!("Administer::{}", Uuid::new_v4().as_simple());
        let command = CreateAdminister {
            id: Uuid::new_v4().as_simple().to_string(),
            created_by: user_id.clone(),
            updated_by: user_id.clone(),
            created_at: Utc::now(),
            last_updated: Utc::now(),
            org_id: organization_id.clone(),
            patient_id: params.patient_id,
            vaccine: params.vaccine,
            types: params.types,
            brand: params.brand,
            generic: params.generic,
            ordered: params.ordered,
            recorded: params.recorded,
            dose: params.dose,
            site: params.site,
            number_of_series: params.number_of_serial,
            lot: params.lot,
            expiration: params.expiration,
            consent_obtain: params.consent_obtain,
            administrated_by: params.administrated_by,
            clinic_location: params.clinic_location,
            provider: params.provider,
            vis_date: params.vis_date,
            vfs_financial_class: params.vfs_financial_class,
            comments: params.comments,
        };
        let events = make_handler(
            &ADMINISTER_AGGREGATE,
            &store,
            &AdministerCommand::CreateAdminister(command.clone()),
            &stream_id,
            &EventsReadRange::AllEvents,
            &ExpectedVersion::Any,
        )
        .await?;
        let res =
            process_administer_events(app_state.read_pool.clone(), command.id, stream_id, events)
                .await;

        let row = sync(
            params.last_updated_input,
            organization_id.clone(),
            app_state.read_pool.clone(),
        )
        .await?;

        match res {
            Ok(_) => {
                info!("Administer entry added successfully");
                serde_json::to_value(row).map_err(ErrorData::from)
            }
            Err(err) => {
                error!("{:#?}", err);
                Err(ErrorData {
                    message: String::from("ADMINISTER_NOT_ADDED"),
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

pub async fn update_administer(
    params: AdministerUpdate,
    app_state: AppState,
    organization_id: String,
    user_id: String,
) -> Result<Value, ErrorData> {
    let patient_id = patient_id_fetch(app_state.clone(), params.input.patient_id.clone()).await?;

    if patient_id {
        info!("Start updating administer entry");

        let store = patient_store(&app_state.write_pool, &organization_id).await?;
        let administer_db = sqlx::query_as::<_, Stream>(
        "SELECT id,stream_id,version,(data->>'created_by') AS created_by, (data->>'created_at') AS created_at,data FROM administer_table_state WHERE id = ?",
    )
    .bind(params.id.clone())
    .fetch_optional(&app_state.read_pool.clone())
    .await?;

        let created_by = administer_db
            .as_ref()
            .map(|org| org.created_by.clone())
            .unwrap_or_default();
        let stream_id = administer_db
            .as_ref()
            .map(|org| org.stream_id.clone())
            .unwrap_or_default();
        let created_at = administer_db
            .as_ref()
            .map(|org| org.created_at)
            .unwrap_or_default();
        let command = UpdateAdminister {
            id: params.id,
            created_by,
            updated_by: user_id,
            created_at,
            last_updated: Utc::now(),
            org_id: organization_id.clone(),
            patient_id: params.input.patient_id,
            vaccine: params.input.vaccine,
            types: params.input.types,
            brand: params.input.brand,
            generic: params.input.generic,
            ordered: params.input.ordered,
            recorded: params.input.recorded,
            dose: params.input.dose,
            site: params.input.site,
            number_of_series: params.input.number_of_serial,
            lot: params.input.lot,
            expiration: params.input.expiration,
            consent_obtain: params.input.consent_obtain,
            administrated_by: params.input.administrated_by,
            clinic_location: params.input.clinic_location,
            provider: params.input.provider,
            vis_date: params.input.vis_date,
            vfs_financial_class: params.input.vfs_financial_class,
            comments: params.input.comments,
        };
        let events = make_handler(
            &ADMINISTER_AGGREGATE,
            &store,
            &AdministerCommand::UpdateAdminister(command.clone()),
            &stream_id,
            &EventsReadRange::AllEvents,
            &ExpectedVersion::Any,
        )
        .await?;
        let res =
            process_administer_events(app_state.read_pool.clone(), command.id, stream_id, events)
                .await;

        let row = sync(
            params.input.last_updated_input,
            organization_id.clone(),
            app_state.read_pool.clone(),
        )
        .await?;

        match res {
            Ok(_) => {
                info!("Administer entry updated successfully");
                serde_json::to_value(row).map_err(ErrorData::from)
            }
            Err(err) => {
                error!("{:#?}", err);
                Err(ErrorData {
                    message: String::from("ADMINISTER_NOT_UPDATED"),
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

pub async fn delete_administer(
    params: AdministerDelete,
    app_state: AppState,
    organization_id: String,
    user_id: String,
) -> Result<Value, ErrorData> {
    info!("Start deleting administer entry");

    let patient_id = patient_id_fetch(app_state.clone(), params.patient_id.clone()).await?;

    if patient_id {
        let store = patient_store(&app_state.write_pool, &organization_id).await?;
        let administer_db = sqlx::query_as::<_, Stream>(
            "SELECT id,stream_id,version,(data->>'created_by') AS created_by, (data->>'created_at') AS created_at,data FROM administer_table_state WHERE id = ?",
        )
        .bind(params.id.clone())
        .fetch_optional(&app_state.read_pool.clone())
        .await?;

        let created_by = administer_db
            .as_ref()
            .map(|org| org.created_by.clone())
            .unwrap_or_default();
        let stream_id = administer_db
            .as_ref()
            .map(|org| org.stream_id.clone())
            .unwrap_or_default();
        let created_at = administer_db
            .as_ref()
            .map(|org| org.created_at)
            .unwrap_or_default();
        let command = DeleteAdminister {
            id: params.id,
            created_by,
            updated_by: user_id,
            created_at,
            last_updated: Utc::now(),
            org_id: organization_id.clone(),
            patient_id: params.patient_id,
        };
        let events = make_handler(
            &ADMINISTER_AGGREGATE,
            &store,
            &AdministerCommand::DeleteAdminister(command.clone()),
            &stream_id,
            &EventsReadRange::AllEvents,
            &ExpectedVersion::Any,
        )
        .await?;
        let res =
            process_administer_events(app_state.read_pool.clone(), command.id, stream_id, events)
                .await;

        let row = sync(
            params.last_updated_input,
            organization_id.clone(),
            app_state.read_pool.clone(),
        )
        .await?;

        match res {
            Ok(_) => {
                info!("Administer entry deleted successfully");
                serde_json::to_value(row).map_err(ErrorData::from)
            }
            Err(err) => {
                error!("{:#?}", err);
                Err(ErrorData {
                    message: String::from("ADMINISTER_NOT_DELETED"),
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
