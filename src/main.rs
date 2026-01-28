use core::bootstrap;
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let (router, _, file_config) = bootstrap().await?;
    let listener = TcpListener::bind(format!("{}:{}", file_config.host, file_config.port)).await?;

    axum::serve(
        listener,
        router.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await?;
    Ok(())
}
