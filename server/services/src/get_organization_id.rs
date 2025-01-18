use common::dto::id::Id;

use crate::app_state::AppState;

pub async fn organization_id_fetch(
    app_state: AppState,
    organization_id: String,
) -> Result<bool, sqlx::Error> {
    let organization_ids = sqlx::query_as::<_, Id>(
        "SELECT id FROM organization_table_state WHERE data->>'is_deleted' = $1",
    )
    .bind(false)
    .fetch_all(&app_state.read_pool.clone())
    .await?;

    let mut organization_id_found = false;
    for id in organization_ids {
        if id.id == organization_id.clone() {
            organization_id_found = true;
            break;
        }
    }
    Ok(organization_id_found)
}
