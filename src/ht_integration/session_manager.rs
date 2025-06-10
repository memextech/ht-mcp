use std::collections::HashMap;
use uuid::Uuid;
use ht_core::{HtLibrary, SessionConfig, InputSeq};
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
    ht_lib: HtLibrary,
    sessions: HashMap<String, SessionInfo>,
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            ht_lib: HtLibrary::new(),
            sessions: HashMap::new(),
        }
    }

    pub async fn create_session(&mut self, args: CreateSessionArgs) -> Result<serde_json::Value> {
        let session_id = Uuid::new_v4().to_string();
        let command = args.command.unwrap_or_else(|| vec!["bash".to_string()]);
        
        // Create HT session config
        let config = SessionConfig {
            command: command.clone(),
            size: (120, 40), // Default terminal size
            enable_web_server: args.enable_web_server.unwrap_or(false),
        };

        // Create the actual HT session
        let internal_id = self.ht_lib.create_session(config).await
            .map_err(|e| HtMcpError::HtLibrary(format!("Failed to create HT session: {}", e)))?;
        
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
        let session = self.sessions.get(&args.session_id)
            .ok_or_else(|| HtMcpError::SessionNotFound(args.session_id.clone()))?;

        // Convert keys to InputSeq format
        let input_seqs: Vec<InputSeq> = args.keys.iter()
            .map(|key| {
                // Handle special keys
                match key.as_str() {
                    "Enter" => InputSeq::Standard("\r".to_string()),
                    "Tab" => InputSeq::Standard("\t".to_string()),
                    "Escape" => InputSeq::Standard("\x1b".to_string()),
                    "Backspace" => InputSeq::Standard("\x08".to_string()),
                    "Delete" => InputSeq::Standard("\x7f".to_string()),
                    "Up" => InputSeq::Cursor("\x1b[A".to_string(), "\x1bOA".to_string()),
                    "Down" => InputSeq::Cursor("\x1b[B".to_string(), "\x1bOB".to_string()),
                    "Right" => InputSeq::Cursor("\x1b[C".to_string(), "\x1bOC".to_string()),
                    "Left" => InputSeq::Cursor("\x1b[D".to_string(), "\x1bOD".to_string()),
                    _ if key.starts_with("^") => {
                        // Handle Ctrl sequences like ^C, ^D, etc.
                        let ctrl_char = key.chars().nth(1).unwrap_or('c');
                        let ctrl_code = (ctrl_char as u8 - b'a' + 1) as char;
                        InputSeq::Standard(ctrl_code.to_string())
                    }
                    _ => InputSeq::Standard(key.clone()),
                }
            })
            .collect();

        // Send to HT library
        self.ht_lib.send_input(session.internal_id, input_seqs).await
            .map_err(|e| HtMcpError::HtLibrary(format!("Failed to send keys: {}", e)))?;

        Ok(serde_json::json!({
            "success": true,
            "message": format!("Keys sent successfully to session {}", args.session_id),
            "keys": args.keys
        }))
    }

    pub async fn take_snapshot(&self, args: TakeSnapshotArgs) -> Result<serde_json::Value> {
        let session = self.sessions.get(&args.session_id)
            .ok_or_else(|| HtMcpError::SessionNotFound(args.session_id.clone()))?;

        // Take snapshot from HT library
        let snapshot = self.ht_lib.take_snapshot(session.internal_id).await
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
        let session = self.sessions.remove(&args.session_id)
            .ok_or_else(|| HtMcpError::SessionNotFound(args.session_id.clone()))?;

        // Close the HT session
        self.ht_lib.close_session(session.internal_id).await
            .map_err(|e| HtMcpError::HtLibrary(format!("Failed to close session: {}", e)))?;

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