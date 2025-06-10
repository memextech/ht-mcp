use std::sync::Arc;
use rmcp::{
    Error as McpError, RoleServer, ServerHandler, model::*, schemars,
    service::RequestContext, tool,
};
use crate::ht_integration::SessionManager;
use tokio::sync::Mutex;
use crate::mcp::types::*;

#[derive(Clone)]
pub struct HtMcpServer {
    session_manager: Arc<Mutex<SessionManager>>,
    name: String,
}

#[tool(tool_box)]
impl HtMcpServer {
    pub fn new(name: &str) -> Self {
        Self {
            session_manager: Arc::new(Mutex::new(SessionManager::new())),
            name: name.to_string(),
        }
    }

    #[tool(description = "Create a new HT session. Returns a session ID that can be used with other HT tools. Optionally enable web server for live terminal preview.")]
    async fn ht_create_session(
        &self,
        #[tool(param)]
        #[schemars(description = "Command to run in the terminal")]
        command: Option<Vec<String>>,
        #[tool(param)]  
        #[schemars(description = "Enable HT web server for live terminal preview")]
        enable_web_server: Option<bool>,
    ) -> Result<CallToolResult, McpError> {
        let mut session_manager = self.session_manager.lock().await;
        let args = CreateSessionArgs { command, enable_web_server };
        
        match session_manager.create_session(args).await {
            Ok(result) => {
                let session_data: serde_json::Value = result;
                let session_id = session_data["sessionId"].as_str().unwrap_or("unknown");
                let web_enabled = session_data["webServerEnabled"].as_bool().unwrap_or(false);
                let web_url = session_data["webServerUrl"].as_str();
                
                let web_server_info = if web_enabled {
                    if let Some(url) = web_url {
                        format!("\n\n🌐 Web server enabled! View live terminal at: {}", url)
                    } else {
                        "\n\n🌐 Web server enabled! Check console for URL.".to_string()
                    }
                } else {
                    "".to_string()
                };
                
                let message = format!(
                    "HT session created successfully!\n\nSession ID: {}\n\nYou can now use this session ID with other HT tools to send commands and take snapshots.{}",
                    session_id, web_server_info
                );
                
                Ok(CallToolResult::success(vec![Content::text(message)]))
            },
            Err(e) => Err(McpError::internal_error(format!("Error creating HT session: {}", e), None))
        }
    }

    #[tool(description = "Send keys to an HT session. Keys can include text and special keys like 'Enter', 'Down', 'Up', '^c' (Ctrl+C), etc.")]
    async fn ht_send_keys(
        &self,
        #[tool(param)]
        #[schemars(description = "HT session ID")]
        session_id: String,
        #[tool(param)]
        #[schemars(description = "Array of keys to send")]
        keys: Vec<String>,
    ) -> Result<CallToolResult, McpError> {
        let mut session_manager = self.session_manager.lock().await;
        let args = SendKeysArgs { session_id: session_id.clone(), keys: keys.clone() };
        
        match session_manager.send_keys(args).await {
            Ok(_result) => {
                let message = format!(
                    "Keys sent successfully to session {}\n\nKeys: {}",
                    session_id,
                    serde_json::to_string(&keys).unwrap_or_else(|_| "[]".to_string())
                );
                Ok(CallToolResult::success(vec![Content::text(message)]))
            },
            Err(e) => Err(McpError::internal_error(format!("Error sending keys: {}", e), None))
        }
    }

    #[tool(description = "Take a snapshot of the current terminal state. Returns the terminal content as text.")]
    async fn ht_take_snapshot(
        &self,
        #[tool(param)]
        #[schemars(description = "HT session ID")]
        session_id: String,
    ) -> Result<CallToolResult, McpError> {
        let session_manager = self.session_manager.lock().await;
        let args = TakeSnapshotArgs { session_id };
        
        match session_manager.take_snapshot(args).await {
            Ok(result) => {
                let snapshot_data: serde_json::Value = result;
                let session_id = snapshot_data["sessionId"].as_str().unwrap_or("unknown");
                let snapshot = snapshot_data["snapshot"].as_str().unwrap_or("No snapshot data");
                
                let message = format!(
                    "Terminal Snapshot (Session: {})\n\n```\n{}\n```",
                    session_id, snapshot
                );
                Ok(CallToolResult::success(vec![Content::text(message)]))
            },
            Err(e) => Err(McpError::internal_error(format!("Error taking snapshot: {}", e), None))
        }
    }

    #[tool(description = "Execute a command in the terminal and return the output. This combines sending the command + Enter key + taking a snapshot.")]
    async fn ht_execute_command(
        &self,
        #[tool(param)]
        #[schemars(description = "HT session ID")]
        session_id: String,
        #[tool(param)]
        #[schemars(description = "Command to execute in the terminal")]
        command: String,
    ) -> Result<CallToolResult, McpError> {
        let mut session_manager = self.session_manager.lock().await;
        let args = ExecuteCommandArgs { session_id, command };
        
        match session_manager.execute_command(args).await {
            Ok(result) => {
                let command_data: serde_json::Value = result;
                let command = command_data["command"].as_str().unwrap_or("unknown");
                let output = command_data["output"].as_str().unwrap_or("No output");
                
                let message = format!(
                    "Command executed: {}\n\nTerminal Output:\n```\n{}\n```",
                    command, output
                );
                Ok(CallToolResult::success(vec![Content::text(message)]))
            },
            Err(e) => Err(McpError::internal_error(format!("Error executing command: {}", e), None))
        }
    }

    #[tool(description = "List all active HT sessions.")]
    async fn ht_list_sessions(&self) -> Result<CallToolResult, McpError> {
        let session_manager = self.session_manager.lock().await;
        match session_manager.list_sessions().await {
            Ok(result) => {
                let sessions_data: serde_json::Value = result;
                let empty_sessions = vec![];
                let sessions = sessions_data["sessions"].as_array().unwrap_or(&empty_sessions);
                let count = sessions_data["count"].as_u64().unwrap_or(0);
                
                let sessions_list = if sessions.is_empty() {
                    "No active sessions".to_string()
                } else {
                    sessions.iter()
                        .map(|session| {
                            let id = session["id"].as_str().unwrap_or("unknown");
                            let is_alive = session["isAlive"].as_bool().unwrap_or(false);
                            let created_at = session["createdAt"].as_u64().unwrap_or(0);
                            let created_date = chrono::DateTime::from_timestamp(created_at as i64, 0)
                                .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
                                .unwrap_or_else(|| created_at.to_string());
                            
                            format!("- {} ({}) - Created: {}", 
                                id, 
                                if is_alive { "alive" } else { "dead" }, 
                                created_date
                            )
                        })
                        .collect::<Vec<_>>()
                        .join("\n")
                };
                
                let message = format!(
                    "Active HT Sessions ({}):\n\n{}",
                    count, sessions_list
                );
                Ok(CallToolResult::success(vec![Content::text(message)]))
            },
            Err(e) => Err(McpError::internal_error(format!("Error listing sessions: {}", e), None))
        }
    }

    #[tool(description = "Close an HT session and clean up resources.")]
    async fn ht_close_session(
        &self,
        #[tool(param)]
        #[schemars(description = "HT session ID to close")]
        session_id: String,
    ) -> Result<CallToolResult, McpError> {
        let mut session_manager = self.session_manager.lock().await;
        let args = CloseSessionArgs { session_id: session_id.clone() };
        
        match session_manager.close_session(args).await {
            Ok(_result) => {
                let message = format!("Session {} closed successfully.", session_id);
                Ok(CallToolResult::success(vec![Content::text(message)]))
            },
            Err(e) => Err(McpError::internal_error(format!("Error closing session: {}", e), None))
        }
    }
}

#[tool(tool_box)]
impl ServerHandler for HtMcpServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder()
                .enable_tools()
                .build(),
            server_info: Implementation {
                name: self.name.clone(),
                version: env!("CARGO_PKG_VERSION").to_string(),
            },
            instructions: Some("This server provides headless terminal (HT) functionality for AI assistants. Create terminal sessions, send commands, and capture output through a clean JSON API. Optionally enable web server for live terminal preview.".to_string()),
        }
    }

    async fn initialize(
        &self,
        _request: InitializeRequestParam,
        _context: RequestContext<RoleServer>,
    ) -> Result<InitializeResult, McpError> {
        Ok(self.get_info())
    }
}