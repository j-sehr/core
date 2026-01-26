#![allow(dead_code)]

mod connection;
mod extractors;
mod handlers;
mod models;
mod password;
mod token;
use anyhow::Result;
use axum::Router;

#[tokio::main]
async fn main() -> Result<()> {
    let db = connection::init_db().await?;
    let router = handlers::register_handlers(Router::new(), db);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, router.into_make_service()).await?;

    println!("Hello, world!");

    Ok(())
}
