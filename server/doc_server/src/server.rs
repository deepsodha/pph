use crate::clients::clients_config;
use crate::health_check::health_check;
use crate::phone_code::phone_code;
use crate::{auth_rpc_handler, rpc_handler};
use actix_cors::Cors;
use actix_web::http::header;
use actix_web::{middleware, middleware::Logger, web, App, HttpServer};
use anyhow::Result;
use biscuit_actix_middleware::BiscuitMiddleware;
use biscuit_auth::{PrivateKey, PublicKey};
use cosmo_store_sqlx_sqlite::event_store_sqlx_sqlite::EventStoreSQLXSqlite;
use dotenv::var;
use features::setup::prelude::location::create_location_table;
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

    let read_pool_clone = read_pool.clone();
    tokio::spawn(async move {
        if let Err(err) = create_location_table(read_pool_clone).await {
            eprintln!("Error while running create_location_table: {:?}", err);
        }
    });

    let server = HttpServer::new(move || {
        let cors_base = Cors::default()
            .allowed_methods(vec!["POST", "GET"])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            .allowed_header(header::CONTENT_TYPE)
            .supports_credentials()
            .max_age(3600);

        let cors = match app_environment {
            AppEnv::Development => cors_base
                .allowed_origin("http://localhost:5000")
                .allowed_origin("http://127.0.0.1:5174")
                .allowed_origin("http://127.0.0.1:5173")
                .allowed_origin("http://localhost:5173")
                .allowed_origin("https://pi-gp.fuzzyapps.in")
                .allowed_origin("http://localhost:8080"),
            AppEnv::Production => cors_base
                .allowed_origin("http://localhost:5000")
                .allowed_origin("http://127.0.0.1:5174")
                .allowed_origin("https://pi-gp.fuzzyapps.in")
                .allowed_origin("http://localhost:8080"),
            AppEnv::Staging => cors_base
                .allowed_origin("https://pi-gp.fuzzyapps.in")
                .allowed_origin("http://127.0.0.1:5174")
                .allowed_origin("http://localhost:8080"),
            AppEnv::Test => cors_base,
        };
        let app_state = AppState {
            store: store.clone(),
            read_pool: read_pool.clone(),
            private_key: private_key.clone(),
            write_pool: write_pool.clone(),
        };
        App::new()
            .app_data(web::Data::new(app_state))
            .wrap(cors)
            .wrap(Logger::new("%a %{User-Agent}i - %D millisecond"))
            .wrap(middleware::Compress::default())
            .service(web::resource("/auth").route(web::post().to(auth_rpc_handler)))
            .service(
                web::resource("/api")
                    .route(web::post().to(rpc_handler))
                    .wrap(BiscuitMiddleware::new(public_key)),
            )
            .service(web::resource("/phone_code").route(web::post().to(phone_code)))
            .route("/health_check", web::get().to(health_check))
            .configure(clients_config)
    });
    let _res = server.bind("0.0.0.0:5000")?.run().await;
    Ok(())
}
