use axum::Router;

mod status;

pub fn routes() -> Router {
    Router::new().nest("/status", status::routes())
}
