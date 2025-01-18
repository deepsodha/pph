use db_services::tables::systemadmin::GetSystemadmin;
use serde_json::Value;

use crate::{app_state::AppState, method::convention::ErrorData};

pub async fn get_all_systemadmin(state: AppState) -> Result<Value, ErrorData> {
    log::info!("Handling get_all_systemadmin request");

    let mut tx = state.read_pool.begin().await?;

    let query = r#"
        SELECT
        data ->> 'id' AS id,
        data ->> 'org_id' AS org_id,
        data ->> 'user' ->> 'first_name' AS first_name,
        data ->> 'user' ->> 'middle_name' AS middle_name,
        data ->> 'user' ->> 'last_name' AS last_name,
        data ->> 'user' ->> 'dob' AS dob,
        data ->> 'user' ->> 'email' AS email,
        data ->> 'user' ->> 'gender' AS gender,
        data ->> 'user' ->> 'photo_url' AS photo_url,
        data -> 'address' ->> 'pin_code' AS pin_code,
        data -> 'address' ->> 'city' AS city,
        data -> 'address' ->> 'state' AS state,
        data -> 'address' ->> 'address_line' AS address_line,
        data -> 'address' ->> 'country' AS country,
        data -> 'phone' ->> 'number' AS number,
        data -> 'phone' ->> 'number_type' AS number_type,
        data -> 'government_info' ->> 'id_no' AS id_no,
        data -> 'government_info' ->> 'id_type' AS id_type  
    FROM systemadmin_table_state
    WHERE data ->> 'is_deleted' = $1;
    "#;
    let rows: Vec<GetSystemadmin> = sqlx::query_as::<_, GetSystemadmin>(query)
        .bind(false)
        .fetch_all(&mut *tx)
        .await?;

    log::info!("get_all_systemadmin request handled successfully");
    serde_json::to_value(rows).map_err(ErrorData::from)
}
