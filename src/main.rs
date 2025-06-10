use clap::Parser;
use tracing::info;
use rmcp::ServiceExt;
use tokio::io::{stdin, stdout};

mod mcp;
mod ht_integration;
mod transport;
mod error;

use mcp::server::HtMcpServer;

#[derive(Parser)]
#[command(name = "ht-mcp-rust")]
#[command(about = "Pure Rust MCP server for headless terminal interactions")]
struct Cli {
    /// Enable debug logging
    #[arg(short, long)]
    debug: bool,
    
    /// Server name for MCP identification
    #[arg(long, default_value = "ht-mcp-server")]
    name: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    
    // Initialize logging
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(if cli.debug { tracing::Level::DEBUG } else { tracing::Level::INFO })
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    info!("Starting HT MCP Server v{}", env!("CARGO_PKG_VERSION"));

    // Create transport (stdio)
    let transport = (stdin(), stdout());
    
    // Create MCP server
    let service = HtMcpServer::new();
    
    info!("HT MCP Server created successfully");
    info!("Server info: {:?}", service.server_info());

    // Start the MCP server
    let server = service.serve(transport).await
        .map_err(|e| {
            tracing::error!("Failed to start MCP server: {}", e);
            e
        })?;

    info!("HT MCP Server started successfully");

    // Wait for server to finish
    let quit_reason = server.waiting().await?;
    info!("Server shutdown: {:?}", quit_reason);

    Ok(())
}
