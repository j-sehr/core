pub mod status;

pub fn routes() -> axum::Router {
    axum::Router::new().nest("/status", status::routes())
}
