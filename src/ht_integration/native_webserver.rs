use anyhow::{anyhow, Result};
use serde_json::Value;
use std::collections::HashMap;
/// Integration with HT's native webserver functionality
/// This module handles starting HT with its built-in webserver enabled,
/// which provides real-time terminal updates via WebSocket.
use std::process::Stdio;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::{Child, Command};
use tokio::sync::mpsc;
use tracing::{debug, error, info, warn};
use uuid::Uuid;

/// Represents an HT session running with native webserver support
#[derive(Debug)]
pub struct NativeHtSession {
    pub id: String,
    pub internal_id: Uuid,
    pub command: Vec<String>,
    pub web_server_url: Option<String>,
    pub web_server_port: Option<u16>,
    pub is_alive: bool,
    pub created_at: std::time::SystemTime,

    // Process handle for the HT binary
    process: Child,

    // Communication channels
    stdin_tx: mpsc::Sender<String>,
    stdout_rx: mpsc::Receiver<String>,
}

/// Manager for HT sessions that use the native webserver
pub struct NativeHtManager {
    sessions: HashMap<String, NativeHtSession>,
}

impl NativeHtManager {
    pub fn new() -> Self {
        Self {
            sessions: HashMap::new(),
        }
    }

    /// Create a new HT session with optional webserver
    pub async fn create_session(
        &mut self,
        command: Vec<String>,
        enable_web_server: bool,
    ) -> Result<String> {
        let session_id = Uuid::new_v4().to_string();
        let internal_id = Uuid::new_v4();

        // Find available port for webserver if enabled
        let (web_server_port, web_server_url) = if enable_web_server {
            let port = self.find_available_port().await?;
            let url = format!("http://127.0.0.1:{}", port);
            (Some(port), Some(url))
        } else {
            (None, None)
        };

        // Build HT command
        let mut ht_args = vec!["--subscribe".to_string(), "snapshot,output".to_string()];

        if let Some(port) = web_server_port {
            ht_args.extend_from_slice(&["-l".to_string(), format!("127.0.0.1:{}", port)]);
        }

        ht_args.extend(command.clone());

        info!("Starting HT with args: {:?}", ht_args);

        // Start HT process
        // Find the HT binary in the PATH or use a platform-specific approach
        let ht_binary = if cfg!(windows) {
            // On Windows, we might need to handle .exe extension
            "ht.exe"
        } else {
            "ht"
        };
        
        let mut child = Command::new(ht_binary)
            .args(&ht_args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| anyhow!("Failed to start HT process: {}", e))?;

        // Set up communication channels
        let stdin = child
            .stdin
            .take()
            .ok_or_else(|| anyhow!("Failed to get stdin"))?;
        let stdout = child
            .stdout
            .take()
            .ok_or_else(|| anyhow!("Failed to get stdout"))?;

        let (stdin_tx, mut stdin_rx) = mpsc::channel::<String>(1024);
        let (stdout_tx, stdout_rx) = mpsc::channel::<String>(1024);

        // Handle stdin forwarding
        let mut stdin_writer = stdin;
        tokio::spawn(async move {
            while let Some(input) = stdin_rx.recv().await {
                if let Err(e) = stdin_writer.write_all(input.as_bytes()).await {
                    error!("Failed to write to HT stdin: {}", e);
                    break;
                }
                if let Err(e) = stdin_writer.flush().await {
                    error!("Failed to flush HT stdin: {}", e);
                    break;
                }
            }
        });

        // Handle stdout reading
        let stdout_reader = BufReader::new(stdout);
        tokio::spawn(async move {
            let mut lines = stdout_reader.lines();
            while let Ok(Some(line)) = lines.next_line().await {
                if stdout_tx.send(line).await.is_err() {
                    break;
                }
            }
        });

        // Wait a moment for HT to start up
        tokio::time::sleep(tokio::time::Duration::from_millis(if enable_web_server {
            2000
        } else {
            800
        }))
        .await;

        // Create session info
        let session = NativeHtSession {
            id: session_id.clone(),
            internal_id,
            command,
            web_server_url,
            web_server_port,
            is_alive: true,
            created_at: std::time::SystemTime::now(),
            process: child,
            stdin_tx,
            stdout_rx,
        };

        self.sessions.insert(session_id.clone(), session);

        info!(
            "Created HT session {} with webserver: {}",
            session_id, enable_web_server
        );

        Ok(session_id)
    }

    /// Send keys to an HT session
    pub async fn send_keys(&mut self, session_id: &str, keys: Vec<String>) -> Result<()> {
        let session = self
            .sessions
            .get_mut(session_id)
            .ok_or_else(|| anyhow!("Session not found: {}", session_id))?;

        if !session.is_alive {
            return Err(anyhow!("Session {} is not alive", session_id));
        }

        let command = serde_json::json!({
            "type": "sendKeys",
            "keys": keys
        });

        let command_str = format!("{}\n", command.to_string());
        session
            .stdin_tx
            .send(command_str)
            .await
            .map_err(|e| anyhow!("Failed to send keys: {}", e))?;

        debug!("Sent keys {:?} to session {}", keys, session_id);
        Ok(())
    }

    /// Take a snapshot of an HT session
    pub async fn take_snapshot(&mut self, session_id: &str) -> Result<String> {
        let session = self
            .sessions
            .get_mut(session_id)
            .ok_or_else(|| anyhow!("Session not found: {}", session_id))?;

        if !session.is_alive {
            return Err(anyhow!("Session {} is not alive", session_id));
        }

        let command = serde_json::json!({
            "type": "takeSnapshot"
        });

        let command_str = format!("{}\n", command.to_string());
        session
            .stdin_tx
            .send(command_str)
            .await
            .map_err(|e| anyhow!("Failed to send snapshot command: {}", e))?;

        // Wait for snapshot response
        let timeout = tokio::time::Duration::from_secs(8);
        let start_time = std::time::Instant::now();

        while start_time.elapsed() < timeout {
            if let Ok(line) = tokio::time::timeout(
                tokio::time::Duration::from_millis(100),
                session.stdout_rx.recv(),
            )
            .await
            {
                if let Some(line) = line {
                    if let Ok(response) = serde_json::from_str::<Value>(&line) {
                        if let Some(event_type) = response.get("type").and_then(|t| t.as_str()) {
                            if event_type == "snapshot" {
                                if let Some(data) = response.get("data") {
                                    if let Some(text) = data.get("text").and_then(|t| t.as_str()) {
                                        debug!(
                                            "Received snapshot for session {}, length: {}",
                                            session_id,
                                            text.len()
                                        );
                                        return Ok(text.to_string());
                                    } else if let Some(seq) =
                                        data.get("seq").and_then(|s| s.as_str())
                                    {
                                        debug!(
                                            "Received snapshot seq for session {}, length: {}",
                                            session_id,
                                            seq.len()
                                        );
                                        return Ok(seq.to_string());
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        Err(anyhow!("Snapshot timeout for session {}", session_id))
    }

    /// Execute a command in an HT session
    pub async fn execute_command(&mut self, session_id: &str, command: &str) -> Result<String> {
        // Send the command
        self.send_keys(session_id, vec![command.to_string()])
            .await?;

        // Send Enter
        self.send_keys(session_id, vec!["Enter".to_string()])
            .await?;

        // Wait for command to execute
        tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;

        // Take snapshot to see result
        self.take_snapshot(session_id).await
    }

    /// Get session information
    pub fn get_session(&self, session_id: &str) -> Option<&NativeHtSession> {
        self.sessions.get(session_id)
    }

    /// List all sessions
    pub fn list_sessions(&self) -> Vec<&NativeHtSession> {
        self.sessions.values().collect()
    }

    /// Close a session
    pub async fn close_session(&mut self, session_id: &str) -> Result<()> {
        if let Some(mut session) = self.sessions.remove(session_id) {
            // Mark as not alive
            session.is_alive = false;

            // Kill the process
            if let Err(e) = session.process.kill().await {
                warn!(
                    "Failed to kill HT process for session {}: {}",
                    session_id, e
                );
            }

            info!("Closed session {}", session_id);
            Ok(())
        } else {
            Err(anyhow!("Session not found: {}", session_id))
        }
    }

    /// Find an available port for the webserver
    async fn find_available_port(&self) -> Result<u16> {
        use tokio::net::TcpListener;

        // Increase the port range for Windows which may have more system services
        let (start_port, end_port) = if cfg!(windows) {
            (5000, 6000) // Higher range to avoid conflicts with Windows system services
        } else {
            (3000, 4000)
        };

        for port in start_port..end_port {
            if let Ok(listener) = TcpListener::bind(format!("127.0.0.1:{}", port)).await {
                drop(listener);
                return Ok(port);
            }
        }

        Err(anyhow!("No available ports found"))
    }
}

impl Drop for NativeHtSession {
    fn drop(&mut self) {
        // Ensure process is killed when session is dropped
        if let Ok(child) = self.process.try_wait() {
            if child.is_none() {
                // Windows may have different process termination behavior
                if cfg!(windows) {
                    // For Windows, ensure process and any child processes are terminated
                    let _ = self.process.start_kill();
                    
                    // On Windows, we might need additional cleanup for child processes
                    // This would be a future enhancement if needed
                } else {
                    let _ = self.process.start_kill();
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[cfg(not(ci))]
    use super::*;

    #[tokio::test]
    #[cfg(not(ci))] // Skip in CI as it requires actual terminal processes
    async fn test_native_ht_session() {
        let mut manager = NativeHtManager::new();

        // Choose shell based on platform
        let shell = if cfg!(windows) {
            "powershell.exe"
        } else {
            "bash"
        };

        // Create session without webserver first (faster)
        let session_id = manager
            .create_session(vec![shell.to_string()], false)
            .await
            .unwrap();

        // Test basic commands
        let snapshot = manager.take_snapshot(&session_id).await.unwrap();
        assert!(!snapshot.is_empty());

        manager
            .send_keys(
                &session_id,
                vec!["echo".to_string(), " ".to_string(), "hello".to_string()],
            )
            .await
            .unwrap();
        manager
            .send_keys(&session_id, vec!["Enter".to_string()])
            .await
            .unwrap();

        // Wait and take another snapshot
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        let snapshot2 = manager.take_snapshot(&session_id).await.unwrap();
        assert!(snapshot2.contains("hello"));

        // Clean up
        manager.close_session(&session_id).await.unwrap();
    }
}
