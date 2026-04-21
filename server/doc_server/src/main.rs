mod clients;
mod compaction;
mod health_check;
mod metrics;
mod rpc_handler;
mod server;
use anyhow::Result;
pub use rpc_handler::rpc_handler;
mod auth_rpc_handler;
pub use auth_rpc_handler::auth_rpc_handler;
extern crate validator;
pub mod medicine_prediction;
pub mod phone_code;

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    // Bridge `log` crate macros to tracing, then set up a tracing subscriber.
    // In production (APP_ENVIRONMENT=production) emit JSON lines; otherwise
    // emit human-readable output.
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

    metrics::init_metrics();
    server::server().await
}
