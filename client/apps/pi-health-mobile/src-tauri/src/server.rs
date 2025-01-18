use crate::rpc_hand;
use anyhow::Result;
use biscuit_auth::{PrivateKey, PublicKey};
use cosmo_store_sqlx_sqlite::event_store_sqlx_sqlite::EventStoreSQLXSqlite;
use dotenv::var;
use features::setup::set_up::setup;
use log::{debug, info};
use services::app_state::AppState;
use sqlx::migrate::MigrateDatabase;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::Sqlite;
use utils::env_helper::AppEnv;


pub async fn server() -> Result<()> {
    let app_environment = AppEnv::current_env()?;

    let private_key =
        PrivateKey::from_bytes_hex(&var("PRIVATE_KEY").expect("Private is not set in .env file"))
            .expect("Failed to parse private key");
    let public_key =
        PublicKey::from_bytes_hex(&var("PUBLIC_KEY").expect("Public is not set in .env file"))
            .expect("Failed to parse public key");
    debug!("Public Key for testing purpose: {}", public_key);

    

    let read_dev_db = var("READ_DB_FILE").expect("READ DB name must be set");
    let write_db = var("WRITE_DB_FILE").expect("Write DB name must be set");

    let write_db_conn = format!("sqlite://{}", &write_db);
    let read_db_conn = format!("sqlite://{}", &read_dev_db);

    if !Sqlite::database_exists(&write_db_conn)
        .await
        .unwrap_or(false)
    {
        info!("Creating database {}", write_db_conn);
        match Sqlite::create_database(&write_db_conn).await {
            Ok(_) => info!("Create db success for {}", write_db_conn),
            Err(error) => panic!("error: {}", error),
        }
    } else {
        info!("{} Database already exists", write_db_conn);
    }

    if !Sqlite::database_exists(&read_db_conn)
        .await
        .unwrap_or(false)
    {
        info!("Creating database {}", read_db_conn);
        match Sqlite::create_database(&read_dev_db).await {
            Ok(_) => info!("Create db success"),
            Err(error) => panic!("error: {}", error),
        }
    } else {
        info!("{} Database already exists", read_db_conn);
    }

    let write_pool = SqlitePoolOptions::new().connect(&write_db_conn).await?;

    let read_pool = SqlitePoolOptions::new()
        .connect(&format!("sqlite:{}", &read_dev_db))
        .await?;

    let store = EventStoreSQLXSqlite::new(&write_pool, "PiHealth").await?;

    setup(app_environment.clone(), read_pool.clone()).await?;

    let app_state = AppState {
        store: store.clone(),
        read_pool: read_pool.clone(),
        private_key: private_key.clone(),
        write_pool: write_pool.clone(),
    };

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![rpc_hand::rpc_handler])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    Ok(())
}
