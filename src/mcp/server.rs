use std::sync::Arc;
use tokio::sync::Mutex;
use rmcp::{ServerHandler, model::*, service::RequestContext, Error as McpError, RoleServer};
use crate::ht_integration::SessionManager;
use crate::error::{HtMcpError, Result};
use crate::mcp::tools::get_tool_definitions;

#[derive(Clone)]
pub struct HtMcpServer {
    session_manager: Arc<Mutex<SessionManager>>,
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
        }
    }

    pub fn server_info(&self) -> InitializeResult {
        InitializeResult {
            protocol_version: ProtocolVersion::default(),
            capabilities: ServerCapabilities {
                tools: Some(ToolsCapability {
                    list_changed: Some(false),
                }),
                ..Default::default()
            },
            instructions: None,
            server_info: Implementation {
                name: "ht-mcp-server".to_string(),
                version: env!("CARGO_PKG_VERSION").to_string(),
            },
        }
    }

    pub async fn handle_tool_call(&self, tool_name: &str, arguments: serde_json::Value) -> Result<serde_json::Value> {
        let mut session_manager = self.session_manager.lock().await;
        
        match tool_name {
            "ht_create_session" => {
                let args: crate::mcp::types::CreateSessionArgs = serde_json::from_value(arguments)
                    .map_err(|e| HtMcpError::InvalidRequest(format!("Invalid arguments: {}", e)))?;
                session_manager.create_session(args).await
            }
            "ht_send_keys" => {
                let args: crate::mcp::types::SendKeysArgs = serde_json::from_value(arguments)
                    .map_err(|e| HtMcpError::InvalidRequest(format!("Invalid arguments: {}", e)))?;
                session_manager.send_keys(args).await
            }
            "ht_take_snapshot" => {
                let args: crate::mcp::types::TakeSnapshotArgs = serde_json::from_value(arguments)
                    .map_err(|e| HtMcpError::InvalidRequest(format!("Invalid arguments: {}", e)))?;
                session_manager.take_snapshot(args).await
            }
            "ht_execute_command" => {
                let args: crate::mcp::types::ExecuteCommandArgs = serde_json::from_value(arguments)
                    .map_err(|e| HtMcpError::InvalidRequest(format!("Invalid arguments: {}", e)))?;
                session_manager.execute_command(args).await
            }
            "ht_list_sessions" => {
                session_manager.list_sessions().await
            }
            "ht_close_session" => {
                let args: crate::mcp::types::CloseSessionArgs = serde_json::from_value(arguments)
                    .map_err(|e| HtMcpError::InvalidRequest(format!("Invalid arguments: {}", e)))?;
                session_manager.close_session(args).await
            }
            _ => Err(HtMcpError::InvalidRequest(format!("Unknown tool: {}", tool_name))),
        }
    }
}

impl ServerHandler for HtMcpServer {
    fn get_info(&self) -> InitializeResult {
        self.server_info()
    }

    async fn list_tools(
        &self,
        _request: PaginatedRequestParam,
        _context: RequestContext<RoleServer>,
    ) -> std::result::Result<ListToolsResult, McpError> {
        let tools = get_tool_definitions()
            .into_iter()
            .map(|tool_def| {
                let schema_map = tool_def["inputSchema"].as_object().unwrap().clone();
                Tool {
                    name: tool_def["name"].as_str().unwrap().to_string().into(),
                    description: tool_def["description"].as_str().unwrap().to_string().into(),
                    input_schema: Arc::new(schema_map),
                }
            })
            .collect();

        Ok(ListToolsResult { 
            tools,
            next_cursor: None,
        })
    }

    async fn call_tool(
        &self,
        request: CallToolRequestParam,
        _context: RequestContext<RoleServer>,
    ) -> std::result::Result<CallToolResult, McpError> {
        let arguments = match request.arguments {
            Some(args) => serde_json::Value::Object(args),
            None => serde_json::json!({}),
        };
        
        let result = self.handle_tool_call(&request.name, arguments).await
            .map_err(|e| McpError::internal_error(format!("Tool call failed: {}", e), None))?;

        Ok(CallToolResult {
            content: vec![
                Content::text(serde_json::to_string_pretty(&result).unwrap_or_else(|_| result.to_string()))
            ],
            is_error: Some(false),
        })
    }
}