use crate::common::app_state::AppStateContext;
use axum::{Extension, Router, debug_handler, routing::get};

#[debug_handler]
async fn status() -> &'static str {
    "OK"
}

#[debug_handler]
async fn health_check() -> &'static str {
    "Healthy"
}

#[debug_handler]
async fn readiness_check(Extension(app_state): Extension<AppStateContext>) -> &'static str {
    let healthy = app_state.database.health().await;
    if healthy.is_err() {
        return "Not Ready";
    };
    "Ready"
}

pub fn routes() -> Router {
    Router::new()
        .route("/", get(status))
        .route("/health", get(health_check))
        .route("/readiness", get(readiness_check))
}
