use anyhow::Result;
use cosmo_store_sqlx_sqlite::event_store_sqlx_sqlite::EventStoreSQLXSqlite;
use sqlx::{Pool, Sqlite};

pub async fn familyhistory_store(
    write_pool: &Pool<Sqlite>,
    org_id: &str,
) -> Result<EventStoreSQLXSqlite> {
    let store = EventStoreSQLXSqlite::new(write_pool, &format!("{}familyhistory", org_id)).await?;
    Ok(store)
}

pub async fn hospitalization_store(
    write_pool: &Pool<Sqlite>,
    org_id: &str,
) -> Result<EventStoreSQLXSqlite> {
    let store =
        EventStoreSQLXSqlite::new(write_pool, &format!("{}hospitalization", org_id)).await?;
    Ok(store)
}

pub async fn implantabledevices_store(
    write_pool: &Pool<Sqlite>,
    org_id: &str,
) -> Result<EventStoreSQLXSqlite> {
    let store =
        EventStoreSQLXSqlite::new(write_pool, &format!("{}implantabledevices", org_id)).await?;
    Ok(store)
}

pub async fn obandpregnancy_store(
    write_pool: &Pool<Sqlite>,
    org_id: &str,
) -> Result<EventStoreSQLXSqlite> {
    let store = EventStoreSQLXSqlite::new(write_pool, &format!("{}obandpregnancy", org_id)).await?;
    Ok(store)
}

pub async fn pastmedicalhistory_store(
    write_pool: &Pool<Sqlite>,
    org_id: &str,
) -> Result<EventStoreSQLXSqlite> {
    let store = EventStoreSQLXSqlite::new(write_pool, &format!("{}medicalhistory", org_id)).await?;
    Ok(store)
}

pub async fn pastsurgicalhistory_store(
    write_pool: &Pool<Sqlite>,
    org_id: &str,
) -> Result<EventStoreSQLXSqlite> {
    let store = EventStoreSQLXSqlite::new(write_pool, &format!("{}pastsurgical", org_id)).await?;
    Ok(store)
}

pub async fn socialhistory_store(
    write_pool: &Pool<Sqlite>,
    org_id: &str,
) -> Result<EventStoreSQLXSqlite> {
    let store = EventStoreSQLXSqlite::new(write_pool, &format!("{}socialhistory", org_id)).await?;
    Ok(store)
}

pub async fn appointment_store(
    write_pool: &Pool<Sqlite>,
    org_id: &str,
) -> Result<EventStoreSQLXSqlite> {
    let store = EventStoreSQLXSqlite::new(write_pool, &format!("{}appointment", org_id)).await?;
    Ok(store)
}

pub async fn doctor_store(write_pool: &Pool<Sqlite>, org_id: &str) -> Result<EventStoreSQLXSqlite> {
    let store = EventStoreSQLXSqlite::new(write_pool, &format!("{}doctor", org_id)).await?;

    Ok(store)
}

pub async fn first_user_store(write_pool: &Pool<Sqlite>, id: &str) -> Result<EventStoreSQLXSqlite> {
    let store = EventStoreSQLXSqlite::new(write_pool, &format!("{}user", id)).await?;
    Ok(store)
}

pub async fn prescription_store(
    write_pool: &Pool<Sqlite>,
    org_id: &str,
) -> Result<EventStoreSQLXSqlite> {
    let store = EventStoreSQLXSqlite::new(write_pool, &format!("{}prescription", org_id)).await?;
    Ok(store)
}

pub async fn servicelocation_store(
    write_pool: &Pool<Sqlite>,
    org_id: &str,
) -> Result<EventStoreSQLXSqlite> {
    let store =
        EventStoreSQLXSqlite::new(write_pool, &format!("{}servicelocation", org_id)).await?;
    Ok(store)
}

pub async fn staff_store(write_pool: &Pool<Sqlite>, org_id: &str) -> Result<EventStoreSQLXSqlite> {
    let store = EventStoreSQLXSqlite::new(write_pool, &format!("{}staff", org_id)).await?;
    Ok(store)
}

pub async fn user_store(write_pool: &Pool<Sqlite>, org_id: &str) -> Result<EventStoreSQLXSqlite> {
    let store = EventStoreSQLXSqlite::new(write_pool, &format!("{}user", org_id)).await?;
    Ok(store)
}

pub async fn systemadmin_store(write_pool: &Pool<Sqlite>) -> Result<EventStoreSQLXSqlite> {
    let store = EventStoreSQLXSqlite::new(write_pool, "systemadmin").await?;
    Ok(store)
}

pub async fn organization_store(write_pool: &Pool<Sqlite>) -> Result<EventStoreSQLXSqlite> {
    let store = EventStoreSQLXSqlite::new(write_pool, "organization").await?;
    Ok(store)
}

pub async fn patient_store(
    write_pool: &Pool<Sqlite>,
    org_id: &str,
) -> Result<EventStoreSQLXSqlite> {
    let store = EventStoreSQLXSqlite::new(write_pool, &format!("{}patient", org_id)).await?;
    Ok(store)
}

pub async fn note_store(write_pool: &Pool<Sqlite>, org_id: &str) -> Result<EventStoreSQLXSqlite> {
    let store = EventStoreSQLXSqlite::new(write_pool, &format!("{}note", org_id)).await?;
    Ok(store)
}
