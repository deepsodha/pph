mod server;
use anyhow::Result;
pub mod apis;
pub mod auth;
pub mod clients;
pub mod dto;
pub mod meili_queries;

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    tracing_log::LogTracer::init().ok();
    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info"));

    let is_production = std::env::var("APP_ENVIRONMENT")
        .map(|v| v == "production")
        .unwrap_or(false);

    if is_production {
        tracing_subscriber::fmt()
            .json()
            .with_env_filter(env_filter)
            .with_current_span(true)
            .init();
    } else {
        tracing_subscriber::fmt()
            .with_env_filter(env_filter)
            .init();
    }

    server::server().await
}
