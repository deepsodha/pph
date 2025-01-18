use anyhow::Result;
use log::info;
use services::doc::organization::helper::read_csv;
use sqlx::{Pool, Sqlite};
use uuid::Uuid;

pub async fn create_location_table(read_pool: Pool<Sqlite>) -> Result<()> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS location (
            id TEXT PRIMARY KEY,
            pin_code TEXT NOT NULL,
            city_name TEXT NOT NULL,
            district TEXT NOT NULL,
            state_name TEXT NOT NULL,
            country_name TEXT NOT NULL
        );
        "#,
    )
    .execute(&read_pool)
    .await?;

    let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM location")
        .fetch_one(&read_pool)
        .await?;

    if row.0 == 0 {
        let path = "statics/data/locations.csv";
        let records = read_csv(path).unwrap();

        for data in records {
            let insert_location_query =
                "INSERT INTO location(id,pin_code,district,city_name,state_name,country_name)
    VALUES($1,$2,$3,$4,$5,$6)";
            let _ = sqlx::query(insert_location_query)
                .bind(Uuid::new_v4().to_string())
                .bind(data.pin_code)
                .bind(data.district)
                .bind(data.city_name)
                .bind(data.state_name)
                .bind(data.country_name)
                .execute(&read_pool)
                .await?;
        }
    } else {
        info!(target: "Setup", "Location data already exists. Please delete file to reset");
    }

    Ok(())
}
