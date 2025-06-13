use clap::Parser;
use tracing::{info, error};
use serde_json::{json, Value};
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

mod mcp;
mod ht_integration;
mod transport;
mod error;

use crate::mcp::server::HtMcpServer;

#[derive(Parser)]
#[command(name = "ht-mcp")]
#[command(about = "Pure Rust MCP server for headless terminal interactions")]
struct Cli {
    /// Enable debug logging
    #[arg(short, long)]
    debug: bool,
    
    /// Server name for MCP identification
    #[arg(long, default_value = "ht-mcp-server")]
    name: String,
}

// Implement ServerHandler for HtMcpServer  
#[async_trait::async_trait]
impl ServerHandler for HtMcpServer {
    async fn list_tools(&self, _params: Option<PaginatedRequestParamInner>, _context: RequestContext<RoleServer>) -> Result<ListToolsResult, ErrorData> {
        Ok(ListToolsResult {
            tools: TOOLS.as_slice().to_vec(),
            next_cursor: None,
        })
    }

    async fn call_tool(&self, params: CallToolRequestParam, _context: RequestContext<RoleServer>) -> Result<CallToolResult, ErrorData> {
        info!("Tool call received: {}", params.name);
        
        match self.handle_tool_call(&params.name, serde_json::Value::Object(params.arguments.unwrap_or_default())).await {
            Ok(result) => {
                let text_content = TextContent {
                    raw: rmcp::model::RawTextContent {
                        text: serde_json::to_string_pretty(&result)
                            .unwrap_or_else(|_| "Error serializing result".to_string()),
                    },
                    annotations: None,
                };
                Ok(CallToolResult {
                    content: vec![rmcp::model::Content::Text(text_content)],
                    is_error: None,
                })
            }
            Err(e) => {
                error!("Tool call failed: {}", e);
                Err(ErrorData::internal_error(format!("Tool execution failed: {}", e)))
            }
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    
    // Initialize logging to stderr (stdout is reserved for MCP protocol)
    let subscriber = tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_max_level(if cli.debug { tracing::Level::DEBUG } else { tracing::Level::INFO })
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    info!("Starting HT MCP Server v{}", env!("CARGO_PKG_VERSION"));

    // Create MCP server
    let server = Arc::new(HtMcpServer::new());
    
    info!("HT MCP Server created successfully");
    info!("Server info: {:?}", server.server_info());

    // Start the MCP service
    info!("Starting MCP service...");
    server.run().await?;

    Ok(())
}
