use crate::error::{HtMcpError, Result};
use crate::ht_integration::native_webserver::NativeHtManager;
use crate::mcp::types::*;
use chrono::{DateTime, Utc};
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::info;

/// Session manager that uses native HT subprocess with webserver support
pub struct NativeSessionManager {
    ht_manager: Arc<Mutex<NativeHtManager>>,
}

impl NativeSessionManager {
    pub fn new() -> Self {
        Self {
            ht_manager: Arc::new(Mutex::new(NativeHtManager::new())),
        }
    }

    pub async fn create_session(&self, args: CreateSessionArgs) -> Result<serde_json::Value> {
        let command = args.command.unwrap_or_else(|| vec!["bash".to_string()]);
        let enable_web_server = args.enable_web_server.unwrap_or(false);

        let mut manager = self.ht_manager.lock().await;
        let session_id = manager
            .create_session(command.clone(), enable_web_server)
            .await
            .map_err(|e| HtMcpError::Internal(format!("Failed to create HT session: {}", e)))?;

        // Get session info to include web server URL
        let session = manager
            .get_session(&session_id)
            .ok_or_else(|| HtMcpError::Internal("Session created but not found".to_string()))?;

        let mut response_message = format!(
            "HT session created successfully!\n\nSession ID: {}\n\nYou can now use this session ID with other HT tools to send commands and take snapshots.",
            session_id
        );

        if let Some(ref url) = session.web_server_url {
            response_message.push_str(&format!(
                "\n\n🌐 Web server enabled! View live terminal at: {}",
                url
            ));
        }

        let result = CreateSessionResult {
            session_id,
            message: response_message,
            web_server_enabled: enable_web_server,
            web_server_url: session.web_server_url.clone(),
        };

        info!("Created session with native HT webserver: {:?}", result);
        Ok(serde_json::to_value(result)?)
    }

    pub async fn send_keys(&self, args: SendKeysArgs) -> Result<serde_json::Value> {
        let mut manager = self.ht_manager.lock().await;
        manager
            .send_keys(&args.session_id, args.keys.clone())
            .await
            .map_err(|e| HtMcpError::Internal(format!("Failed to send keys: {}", e)))?;

        let response = format!(
            "Keys sent successfully to session {}\n\n```\nSent keys: {:?}\n```",
            args.session_id, args.keys
        );

        Ok(serde_json::json!({
            "success": true,
            "message": response,
            "sessionId": args.session_id,
            "keys": args.keys
        }))
    }

    pub async fn take_snapshot(&self, args: TakeSnapshotArgs) -> Result<serde_json::Value> {
        let mut manager = self.ht_manager.lock().await;
        let snapshot = manager
            .take_snapshot(&args.session_id)
            .await
            .map_err(|e| HtMcpError::Internal(format!("Failed to take snapshot: {}", e)))?;

        Ok(serde_json::json!({
            "sessionId": args.session_id,
            "snapshot": snapshot
        }))
    }

    pub async fn execute_command(&self, args: ExecuteCommandArgs) -> Result<serde_json::Value> {
        let mut manager = self.ht_manager.lock().await;
        let output = manager
            .execute_command(&args.session_id, &args.command)
            .await
            .map_err(|e| HtMcpError::Internal(format!("Failed to execute command: {}", e)))?;

        Ok(serde_json::json!({
            "command": args.command,
            "sessionId": args.session_id,
            "output": output
        }))
    }

    pub async fn list_sessions(&self) -> Result<serde_json::Value> {
        let manager = self.ht_manager.lock().await;
        let sessions = manager.list_sessions();

        if sessions.is_empty() {
            return Ok(serde_json::json!({
                "sessions": [],
                "count": 0,
                "message": "No active HT sessions found."
            }));
        }

        let session_list: Vec<serde_json::Value> = sessions
            .iter()
            .map(|session| {
                let created_at = DateTime::<Utc>::from(session.created_at);
                serde_json::json!({
                    "id": session.id,
                    "isAlive": session.is_alive,
                    "createdAt": created_at.format("%Y-%m-%d %H:%M:%S UTC").to_string(),
                    "command": session.command,
                    "webServerUrl": session.web_server_url
                })
            })
            .collect();

        let mut response = format!("Active HT sessions ({})\n", sessions.len());
        for session in &sessions {
            let created_at = DateTime::<Utc>::from(session.created_at);
            response.push_str(&format!(
                "\n• Session {}\n  Command: {:?}\n  Created: {}\n  Status: {}\n",
                session.id,
                session.command,
                created_at.format("%Y-%m-%d %H:%M:%S UTC"),
                if session.is_alive {
                    "🟢 Alive"
                } else {
                    "🔴 Dead"
                }
            ));
            if let Some(ref url) = session.web_server_url {
                response.push_str(&format!("  Web server: {}\n", url));
            }
        }

        Ok(serde_json::json!({
            "sessions": session_list,
            "count": sessions.len(),
            "message": response
        }))
    }

    pub async fn close_session(&self, args: CloseSessionArgs) -> Result<serde_json::Value> {
        let mut manager = self.ht_manager.lock().await;

        // Verify session exists first
        if manager.get_session(&args.session_id).is_none() {
            return Err(HtMcpError::SessionNotFound(args.session_id.clone()));
        }

        manager
            .close_session(&args.session_id)
            .await
            .map_err(|e| HtMcpError::Internal(format!("Failed to close session: {}", e)))?;

        let response = format!("HT session {} closed successfully", args.session_id);

        Ok(serde_json::json!({
            "success": true,
            "message": response,
            "sessionId": args.session_id
        }))
    }
}

impl Default for NativeSessionManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    #[cfg(not(ci))]
    use super::*;

    #[tokio::test]
    #[cfg(not(ci))] // Skip in CI as it requires actual terminal processes
    async fn test_native_session_manager() {
        let manager = NativeSessionManager::new();

        // Create a session
        let create_result = manager
            .create_session(CreateSessionArgs {
                command: Some(vec!["bash".to_string()]),
                enable_web_server: Some(false),
            })
            .await
            .unwrap();

        let session_id = create_result["sessionId"].as_str().unwrap().to_string();

        // Test snapshot
        let snapshot_result = manager
            .take_snapshot(TakeSnapshotArgs {
                session_id: session_id.clone(),
            })
            .await
            .unwrap();

        assert!(snapshot_result["snapshot"]
            .as_str()
            .unwrap()
            .contains("session"));

        // Test command execution
        let exec_result = manager
            .execute_command(ExecuteCommandArgs {
                session_id: session_id.clone(),
                command: "echo test".to_string(),
            })
            .await
            .unwrap();

        assert!(exec_result["output"].as_str().unwrap().contains("test"));

        // List sessions
        let list_result = manager.list_sessions().await.unwrap();
        assert_eq!(list_result["count"].as_u64().unwrap(), 1);

        // Close session
        manager
            .close_session(CloseSessionArgs { session_id })
            .await
            .unwrap();
    }
}
