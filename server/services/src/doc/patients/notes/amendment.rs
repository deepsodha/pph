use crate::app_state::AppState;
use crate::doc::patients::notes::helper::process_notes_events;
use crate::doc::syncs::sync;
use crate::method::convention::ErrorData;
use chrono::Utc;
use cosmo_store::types::event_read_range::EventsReadRange;
use cosmo_store::types::expected_version::ExpectedVersion;
use cosmo_store_util::aggregate::make_handler;
use log::error;
use log::info;
use patient::domain::notes::note_aggregate::NOTE_AGGREGATE;
use patient::domain::notes::note_command::CreateNote;
use patient::domain::notes::note_command::NoteCommand;
use patient::dto::notes::amendment::amendment_add::AmendmentAdd;
use patient::dto::notes::notes_types::Amendment;
use patient::dto::notes::notes_types::NoteType;
use serde_json::Value;
use utils::store_helper::note_store;
use uuid::Uuid;

pub async fn add_amendment(
    params: AmendmentAdd,
    app_state: AppState,
    organization_id: String,
    user_id: String,
) -> Result<Value, ErrorData> {
    let store = note_store(&app_state.write_pool, &organization_id).await?;
    let stream_id = format!("Note::{}", Uuid::new_v4().as_simple());

    let command = CreateNote {
        id: Uuid::new_v4().as_simple().to_string(),
        created_by: user_id.clone(),
        updated_by: user_id.clone(),
        created_at: Utc::now(),
        last_updated: Utc::now(),
        org_id: organization_id.clone(),
        patient_id: params.patient_id.clone(),
        note: NoteType::Amendment(Amendment {
            source_of_request: params.source_of_request.clone(),
            request_details: params.request_details.clone(),
            decision: params.decision.clone(),
        }),
        note_state: params.note_state.clone(),
    };

    info!("Making handler to process note creation command...");
    let events = make_handler(
        &NOTE_AGGREGATE,
        &store,
        &NoteCommand::CreateNote(command.clone()),
        &stream_id,
        &EventsReadRange::AllEvents,
        &ExpectedVersion::Any,
    )
    .await?;

    info!("Processing Amendment events...");
    let res =
        process_notes_events(app_state.read_pool.clone(), command.id, stream_id, events).await;

    info!("Synchronizing data...");
    let row = sync(
        params.last_updated_input,
        organization_id.clone(),
        app_state.read_pool.clone(),
    )
    .await?;

    match res {
        anyhow::Result::Ok(_) => {
            info!("Amendment added successfully.");
            serde_json::to_value(row).map_err(ErrorData::from)
        }
        Err(err) => {
            error!("Error occurred while adding Amendment: {:#?}", err);
            Err(ErrorData {
                message: String::from("AMENDMENT_FOLLOW_UP_NOT_ADDED"),
                data: Value::Null,
                code: -32600,
            })
        }
    }
}
