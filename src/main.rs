use clap::Parser;
use tracing::info;
use rmcp::{ServiceExt, transport::stdio};
use anyhow::Result;

mod mcp;
mod ht_integration;
mod transport;
mod error;
mod web_server;

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
async fn main() -> Result<()> {
    let cli = Cli::parse();
    
    // Initialize logging to stderr to avoid interfering with MCP protocol on stdout
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_max_level(if cli.debug { tracing::Level::DEBUG } else { tracing::Level::INFO })
        .with_ansi(false)
        .init();

    info!("Starting HT MCP Server v{}", env!("CARGO_PKG_VERSION"));

    // Create an instance of our HT MCP server
    let service = HtMcpServer::new(&cli.name)
        .serve(stdio())
        .await
        .inspect_err(|e| {
            tracing::error!("serving error: {:?}", e);
        })?;

    service.waiting().await?;
    Ok(())
}
