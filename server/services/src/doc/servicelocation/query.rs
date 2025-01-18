use db_services::tables::servicelocation::GetServiceLocation;
use serde_json::Value;

use crate::{app_state::AppState, method::convention::ErrorData};

pub async fn get_all_service_location(state: AppState, org_id: String) -> Result<Value, ErrorData> {
    log::info!("Handling get_all_service_location request");

    let mut tx = state.read_pool.begin().await?;

    let query = r#"
        SELECT
        data ->> 'id' AS id,
        data ->> 'org_id' AS org_id,
        data ->> 'org_name' AS org_name,
        data ->> 'service_location_name' AS service_location_name,
        data -> 'address' ->> 'pin_code' AS pin_code,
        data -> 'address' ->> 'city' AS city,
        data -> 'address' ->> 'state' AS state,
        data -> 'address' ->> 'address_line' AS address_line,
        data -> 'address' ->> 'country' AS country
    FROM servicelocation_table_state
    WHERE data ->> 'is_deleted' = $1 AND data ->> 'org_id' = $2;
    "#;
    let rows: Vec<GetServiceLocation> = sqlx::query_as::<_, GetServiceLocation>(query)
        .bind(false)
        .bind(org_id)
        .fetch_all(&mut *tx)
        .await?;

    log::info!("get_all_service_location request handled successfully");
    serde_json::to_value(rows).map_err(ErrorData::from)
}
