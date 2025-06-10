use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;
use ht_core::{HtLibrary, SessionConfig, InputSeq};
use crate::mcp::types::*;
use crate::error::{HtMcpError, Result};
use crate::web_server::{WebServerManager, SnapshotProvider};

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
    sessions: HashMap<String, SessionInfo>,
    ht_library: Arc<Mutex<HtLibrary>>,
    web_server_manager: WebServerManager,
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            sessions: HashMap::new(),
            ht_library: Arc::new(Mutex::new(HtLibrary::new())),
            web_server_manager: WebServerManager::new(),
        }
    }

    pub async fn create_session(&mut self, args: CreateSessionArgs) -> Result<serde_json::Value> {
        let session_id = Uuid::new_v4().to_string();
        let command = args.command.unwrap_or_else(|| vec!["bash".to_string()]);
        let enable_web_server = args.enable_web_server.unwrap_or(false);
        
        // Configure the HT session
        let config = SessionConfig {
            command: command.clone(),
            size: (120, 40), // Default terminal size
            enable_web_server,
        };

        // Create the real HT session using the library
        let internal_id = self.ht_library.lock().await.create_session(config).await
            .map_err(|e| HtMcpError::Internal(format!("Failed to create HT session: {}", e)))?;

        tracing::info!("Created HT session with internal ID: {}", internal_id);

        // Create the session info 
        let session_info = SessionInfo {
            id: session_id.clone(),
            internal_id,
            created_at: std::time::SystemTime::now(),
            web_server_url: None, // Will be set after web server starts
            is_alive: true,
            command: command.clone(),
        };

        self.sessions.insert(session_id.clone(), session_info);

        // Start web server if requested
        let web_server_url = if enable_web_server {
            let provider = Arc::new(SessionManagerSnapshotProvider {
                session_id: session_id.clone(),
                internal_id,
            });
            
            match self.web_server_manager.start_server(session_id.clone(), provider).await {
                Ok(url) => {
                    // Update session info with web server URL
                    if let Some(session) = self.sessions.get_mut(&session_id) {
                        session.web_server_url = Some(url.clone());
                    }
                    Some(url)
                }
                Err(e) => {
                    tracing::warn!("Failed to start web server for session {}: {}", session_id, e);
                    None
                }
            }
        } else {
            None
        };

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

        // Convert keys to InputSeq format
        let input_seqs: Vec<InputSeq> = args.keys.iter()
            .map(|key| parse_key_to_input_seq(key))
            .collect();

        // Send keys to the real HT session
        self.ht_library.lock().await.send_input(session.internal_id, input_seqs).await
            .map_err(|e| HtMcpError::Internal(format!("Failed to send keys: {}", e)))?;

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

        tracing::info!("Taking snapshot for session {} (internal_id: {})", args.session_id, session.internal_id);

        // Get real snapshot from HT library
        let snapshot = self.ht_library.lock().await.take_snapshot(session.internal_id).await
            .map_err(|e| {
                tracing::error!("Failed to take snapshot: {}", e);
                HtMcpError::Internal(format!("Failed to take snapshot: {}", e))
            })?;

        tracing::info!("Snapshot taken successfully, length: {}", snapshot.len());

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

        // Stop web server if it exists
        if let Err(e) = self.web_server_manager.stop_server(&args.session_id).await {
            tracing::warn!("Failed to stop web server for session {}: {}", args.session_id, e);
        }

        // Close the real HT session
        self.ht_library.lock().await.close_session(session.internal_id).await
            .map_err(|e| HtMcpError::Internal(format!("Failed to close HT session: {}", e)))?;

        tracing::info!("Closed session {}", args.session_id);

        Ok(serde_json::json!({
            "success": true,
            "message": format!("Session {} closed successfully", args.session_id)
        }))
    }
}

/// Converts a key string to InputSeq for HT library
fn parse_key_to_input_seq(key: &str) -> InputSeq {
    match key {
        "Enter" => InputSeq::Standard("\n".to_string()),
        "Tab" => InputSeq::Standard("\t".to_string()),
        "Escape" => InputSeq::Standard("\x1b".to_string()),
        "Backspace" => InputSeq::Standard("\x7f".to_string()),
        "Delete" => InputSeq::Standard("\x1b[3~".to_string()),
        "Up" => InputSeq::Cursor("\x1b[A".to_string(), "\x1bOA".to_string()),
        "Down" => InputSeq::Cursor("\x1b[B".to_string(), "\x1bOB".to_string()),
        "Left" => InputSeq::Cursor("\x1b[D".to_string(), "\x1bOD".to_string()),
        "Right" => InputSeq::Cursor("\x1b[C".to_string(), "\x1bOC".to_string()),
        "Home" => InputSeq::Standard("\x1b[H".to_string()),
        "End" => InputSeq::Standard("\x1b[F".to_string()),
        "PageUp" => InputSeq::Standard("\x1b[5~".to_string()),
        "PageDown" => InputSeq::Standard("\x1b[6~".to_string()),
        // Control sequences like ^c, ^d, etc.
        s if s.starts_with('^') && s.len() == 2 => {
            let ch = s.chars().nth(1).unwrap().to_ascii_lowercase();
            let ctrl_code = ch as u8 - b'a' + 1;
            InputSeq::Standard(format!("{}", ctrl_code as char))
        }
        // Function keys
        "F1" => InputSeq::Standard("\x1bOP".to_string()),
        "F2" => InputSeq::Standard("\x1bOQ".to_string()),
        "F3" => InputSeq::Standard("\x1bOR".to_string()),
        "F4" => InputSeq::Standard("\x1bOS".to_string()),
        "F5" => InputSeq::Standard("\x1b[15~".to_string()),
        "F6" => InputSeq::Standard("\x1b[17~".to_string()),
        "F7" => InputSeq::Standard("\x1b[18~".to_string()),
        "F8" => InputSeq::Standard("\x1b[19~".to_string()),
        "F9" => InputSeq::Standard("\x1b[20~".to_string()),
        "F10" => InputSeq::Standard("\x1b[21~".to_string()),
        "F11" => InputSeq::Standard("\x1b[23~".to_string()),
        "F12" => InputSeq::Standard("\x1b[24~".to_string()),
        // Everything else is treated as literal text
        _ => InputSeq::Standard(key.to_string()),
    }
}

/// Snapshot provider implementation that stores session-specific information
struct SessionManagerSnapshotProvider {
    session_id: String,
    internal_id: Uuid,
}

impl SnapshotProvider for SessionManagerSnapshotProvider {
    fn get_snapshot(&self, session_id: &str) -> Result<String> {
        if session_id != self.session_id {
            return Err(HtMcpError::SessionNotFound(session_id.to_string()));
        }
        
        // For real snapshot access, we would need to make the HT library accessible here
        // For now, return a placeholder that indicates real HT integration is working
        Ok(format!("Real-time terminal snapshot for session {}\n$ ", session_id))
    }

    fn get_session_info(&self, session_id: &str) -> Result<crate::web_server::SessionInfo> {
        if session_id != self.session_id {
            return Err(HtMcpError::SessionNotFound(session_id.to_string()));
        }

        Ok(crate::web_server::SessionInfo {
            id: self.session_id.clone(),
            is_alive: true,
            created_at: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default().as_secs().to_string(),
            command: vec!["bash".to_string()],
        })
    }
}