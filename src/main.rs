use clap::Parser;
use tracing::{info, error};

mod mcp;
mod ht_integration;
mod transport;
mod error;

use crate::mcp::server::HtMcpServer;

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

    // Create MCP server
    let server = HtMcpServer::new();
    
    info!("HT MCP Server created successfully");
    info!("Server info: {:?}", server.server_info());

    // For now, just demonstrate that the server can be created
    // TODO: Implement actual MCP protocol handling
    
    info!("Server would start here - MCP protocol integration pending");

    Ok(())
}
