use crate::common::app_state::AppStateContext;
use axum::{Extension, Json, debug_handler, http::StatusCode};
use serde_json::{Value, json};

#[debug_handler]
async fn status() -> (StatusCode, Json<Value>) {
    (StatusCode::OK, Json(json!({"status": "ok"})))
}

#[debug_handler]
async fn health_check() -> (StatusCode, Json<Value>) {
    (StatusCode::OK, Json(json!({"status": "healthy"})))
}

#[debug_handler]
async fn readiness_check(Extension(app_state): Extension<AppStateContext>) -> (StatusCode, Json<Value>) {
    let healthy = app_state.database.health().await;
    if healthy.is_err() {
        return (StatusCode::SERVICE_UNAVAILABLE, Json(json!({"status": "not ready"})));
    };
    (StatusCode::OK, Json(json!({"status": "ready"})))
}

pub fn routes() -> axum::Router {
    axum::Router::new()
        .route("/", axum::routing::get(status))
        .route("/health", axum::routing::get(health_check))
        .route("/readiness", axum::routing::get(readiness_check))
}
