use crate::apis::clients::clients_config;
use crate::apis::doctors::doctors_config;
use crate::apis::drugs::drug_config;
use crate::apis::health_check::health_check;
use crate::apis::users::users_config;
use crate::auth::login;
use crate::clients::ui_config;
use crate::meili_queries::api::{doctor_search, drug_search, icd_search, location_search};
use actix_cors::Cors;
use actix_governor::{Governor, GovernorConfigBuilder};
use actix_web::http::header;
use actix_web::{middleware, middleware::Logger, web, App, HttpServer};
use anyhow::Result;
use biscuit_actix_middleware::BiscuitMiddleware;
use biscuit_auth::PublicKey;
use dotenv::var;
use log::debug;
use log::info;
use sqlx::migrate::{MigrateDatabase, Migrator};
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::Sqlite;
use std::path::Path;
use utils::env_helper::AppEnv;

pub async fn server() -> Result<()> {
    let app_environment = AppEnv::current_env()?;

    let read_dev_db = var("META_DB_FILE").expect("READ DB name must be set");
    let read_db_conn = format!("sqlite://{}", &read_dev_db);

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

    let read_pool = SqlitePoolOptions::new()
        .connect(&format!("sqlite:{}", &read_dev_db))
        .await?;

    let migrator = Migrator::new(Path::new("././meta/migrations")).await?;

    let public_key =
        PublicKey::from_bytes_hex(&var("PUBLIC_KEY").expect("Public is not set in .env file"))
            .expect("Failed to parse public key");
    debug!("Public Key for testing purpose: {}", public_key.clone());

    migrator
        .run(&read_pool)
        .await
        .expect("Failed to migrate the database");

    let governor_conf = GovernorConfigBuilder::default()
        .requests_per_minute(120)
        .burst_size(30)
        .finish()
        .expect("Invalid rate-limiter config");

    let server = HttpServer::new(move || {
        let governor_conf = governor_conf.clone();
        let cors_base = Cors::default()
            .allowed_methods(vec!["POST", "GET", "PUT", "DELETE"])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            .allowed_header(header::CONTENT_TYPE)
            .supports_credentials()
            .max_age(3600);

        let cors = match app_environment {
            AppEnv::Development => cors_base
                .allowed_origin("http://localhost:5000")
                .allowed_origin("http://localhost:8080")
                .allowed_origin("http://localhost:5173")
                .allowed_origin("http://127.0.0.1:5173"),
            AppEnv::Production => cors_base
                .allowed_origin("http://localhost:5000")
                .allowed_origin("http://127.0.0.1:5173")
                .allowed_origin("http://localhost:5173")
                .allowed_origin("http://localhost:3000")
                .allowed_origin("http://localhost:8080"),
            AppEnv::Staging => cors_base.allowed_origin("http://localhost:8080"),
            AppEnv::Test => cors_base,
        };
        App::new()
            .wrap(cors)
            .wrap(Governor::new(&governor_conf))
            .app_data(web::Data::new(read_pool.clone()))
            .wrap(Logger::new("%a %{User-Agent}i - %D millisecond"))
            .wrap(middleware::Compress::default())
            .service(
                web::scope("/api")
                    .wrap(BiscuitMiddleware::new(public_key))
                    .service(web::scope("/drugs").configure(drug_config))
                    .service(web::scope("/doctors").configure(doctors_config))
                    .service(web::scope("/clients").configure(clients_config))
                    .service(web::scope("/users").configure(users_config)),
            )
            .route("/doctors", web::post().to(doctor_search))
            .route("/login", web::post().to(login))
            .route("/drugs", web::post().to(drug_search))
            .route("/icd10", web::post().to(icd_search))
            .route("/location", web::post().to(location_search))
            .route("/health_check", web::get().to(health_check))
            .configure(ui_config)
    });
    let _res = server.bind("0.0.0.0:8080")?.run().await;
    Ok(())
}
