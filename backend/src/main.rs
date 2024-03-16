mod app;
mod error;
mod logger;
mod server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::from_filename(".env")?;
    let _ = dotenvy::from_filename_override(".env.production");
    logger::logging()?;
    println!("Hello, world!");

    let addr = format!("0.0.0.0:{}", std::env::var("SERVE_PORT")?).parse()?;
    server::listen(addr).await
}
