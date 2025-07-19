use crate::error::{HtMcpError, Result};
use crate::ht_integration::SessionManager;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::info;

pub struct HtMcpServer {
    session_manager: Arc<Mutex<SessionManager>>,
    server_info: ServerInfo,
    call_counter: AtomicU64,
}

#[derive(Debug, Clone)]
pub struct ServerInfo {
    pub name: String,
    pub version: String,
}

impl HtMcpServer {
    pub fn new() -> Self {
        Self {
            session_manager: Arc::new(Mutex::new(SessionManager::new())),
            server_info: ServerInfo {
                name: "ht-mcp-server".to_string(),
                version: env!("CARGO_PKG_VERSION").to_string(),
            },
            call_counter: AtomicU64::new(0),
        }
    }

    pub fn new_with_port_config(web_port_config: Option<u16>) -> Self {
        Self {
            session_manager: Arc::new(Mutex::new(SessionManager::new_with_port_config(web_port_config))),
            server_info: ServerInfo {
                name: "ht-mcp-server".to_string(),
                version: env!("CARGO_PKG_VERSION").to_string(),
            },
            call_counter: AtomicU64::new(0),
        }
    }

    pub fn server_info(&self) -> &ServerInfo {
        &self.server_info
    }

    pub async fn handle_tool_call(
        &self,
        tool_name: &str,
        arguments: serde_json::Value,
    ) -> Result<serde_json::Value> {
        let call_id = self.call_counter.fetch_add(1, Ordering::SeqCst);
        info!("=== TOOL CALL #{} START: {} ===", call_id, tool_name);

        let mut session_manager = self.session_manager.lock().await;

        match tool_name {
            "ht_create_session" => {
                let args: crate::mcp::types::CreateSessionArgs = serde_json::from_value(arguments)
                    .map_err(|e| HtMcpError::InvalidRequest(format!("Invalid arguments: {}", e)))?;
                session_manager.create_session(args).await
            }
            "ht_send_keys" => {
                // Debug: Log the raw arguments JSON
                info!(
                    "CALL #{}: Raw ht_send_keys arguments: {}",
                    call_id,
                    serde_json::to_string_pretty(&arguments).unwrap_or_default()
                );

                let args: crate::mcp::types::SendKeysArgs = serde_json::from_value(arguments)
                    .map_err(|e| HtMcpError::InvalidRequest(format!("Invalid arguments: {}", e)))?;

                // Debug: Log the parsed arguments
                info!(
                    "CALL #{}: Parsed ht_send_keys args: sessionId={}, keys_count={}",
                    call_id,
                    args.session_id,
                    args.keys.len()
                );
                for (i, key) in args.keys.iter().enumerate() {
                    info!("CALL #{}: keys[{}] = '{}'", call_id, i, key);
                }

                let result = session_manager.send_keys(args).await;
                info!("=== TOOL CALL #{} END ===", call_id);
                result
            }
            "ht_take_snapshot" => {
                let args: crate::mcp::types::TakeSnapshotArgs = serde_json::from_value(arguments)
                    .map_err(|e| {
                    HtMcpError::InvalidRequest(format!("Invalid arguments: {}", e))
                })?;
                session_manager.take_snapshot(args).await
            }
            "ht_execute_command" => {
                let args: crate::mcp::types::ExecuteCommandArgs = serde_json::from_value(arguments)
                    .map_err(|e| HtMcpError::InvalidRequest(format!("Invalid arguments: {}", e)))?;
                session_manager.execute_command(args).await
            }
            "ht_list_sessions" => session_manager.list_sessions().await,
            "ht_close_session" => {
                let args: crate::mcp::types::CloseSessionArgs = serde_json::from_value(arguments)
                    .map_err(|e| {
                    HtMcpError::InvalidRequest(format!("Invalid arguments: {}", e))
                })?;
                session_manager.close_session(args).await
            }
            _ => Err(HtMcpError::InvalidRequest(format!(
                "Unknown tool: {}",
                tool_name
            ))),
        }
    }
}
