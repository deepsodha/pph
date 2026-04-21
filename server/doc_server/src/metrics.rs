use actix_web::HttpResponse;
use serde::Serialize;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::SystemTime;

static REQUEST_COUNT: AtomicU64 = AtomicU64::new(0);
static START_TIME_SECS: std::sync::OnceLock<u64> = std::sync::OnceLock::new();

pub fn init_metrics() {
    let secs = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    START_TIME_SECS.get_or_init(|| secs);
}

pub fn increment_request_count() {
    REQUEST_COUNT.fetch_add(1, Ordering::Relaxed);
}

#[derive(Serialize)]
struct Metrics {
    requests_total: u64,
    uptime_seconds: u64,
}

pub async fn metrics_handler() -> HttpResponse {
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let start = START_TIME_SECS.get().copied().unwrap_or(now);

    HttpResponse::Ok().json(Metrics {
        requests_total: REQUEST_COUNT.load(Ordering::Relaxed),
        uptime_seconds: now.saturating_sub(start),
    })
}
