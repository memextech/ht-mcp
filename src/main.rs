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
                    "tools": crate::mcp::tools::get_tool_definitions()
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
                            let text_response = format_tool_response(tool_name, &result);
                            json!({
                                "jsonrpc": "2.0",
                                "id": id,
                                "result": {
                                    "content": [
                                        {
                                            "type": "text",
                                            "text": text_response
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

/// Format tool response into human-readable text matching TypeScript implementation
fn format_tool_response(tool_name: &str, result: &serde_json::Value) -> String {
    match tool_name {
        "ht_create_session" => {
            let session_id = result["sessionId"].as_str().unwrap_or("unknown");
            let web_server_enabled = result["webServerEnabled"].as_bool().unwrap_or(false);
            let web_server_url = result["webServerUrl"].as_str();

            let web_server_info = if web_server_enabled {
                if let Some(url) = web_server_url {
                    format!("\n\n🌐 Web server enabled! View live terminal at: {}", url)
                } else {
                    "\n\n🌐 Web server enabled! Check console for URL.".to_string()
                }
            } else {
                String::new()
            };

            format!(
                "HT session created successfully!\n\nSession ID: {}\n\nYou can now use this session ID with other HT tools to send commands and take snapshots.{}",
                session_id, web_server_info
            )
        }
        "ht_send_keys" => {
            let session_id = result["sessionId"].as_str().unwrap_or("unknown");
            let keys = result["keys"]
                .as_array()
                .map(|arr| {
                    arr.iter()
                        .map(|v| v.as_str().unwrap_or("").to_string())
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();

            format!(
                "Keys sent successfully to session {}\n\nKeys: {}",
                session_id,
                serde_json::to_string(&keys).unwrap_or_else(|_| "[]".to_string())
            )
        }
        "ht_take_snapshot" => {
            let session_id = result["sessionId"].as_str().unwrap_or("unknown");
            let snapshot = result["snapshot"].as_str().unwrap_or("No snapshot data");

            format!(
                "Terminal Snapshot (Session: {})\n\n```\n{}\n```",
                session_id, snapshot
            )
        }
        "ht_execute_command" => {
            let command = result["command"].as_str().unwrap_or("unknown");
            let output = result["output"].as_str().unwrap_or("No output");

            format!(
                "Command executed: {}\n\nTerminal Output:\n```\n{}\n```",
                command, output
            )
        }
        "ht_list_sessions" => {
            let count = result["count"].as_u64().unwrap_or(0);
            let default_sessions = vec![];
            let sessions = result["sessions"].as_array().unwrap_or(&default_sessions);

            if sessions.is_empty() {
                format!("Active HT Sessions ({}):\n\nNo active sessions", count)
            } else {
                let session_list: Vec<String> = sessions
                    .iter()
                    .map(|session| {
                        let id = session["id"].as_str().unwrap_or("unknown");
                        let is_alive = session["isAlive"].as_bool().unwrap_or(false);
                        let created_at = session["createdAt"].as_u64().unwrap_or(0);

                        format!(
                            "- {} ({}) - Created: {}",
                            id,
                            if is_alive { "alive" } else { "dead" },
                            created_at
                        )
                    })
                    .collect();

                format!(
                    "Active HT Sessions ({}):\n\n{}",
                    count,
                    session_list.join("\n")
                )
            }
        }
        "ht_close_session" => {
            let session_id = result["sessionId"].as_str().unwrap_or("unknown");
            format!("Session {} closed successfully.", session_id)
        }
        _ => {
            // Fallback to JSON pretty print for unknown tools
            serde_json::to_string_pretty(result)
                .unwrap_or_else(|_| "Error formatting result".to_string())
        }
    }
}
