use std::collections::HashMap;
use uuid::Uuid;
use ht_core::{HtLibrary, SessionConfig};
use crate::mcp::types::*;
use crate::error::{HtMcpError, Result};
use crate::ht_integration::command_bridge::CommandBridge;

#[derive(Debug, Clone)]
pub struct SessionInfo {
    pub id: String,
    pub internal_id: Uuid,
    pub created_at: std::time::SystemTime,
    pub web_server_url: Option<String>,
    pub is_alive: bool,
    pub command: Vec<String>,
}

pub struct SessionManager {
    ht_library: HtLibrary,
    sessions: HashMap<String, SessionInfo>,
    command_bridge: CommandBridge,
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            ht_library: HtLibrary::new(),
            sessions: HashMap::new(),
            command_bridge: CommandBridge::new(),
        }
    }

    pub async fn create_session(&mut self, args: CreateSessionArgs) -> Result<serde_json::Value> {
        let session_id = Uuid::new_v4().to_string();
        let command = args.command.unwrap_or_else(|| vec!["bash".to_string()]);
        let enable_web_server = args.enable_web_server.unwrap_or(false);
        
        // Create HT session configuration
        let ht_config = SessionConfig {
            command: command.clone(),
            size: (120, 40), // Default terminal size
            enable_web_server,
        };

        // Create the actual HT session
        let internal_id = self.ht_library.create_session(ht_config).await
            .map_err(|e| HtMcpError::HtLibrary(format!("Failed to create HT session: {}", e)))?;

        let web_server_url = if enable_web_server {
            Some(format!("http://127.0.0.1:{}", find_available_port().await?))
        } else {
            None
        };

        let session_info = SessionInfo {
            id: session_id.clone(),
            internal_id,
            created_at: std::time::SystemTime::now(),
            web_server_url: web_server_url.clone(),
            is_alive: true,
            command: command.clone(),
        };

        self.sessions.insert(session_id.clone(), session_info);

        let result = CreateSessionResult {
            session_id,
            message: "HT session created successfully".to_string(),
            web_server_enabled: enable_web_server,
            web_server_url,
        };

        Ok(serde_json::to_value(result)?)
    }

    pub async fn send_keys(&mut self, args: SendKeysArgs) -> Result<serde_json::Value> {
        let session = self.sessions.get(&args.session_id)
            .ok_or_else(|| HtMcpError::SessionNotFound(args.session_id.clone()))?;

        // Translate keys using command bridge
        let input_seqs = self.command_bridge.translate_keys(&args.keys)?;
        
        // Send keys to HT library
        self.ht_library.send_input(session.internal_id, input_seqs).await
            .map_err(|e| HtMcpError::HtLibrary(format!("Failed to send keys: {}", e)))?;

        tracing::info!("Sent keys {:?} to session {}", args.keys, args.session_id);

        Ok(serde_json::json!({
            "success": true,
            "message": format!("Keys sent successfully to session {}", args.session_id),
            "keys": args.keys
        }))
    }

    pub async fn take_snapshot(&self, args: TakeSnapshotArgs) -> Result<serde_json::Value> {
        let session = self.sessions.get(&args.session_id)
            .ok_or_else(|| HtMcpError::SessionNotFound(args.session_id.clone()))?;

        // Take snapshot using HT library
        let snapshot = self.ht_library.take_snapshot(session.internal_id).await
            .map_err(|e| HtMcpError::HtLibrary(format!("Failed to take snapshot: {}", e)))?;

        Ok(serde_json::json!({
            "sessionId": args.session_id,
            "snapshot": snapshot
        }))
    }

    pub async fn execute_command(&mut self, args: ExecuteCommandArgs) -> Result<serde_json::Value> {
        // Send command
        self.send_keys(SendKeysArgs {
            session_id: args.session_id.clone(),
            keys: vec![args.command.clone()],
        }).await?;

        // Send Enter
        self.send_keys(SendKeysArgs {
            session_id: args.session_id.clone(),
            keys: vec!["Enter".to_string()],
        }).await?;

        // Wait for command to execute
        tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;

        // Take snapshot
        let snapshot_result = self.take_snapshot(TakeSnapshotArgs {
            session_id: args.session_id.clone(),
        }).await?;

        Ok(serde_json::json!({
            "command": args.command,
            "sessionId": args.session_id,
            "output": snapshot_result["snapshot"]
        }))
    }

    pub async fn list_sessions(&self) -> Result<serde_json::Value> {
        // Get active sessions from HT library
        let active_ht_sessions = self.ht_library.list_sessions();
        
        let sessions: Vec<serde_json::Value> = self.sessions.values()
            .filter(|session| active_ht_sessions.contains(&session.internal_id))
            .map(|session| serde_json::json!({
                "id": session.id,
                "isAlive": active_ht_sessions.contains(&session.internal_id),
                "createdAt": session.created_at.duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default().as_secs(),
                "command": session.command,
                "webServerUrl": session.web_server_url
            }))
            .collect();

        Ok(serde_json::json!({
            "sessions": sessions,
            "count": sessions.len()
        }))
    }

    pub async fn close_session(&mut self, args: CloseSessionArgs) -> Result<serde_json::Value> {
        let session = self.sessions.remove(&args.session_id)
            .ok_or_else(|| HtMcpError::SessionNotFound(args.session_id.clone()))?;

        // Close session in HT library
        self.ht_library.close_session(session.internal_id).await
            .map_err(|e| HtMcpError::HtLibrary(format!("Failed to close session: {}", e)))?;

        tracing::info!("Closed session {}", args.session_id);

        Ok(serde_json::json!({
            "success": true,
            "message": format!("Session {} closed successfully", args.session_id)
        }))
    }
}

async fn find_available_port() -> Result<u16> {
    // Simple port finding - in a real implementation, we'd check for availability
    Ok(8080)
}