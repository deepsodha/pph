use common::dto::id::Id;

use crate::app_state::AppState;
pub async fn patient_id_fetch(
    app_state: AppState,
    patient_id: String,
) -> Result<bool, sqlx::Error> {
    let patient_ids = sqlx::query_as::<_, Id>(
        "SELECT id FROM patient_table_state WHERE data->>'is_deleted' = $1",
    )
    .bind(false)
    .fetch_all(&app_state.read_pool.clone())
    .await?;

    let mut patient_id_found = false;
    for id in patient_ids {
        if id.id == patient_id {
            patient_id_found = true;
            break;
        }
    }

    Ok(patient_id_found)
}

pub async fn doctor_id_fetch(app_state: AppState, doctor_id: String) -> Result<bool, sqlx::Error> {
    let doctor_ids =
        sqlx::query_as::<_, Id>("SELECT id FROM doctor_table_state WHERE data->>'is_deleted' = $1")
            .bind(false)
            .fetch_all(&app_state.read_pool.clone())
            .await?;

    let mut doctor_id_found = false;
    for id in doctor_ids {
        if id.id == doctor_id {
            doctor_id_found = true;
            break;
        }
    }

    Ok(doctor_id_found)
}
