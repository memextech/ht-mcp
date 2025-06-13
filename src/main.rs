//! HT-MCP main binary

#![allow(dead_code)] // Allow dead code during development
#![allow(clippy::new_without_default)] // Allow new() without Default during development
#![allow(clippy::to_string_in_format_args)] // Allow to_string in format args
#![allow(clippy::collapsible_if)] // Allow nested if statements for clarity
#![allow(clippy::collapsible_match)] // Allow nested match statements for clarity
#![allow(clippy::needless_return)] // Allow explicit returns for clarity

use clap::Parser;
use serde_json::{json, Value};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tracing::{error, info, warn};

mod error;
mod ht_integration;
mod mcp;
mod transport;

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

    // Initialize logging to stderr (MCP protocol uses stdout for JSON-RPC)
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(if cli.debug {
            tracing::Level::DEBUG
        } else {
            tracing::Level::INFO
        })
        .with_writer(std::io::stderr)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    info!("Starting HT MCP Server v{}", env!("CARGO_PKG_VERSION"));

    // Create MCP server
    let mut server = HtMcpServer::new();

    info!("HT MCP Server created successfully");
    info!("Server info: {:?}", server.server_info());

    // Set up stdio transport for MCP protocol
    let stdin = tokio::io::stdin();
    let mut reader = BufReader::new(stdin);
    let mut stdout = tokio::io::stdout();

    let mut line = String::new();
    loop {
        line.clear();
        match reader.read_line(&mut line).await {
            Ok(0) => {
                // EOF
                info!("Client disconnected");
                break;
            }
            Ok(_) => {
                let trimmed = line.trim();
                if trimmed.is_empty() {
                    continue;
                }

                info!("Received request: {}", trimmed);

                // Parse JSON-RPC request
                match serde_json::from_str::<Value>(trimmed) {
                    Ok(request) => {
                        let response = handle_request(&mut server, request).await;

                        // Only send response if it's not null (i.e., not a notification)
                        if !response.is_null() {
                            let response_str = serde_json::to_string(&response).unwrap();

                            if let Err(e) = stdout.write_all(response_str.as_bytes()).await {
                                error!("Failed to write response: {}", e);
                                break;
                            }
                            if let Err(e) = stdout.write_all(b"\n").await {
                                error!("Failed to write newline: {}", e);
                                break;
                            }
                            if let Err(e) = stdout.flush().await {
                                error!("Failed to flush stdout: {}", e);
                                break;
                            }

                            info!("Sent response: {}", response_str);
                        }
                    }
                    Err(e) => {
                        warn!("Failed to parse JSON request: {}", e);
                        let error_response = json!({
                            "jsonrpc": "2.0",
                            "id": null,
                            "error": {
                                "code": -32700,
                                "message": "Parse error"
                            }
                        });
                        let response_str = serde_json::to_string(&error_response).unwrap();
                        let _ = stdout.write_all(response_str.as_bytes()).await;
                        let _ = stdout.write_all(b"\n").await;
                        let _ = stdout.flush().await;
                    }
                }
            }
            Err(e) => {
                error!("Failed to read from stdin: {}", e);
                break;
            }
        }
    }

    info!("HT MCP Server shutting down");
    Ok(())
}

async fn handle_request(server: &mut HtMcpServer, request: Value) -> Value {
    let method = request.get("method").and_then(|m| m.as_str()).unwrap_or("");
    let id = request.get("id");
    let params = request.get("params");

    match method {
        "initialize" => {
            info!("Handling initialize request");
            json!({
                "jsonrpc": "2.0",
                "id": id,
                "result": {
                    "protocolVersion": "2024-11-05",
                    "capabilities": {
                        "tools": {}
                    },
                    "serverInfo": {
                        "name": "ht-mcp-server",
                        "version": env!("CARGO_PKG_VERSION")
                    }
                }
            })
        }
        "notifications/initialized" => {
            info!("Client initialized");
            // No response needed for notifications
            return json!(null);
        }
        "tools/list" => {
            info!("Listing tools");
            json!({
                "jsonrpc": "2.0",
                "id": id,
                "result": {
                    "tools": [
                        {
                            "name": "ht_create_session",
                            "description": "Create a new HT terminal session",
                            "inputSchema": {
                                "type": "object",
                                "properties": {
                                    "command": {
                                        "type": "array",
                                        "items": { "type": "string" },
                                        "description": "Command to run (default: [\"bash\"])"
                                    },
                                    "enableWebServer": {
                                        "type": "boolean",
                                        "description": "Whether to enable web server for this session"
                                    }
                                }
                            }
                        },
                        {
                            "name": "ht_take_snapshot",
                            "description": "Take a snapshot of a terminal session",
                            "inputSchema": {
                                "type": "object",
                                "properties": {
                                    "session_id": {
                                        "type": "string",
                                        "description": "ID of the session to snapshot"
                                    }
                                },
                                "required": ["session_id"]
                            }
                        },
                        {
                            "name": "ht_send_keys",
                            "description": "Send keys to a terminal session",
                            "inputSchema": {
                                "type": "object",
                                "properties": {
                                    "session_id": {
                                        "type": "string",
                                        "description": "ID of the session"
                                    },
                                    "keys": {
                                        "type": "array",
                                        "items": { "type": "string" },
                                        "description": "Keys to send"
                                    }
                                },
                                "required": ["session_id", "keys"]
                            }
                        },
                        {
                            "name": "ht_execute_command",
                            "description": "Execute a command in a terminal session",
                            "inputSchema": {
                                "type": "object",
                                "properties": {
                                    "session_id": {
                                        "type": "string",
                                        "description": "ID of the session"
                                    },
                                    "command": {
                                        "type": "string",
                                        "description": "Command to execute"
                                    }
                                },
                                "required": ["session_id", "command"]
                            }
                        },
                        {
                            "name": "ht_close_session",
                            "description": "Close a terminal session",
                            "inputSchema": {
                                "type": "object",
                                "properties": {
                                    "session_id": {
                                        "type": "string",
                                        "description": "ID of the session to close"
                                    }
                                },
                                "required": ["session_id"]
                            }
                        },
                        {
                            "name": "ht_list_sessions",
                            "description": "List all active terminal sessions",
                            "inputSchema": {
                                "type": "object",
                                "properties": {}
                            }
                        }
                    ]
                }
            })
        }
        "tools/call" => {
            info!("Tool call received");
            if let Some(params) = params {
                if let Some(tool_name) = params.get("name").and_then(|n| n.as_str()) {
                    let arguments = params.get("arguments").cloned().unwrap_or(json!({}));

                    match server.handle_tool_call(tool_name, arguments).await {
                        Ok(result) => {
                            json!({
                                "jsonrpc": "2.0",
                                "id": id,
                                "result": {
                                    "content": [
                                        {
                                            "type": "text",
                                            "text": serde_json::to_string_pretty(&result).unwrap_or_else(|_| "Error serializing result".to_string())
                                        }
                                    ]
                                }
                            })
                        }
                        Err(e) => {
                            error!("Tool call failed: {}", e);
                            json!({
                                "jsonrpc": "2.0",
                                "id": id,
                                "error": {
                                    "code": -32603,
                                    "message": format!("Tool call failed: {}", e)
                                }
                            })
                        }
                    }
                } else {
                    json!({
                        "jsonrpc": "2.0",
                        "id": id,
                        "error": {
                            "code": -32602,
                            "message": "Missing tool name in parameters"
                        }
                    })
                }
            } else {
                json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "error": {
                        "code": -32602,
                        "message": "Missing parameters"
                    }
                })
            }
        }
        _ => {
            warn!("Unknown method: {}", method);
            json!({
                "jsonrpc": "2.0",
                "id": id,
                "error": {
                    "code": -32601,
                    "message": format!("Method not found: {}", method)
                }
            })
        }
    }
}
