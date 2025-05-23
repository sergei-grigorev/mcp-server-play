use anyhow::Result;
use api::MyServer;
// use rmcp::{transport::stdio, ServiceExt};
use rmcp::transport::sse_server::SseServer;
use tracing::info;
use tracing_subscriber::EnvFilter;

mod api;
mod models;

const DEFAULT_BIND_ADDRESS: &str = "0.0.0.0";
const DEFAULT_PORT: u16 = 8000;

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

    // Get bind address and port from environment variables or use defaults
    let bind_address = std::env::var("BIND_ADDRESS")
        .unwrap_or_else(|_| DEFAULT_BIND_ADDRESS.to_string());
    let port = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse::<u16>().ok())
        .unwrap_or(DEFAULT_PORT);
    
    let server_address = format!("{}:{}", bind_address, port);
    info!("Attempting to bind to {}", server_address);

    let server = SseServer::serve(server_address.parse()?)
        .await
        .inspect_err(|e| {
            tracing::error!("serving error: {:?}", e);
        })?;
    info!("Server started on {}", server_address);

    let ct = server.with_service(MyServer::new);
    tokio::signal::ctrl_c().await?;

    ct.cancel();
    Ok(())
}
