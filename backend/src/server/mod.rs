use axum::Router;
use std::net::SocketAddr;

pub async fn listen(addr: SocketAddr) -> anyhow::Result<()> {
    let app = Router::new()
        .route("/", axum::routing::get(|| async { "hello Rust" }))
        .nest("/", crate::app::router::compose())
        .layer(axum::extract::DefaultBodyLimit::max(1024 * 1024));

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
