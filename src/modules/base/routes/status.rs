use crate::common::app_state::AppStateContext;
use axum::{Extension, debug_handler};

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

pub fn routes() -> axum::Router {
    axum::Router::new()
        .route("/", axum::routing::get(status))
        .route("/health", axum::routing::get(health_check))
        .route("/readiness", axum::routing::get(readiness_check))
}
