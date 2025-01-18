use super::helper::process_obandpregnancy_events;
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
    domain::history::obandpregnancy::{
        obandpregnancy_aggregate::OBANDPREGNANCY_AGGREGATE,
        obandpregnancy_commands::{
            CreateOBandPregnancy, DeleteOBandPregnancy, OBandPregnancyCommand, UpdateOBandPregnancy,
        },
    },
    dto::history::obandpregnancy::{
        obandpregnancy_add::OBandPregnancyAdd, obandpregnancy_delete::OBandPregnancyDelete,
        obandpregnancy_update::OBandPregnancyUpdate,
    },
};
use serde_json::Value;
use utils::store_helper::obandpregnancy_store;
use uuid::Uuid;

pub async fn add_ob_and_pregnancy(
    params: OBandPregnancyAdd,
    app_state: AppState,
    organization_id: String,
    user_id: String,
) -> Result<Value, ErrorData> {
    let patient_id = patient_id_fetch(app_state.clone(), params.patient_id.clone()).await?;
    if patient_id {
        info!("Adding OB and Pregnancy information...");

        let store = obandpregnancy_store(&app_state.write_pool, &organization_id).await?;
        let stream_id = format!("OBandPregnancy::{}", uuid::Uuid::new_v4().as_simple());
        let command = CreateOBandPregnancy {
            id: Uuid::new_v4().as_simple().to_string(),
            org_id: organization_id.clone(),
            patient_id: params.patient_id,
            created_by: user_id.clone(),
            updated_by: user_id.clone(),
            created_at: Utc::now(),
            last_updated: Utc::now(),
            age_onset_of_menses: params.age_onset_of_menses,
            age_at_menopause: params.age_at_menopause,
            comments_ob: params.comments_ob,
            total_pregnancy: params.total_pregnancy,
            full_term: params.full_term,
            pre_term: params.pre_term,
            miscarriages: params.miscarriages,
            living: params.living,
            comments_pregnancy: params.comments_pregnancy,
        };

        debug!("Creating OB and Pregnancy command: {:?}", &command);

        let events = make_handler(
            &OBANDPREGNANCY_AGGREGATE,
            &store,
            &OBandPregnancyCommand::CreateOBandPregnancy(command.clone()),
            &stream_id,
            &EventsReadRange::AllEvents,
            &ExpectedVersion::Any,
        )
        .await?;

        debug!("Events created: {:?}", &events);

        let res = process_obandpregnancy_events(
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
                info!("OB and Pregnancy information added successfully.");
                serde_json::to_value(row).map_err(ErrorData::from)
            }
            Err(err) => {
                error!("Error adding OB and Pregnancy information: {:#?}", err);
                Err(ErrorData {
                    message: String::from("OBANDPREGNANCY_NOT_ADDED"),
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

pub async fn update_ob_and_pregnancy(
    params: OBandPregnancyUpdate,
    app_state: AppState,
    organization_id: String,
    user_id: String,
) -> Result<Value, ErrorData> {
    let patient_id = patient_id_fetch(app_state.clone(), params.input.patient_id.clone()).await?;
    if patient_id {
        info!("Updating OB and Pregnancy information...");

        let store = obandpregnancy_store(&app_state.write_pool, &organization_id).await?;
        let obandpregnancy_db = sqlx::query_as::<_, Stream>(
        "SELECT id,stream_id,version,(data->>'created_by') AS created_by, (data->>'created_at') AS created_at,data FROM obandpregnancy_table_state WHERE id = ?",
    )
    .bind(params.id.clone())
    .fetch_optional(&app_state.read_pool.clone())
    .await?;

        let created_by = obandpregnancy_db
            .as_ref()
            .map(|org| org.created_by.clone())
            .unwrap_or_default();
        let stream_id = obandpregnancy_db
            .as_ref()
            .map(|org| org.stream_id.clone())
            .unwrap_or_default();
        let created_at = obandpregnancy_db
            .as_ref()
            .map(|org| org.created_at)
            .unwrap_or_default();
        let command = UpdateOBandPregnancy {
            id: params.id,
            created_by,
            updated_by: user_id.clone(),
            created_at,
            last_updated: Utc::now(),
            org_id: organization_id.clone(),
            patient_id: params.input.patient_id,
            age_onset_of_menses: params.input.age_onset_of_menses,
            age_at_menopause: params.input.age_at_menopause,
            comments_ob: params.input.comments_ob,
            total_pregnancy: params.input.total_pregnancy,
            full_term: params.input.full_term,
            pre_term: params.input.pre_term,
            miscarriages: params.input.miscarriages,
            living: params.input.living,
            comments_pregnancy: params.input.comments_pregnancy,
        };

        debug!("Updating OB and Pregnancy command: {:?}", &command);

        let events = make_handler(
            &OBANDPREGNANCY_AGGREGATE,
            &store,
            &OBandPregnancyCommand::UpdateOBandPregnancy(command.clone()),
            &stream_id,
            &EventsReadRange::AllEvents,
            &ExpectedVersion::Any,
        )
        .await?;

        debug!("Events created: {:?}", &events);

        let res = process_obandpregnancy_events(
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
                info!("OB and Pregnancy information updated successfully.");
                serde_json::to_value(row).map_err(ErrorData::from)
            }
            Err(err) => {
                error!("Error updating OB and Pregnancy information: {:#?}", err);
                Err(ErrorData {
                    message: String::from("OBANDPREGNANCY_NOT_UPDATED"),
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

pub async fn delete_ob_and_pregnancy(
    params: OBandPregnancyDelete,
    app_state: AppState,
    organization_id: String,
    user_id: String,
) -> Result<Value, ErrorData> {
    let patient_id = patient_id_fetch(app_state.clone(), params.patient_id.clone()).await?;
    if patient_id {
        info!("Deleting OB and Pregnancy information...");

        let store = obandpregnancy_store(&app_state.write_pool, &organization_id).await?;
        let obandpregnancy_db = sqlx::query_as::<_, Stream>(
            "SELECT id,stream_id,version,(data->>'created_by') AS created_by, (data->>'created_at') AS created_at,data FROM obandpregnancy_table_state WHERE id = ?",
        )
        .bind(params.id.clone())
        .fetch_optional(&app_state.read_pool.clone())
        .await?;

        let created_by = obandpregnancy_db
            .as_ref()
            .map(|org| org.created_by.clone())
            .unwrap_or_default();
        let stream_id = obandpregnancy_db
            .as_ref()
            .map(|org| org.stream_id.clone())
            .unwrap_or_default();
        let created_at = obandpregnancy_db
            .as_ref()
            .map(|org| org.created_at)
            .unwrap_or_default();

        let command = DeleteOBandPregnancy {
            id: params.id,
            patient_id: params.patient_id,
            org_id: organization_id.clone(),
            created_by,
            updated_by: user_id.clone(),
            created_at,
            last_updated: Utc::now(),
        };

        debug!("Delete OB and Pregnancy command: {:?}", &command);

        let events = make_handler(
            &OBANDPREGNANCY_AGGREGATE,
            &store,
            &OBandPregnancyCommand::DeleteOBandPregnancy(command.clone()),
            &stream_id,
            &EventsReadRange::AllEvents,
            &ExpectedVersion::Any,
        )
        .await?;

        debug!("Events created: {:?}", &events);

        let res = process_obandpregnancy_events(
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
                info!("OB and Pregnancy information deleted successfully.");
                serde_json::to_value(row).map_err(ErrorData::from)
            }
            Err(err) => {
                error!("Error deleting OB and Pregnancy information: {:#?}", err);
                Err(ErrorData {
                    message: String::from("OBANDPREGNANCY_NOT_DELETED"),
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
