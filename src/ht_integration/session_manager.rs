use std::collections::HashMap;
use uuid::Uuid;
use crate::mcp::types::*;
use crate::error::{HtMcpError, Result};

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
    // For now, we'll use a mock implementation until we integrate the HT library
    sessions: HashMap<String, SessionInfo>,
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            sessions: HashMap::new(),
        }
    }

    pub async fn create_session(&mut self, args: CreateSessionArgs) -> Result<serde_json::Value> {
        let session_id = Uuid::new_v4().to_string();
        let internal_id = Uuid::new_v4();
        let command = args.command.unwrap_or_else(|| vec!["bash".to_string()]);
        
        let web_server_url = if args.enable_web_server.unwrap_or(false) {
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
            web_server_enabled: args.enable_web_server.unwrap_or(false),
            web_server_url,
        };

        Ok(serde_json::to_value(result)?)
    }

    pub async fn send_keys(&mut self, args: SendKeysArgs) -> Result<serde_json::Value> {
        let _session = self.sessions.get(&args.session_id)
            .ok_or_else(|| HtMcpError::SessionNotFound(args.session_id.clone()))?;

        // TODO: Implement actual key sending to HT library
        tracing::info!("Sending keys {:?} to session {}", args.keys, args.session_id);

        Ok(serde_json::json!({
            "success": true,
            "message": format!("Keys sent successfully to session {}", args.session_id),
            "keys": args.keys
        }))
    }

    pub async fn take_snapshot(&self, args: TakeSnapshotArgs) -> Result<serde_json::Value> {
        let _session = self.sessions.get(&args.session_id)
            .ok_or_else(|| HtMcpError::SessionNotFound(args.session_id.clone()))?;

        // TODO: Implement actual snapshot taking from HT library
        let mock_snapshot = format!("Mock terminal snapshot for session {}\n$ echo hello\nhello\n$ ", args.session_id);

        Ok(serde_json::json!({
            "sessionId": args.session_id,
            "snapshot": mock_snapshot
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
        let sessions: Vec<serde_json::Value> = self.sessions.values()
            .map(|session| serde_json::json!({
                "id": session.id,
                "isAlive": session.is_alive,
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
        let _session = self.sessions.remove(&args.session_id)
            .ok_or_else(|| HtMcpError::SessionNotFound(args.session_id.clone()))?;

        // TODO: Implement actual session closing in HT library
        tracing::info!("Closing session {}", args.session_id);

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