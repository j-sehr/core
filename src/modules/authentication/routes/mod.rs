mod account;
mod authentication;
mod session;

pub fn routes() -> axum::Router {
    axum::Router::new()
        .nest("/auth", authentication::routes())
        .nest("/account", account::routes())
        .nest("/session", session::routes())
}
