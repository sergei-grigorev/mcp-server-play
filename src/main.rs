use anyhow::Result;
use api::MyServer;
use rmcp::{transport::stdio, ServiceExt};
use tracing::info;
use tracing_subscriber::EnvFilter;

mod api;
mod models;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize the tracing subscriber with file and stdout logging
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(tracing::Level::DEBUG.into()))
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .init();

    info!("Starting MCP Weather and Time Server");

    let service = MyServer::new().serve(stdio()).await.inspect_err(|e| {
        tracing::error!("serving error: {:?}", e);
    })?;

    service.waiting().await?;
    Ok(())
}
