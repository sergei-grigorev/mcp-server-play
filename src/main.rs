use anyhow::Result;
use api::MyServer;
// use rmcp::{transport::stdio, ServiceExt};
use rmcp::transport::sse_server::SseServer;
use tracing::info;
use tracing_subscriber::EnvFilter;

mod api;
mod models;

const BIND_ADDRESS: &str = "0.0.0.0:8000";

#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables from .env file
    dotenv::dotenv().ok();

    // Initialize the tracing subscriber with file and stdout logging
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .init();

    info!("Starting MCP Weather and Time Server");

    // let service = MyServer::new().serve(stdio()).await.inspect_err(|e| {
    //     tracing::error!("serving error: {:?}", e);
    // })?;
    // service.waiting().await?;

    let server = SseServer::serve(BIND_ADDRESS.parse()?)
        .await
        .inspect_err(|e| {
            tracing::error!("serving error: {:?}", e);
        })?;
    info!("Server started on {}", BIND_ADDRESS);

    let ct = server.with_service(MyServer::new);
    tokio::signal::ctrl_c().await?;

    ct.cancel();
    Ok(())
}
