use crate::doc::organization::helper::read_json_from_file;
use crate::{
    app_state::AppState,
    doc::{
        organization::helper::{get_all_vaccines, read_vaccine_csv},
        syncs::sync,
    },
    method::convention::ErrorData,
};
use anyhow::Result;
use common::dto::last_updated_input::LastUpdatedInput;
use db_services::tables::organization::Getorganization;
use organization::dto::{
    csv::{LocationRecord, PinCodeInput},
    organization_by_id::OrganizationsByID,
};
use serde_json::Value;

pub async fn sync_handler(
    params: LastUpdatedInput,
    organization_id: String,
    state: AppState,
) -> Result<Value, ErrorData> {
    log::info!(
        "Handling sync request for organization_id: {}",
        organization_id
    );

    let row = sync(params, organization_id.clone(), state.read_pool.clone()).await?;

    log::info!(
        "Sync request handled successfully for organization_id: {}",
        organization_id
    );
    serde_json::to_value(row).map_err(ErrorData::from)
}

pub async fn drug_handler() -> Result<Value, ErrorData> {
    log::info!("Handling get_all_drug request");

    let path = "statics/data/combined.json";
    let records = read_json_from_file(path).map_err(|e| {
        log::error!("Failed to read drug data from '{}': {}", path, e);
        ErrorData::new(-32603, "Failed to load drug data")
    })?;

    log::info!("get_all_drug request handled successfully");
    serde_json::to_value(records).map_err(ErrorData::from)
}

pub async fn get_all_vaccine_handler() -> Result<Value, ErrorData> {
    log::info!("Handling get_all_vaccines request");

    let path = "statics/data/vaccine.csv";
    let records = read_vaccine_csv(path).map_err(|e| {
        log::error!("Failed to read vaccine data from '{}': {}", path, e);
        ErrorData::new(-32603, "Failed to load vaccine data")
    })?;

    let vaccines = get_all_vaccines(&records);

    log::info!("get_all_vaccines request handled successfully");
    serde_json::to_value(vaccines).map_err(ErrorData::from)
}

pub async fn get_all_pincode_city(
    input: PinCodeInput,
    state: AppState,
) -> Result<Value, ErrorData> {
    log::info!(
        "Handling get_all_cities request for pincode: {}",
        input.pin_code
    );
    let mut tx = state.read_pool.begin().await?;

    let query = "SELECT * FROM location WHERE pin_code = $1";

    let rows: Vec<LocationRecord> = sqlx::query_as::<_, LocationRecord>(query)
        .bind(&input.pin_code)
        .fetch_all(&mut *tx)
        .await?;

    serde_json::to_value(rows).map_err(ErrorData::from)
}

pub async fn get_all_organization(state: AppState) -> Result<Value, ErrorData> {
    log::info!("Handling get_all_organization request");

    let mut tx = state.read_pool.begin().await?;

    let query = r#"
        SELECT
        data ->> 'id' AS id,
        data ->> 'name' AS name,
        data ->> 'details' AS details,
        data ->> 'email' AS email,
        data ->> 'phone_number' AS mobile_number,
        data -> 'address' ->> 'pin_code' AS pin_code,
        data -> 'address' ->> 'city' AS city,
        data -> 'address' ->> 'state' AS state,
        data -> 'address' ->> 'address_line' AS address_line,
        data -> 'address' ->> 'country' AS country
    FROM organization_table_state AS org
    WHERE data ->> 'is_deleted' = $1;
    "#;
    let rows: Vec<Getorganization> = sqlx::query_as::<_, Getorganization>(query)
        .bind(false)
        .fetch_all(&mut *tx)
        .await?;

    log::info!("get_all_organization request handled successfully");
    serde_json::to_value(rows).map_err(ErrorData::from)
}

pub async fn get_by_id_organization(
    params: OrganizationsByID,
    state: AppState,
) -> Result<Value, ErrorData> {
    log::info!(
        "Handling get_by_id_organization request for organization_id: {}",
        params.org_id
    );

    let mut tx = state.read_pool.begin().await?;

    let query = r#"
        SELECT
        data ->> 'id' AS id,
        data ->> 'name' AS name,
        data ->> 'details' AS details,
        data ->> 'email' AS email,
        data ->> 'phone_number' AS mobile_number,
        data -> 'address' ->> 'pin_code' AS pin_code,
        data -> 'address' ->> 'city' AS city,
        data -> 'address' ->> 'state' AS state,
        data -> 'address' ->> 'address_line' AS address_line,
        data -> 'address' ->> 'country' AS country
    FROM organization_table_state AS org
    WHERE data ->> 'id' = $1 AND data ->> 'is_deleted' = $2;

    "#;
    let rows: Vec<Getorganization> = sqlx::query_as::<_, Getorganization>(query)
        .bind(&params.org_id)
        .bind(false)
        .fetch_all(&mut *tx)
        .await?;

    log::info!(
        "get_by_id_organization request handled successfully for organization_id: {}",
        params.org_id
    );
    serde_json::to_value(rows).map_err(ErrorData::from)
}

pub async fn get_all_pincode_handler(state: AppState) -> Result<Value, ErrorData> {
    log::info!("Handling get_all_pincode request");

    let mut tx = state.read_pool.begin().await?;

    let query = "SELECT pin_code FROM location";

    let rows: Vec<String> = sqlx::query_scalar(query).fetch_all(&mut *tx).await?;

    log::info!("get_all_pincodes request handled successfully");
    serde_json::to_value(rows).map_err(ErrorData::from)
}
