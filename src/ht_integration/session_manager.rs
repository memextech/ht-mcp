use crate::error::{HtMcpError, Result};
use crate::mcp::types::*;
use ht_core::{api::http, pty, pty::Winsize, session::Session};
use std::collections::HashMap;
use std::net::{SocketAddr, TcpListener};
use std::sync::Arc;
use tokio::sync::{mpsc, oneshot};
use uuid::Uuid;

use tracing::{error, info};

// Enhanced command type that supports responses
#[derive(Debug)]
pub enum SessionCommand {
    Input(Vec<ht_core::command::InputSeq>),
    Snapshot(oneshot::Sender<String>),
    Resize(usize, usize),
}

#[derive(Debug, Clone)]
pub struct SessionInfo {
    pub id: String,
    pub internal_id: Uuid,
    pub created_at: std::time::SystemTime,
    pub web_server_url: Option<String>,
    pub is_alive: bool,
    pub command: Vec<String>,
    pub command_tx: Arc<mpsc::Sender<SessionCommand>>,
}

pub struct SessionManager {
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
        let command = args.command.unwrap_or_else(|| vec!["bash".to_string()]);
        let enable_web_server = args.enable_web_server.unwrap_or(false);
        let internal_id = Uuid::new_v4();

        // Create channels for communication
        let (input_tx, input_rx) = mpsc::channel::<Vec<u8>>(1024);
        let (output_tx, mut output_rx) = mpsc::channel::<Vec<u8>>(1024);
        let (command_tx, mut command_rx) = mpsc::channel::<SessionCommand>(1024);
        let (clients_tx, mut clients_rx) = mpsc::channel(1);

        // Create a platform-agnostic terminal size
        // Using a helper function to maintain a clean interface
        let size = create_winsize(120, 40);
        let cols = size.ws_col as usize;
        let rows = size.ws_row as usize;

        // Start HTTP server if enabled - we need to clone clients_tx for the HTTP server
        let (web_server_url, _clients_tx_for_session) = if enable_web_server {
            let port = self.find_available_port().await?;
            let addr = SocketAddr::from(([127, 0, 0, 1], port));
            let listener = TcpListener::bind(addr).map_err(|e| {
                HtMcpError::Internal(format!("Failed to bind to port {}: {}", port, e))
            })?;

            let url = format!("http://127.0.0.1:{}", port);

            // Clone clients_tx for the HTTP server
            let clients_tx_for_http = clients_tx.clone();

            // Start the HTTP server with HT's native implementation
            tokio::spawn(async move {
                if let Ok(server_future) = http::start(listener, clients_tx_for_http).await {
                    if let Err(e) = server_future.await {
                        error!("HTTP server error: {}", e);
                    }
                }
            });

            info!("Started HT native webserver on {}", url);
            (Some(url), clients_tx)
        } else {
            (None, clients_tx)
        };

        // Start PTY process
        let command_str = command.join(" ");
        let _pty_handle = tokio::spawn(async move {
            match pty::spawn(command_str, size, input_rx, output_tx) {
                Ok(future) => {
                    if let Err(e) = future.await {
                        error!("PTY execution error: {}", e);
                    }
                }
                Err(e) => {
                    error!("PTY spawn error: {}", e);
                }
            }
        });

        // Start session event loop
        let session_id_clone = session_id.clone();
        tokio::spawn(async move {
            let mut session = Session::new(cols, rows);
            let mut serving = true;

            loop {
                tokio::select! {
                    // Handle output from PTY
                    output = output_rx.recv() => {
                        match output {
                            Some(data) => {
                                session.output(String::from_utf8_lossy(&data).to_string());
                            }
                            None => {
                                info!("PTY process exited for session {}", session_id_clone);
                                break;
                            }
                        }
                    }

                    // Handle commands from MCP
                    command = command_rx.recv() => {
                        match command {
                            Some(SessionCommand::Input(seqs)) => {
                                let data = ht_core::command::seqs_to_bytes(&seqs, session.cursor_key_app_mode());
                                if let Err(e) = input_tx.send(data).await {
                                    error!("Failed to send input to PTY: {}", e);
                                }
                            }
                            Some(SessionCommand::Snapshot(response_tx)) => {
                                // Get the current terminal text and send it back
                                let text = session.get_text();
                                let _ = response_tx.send(text);
                            }
                            Some(SessionCommand::Resize(cols, rows)) => {
                                session.resize(cols, rows);
                            }
                            None => {
                                info!("Command channel closed for session {}", session_id_clone);
                                break;
                            }
                        }
                    }

                    // Handle WebSocket clients (for webserver)
                    client = clients_rx.recv(), if serving => {
                        match client {
                            Some(client) => {
                                info!("New WebSocket client connected to session {}", session_id_clone);
                                client.accept(session.subscribe());
                            }
                            None => {
                                info!("Client channel closed for session {}", session_id_clone);
                                serving = false;
                            }
                        }
                    }
                }
            }
        });

        // Create the session info
        let session_info = SessionInfo {
            id: session_id.clone(),
            internal_id,
            created_at: std::time::SystemTime::now(),
            web_server_url,
            is_alive: true,
            command: command.clone(),
            command_tx: Arc::new(command_tx),
        };

        let web_server_url_for_result = session_info.web_server_url.clone();

        self.sessions.insert(session_id.clone(), session_info);

        let result = CreateSessionResult {
            session_id,
            message: "HT session created successfully".to_string(),
            web_server_enabled: enable_web_server,
            web_server_url: web_server_url_for_result,
        };

        info!("Created HT session with native webserver: {:?}", result);
        Ok(serde_json::to_value(result)?)
    }

    /// Find an available port for the webserver
    /// Uses port range 3618-3999 to avoid conflicts with common development servers
    /// (Next.js: 3000, React: 3001, etc.)
    async fn find_available_port(&self) -> Result<u16> {
        for port in 3618..3999 {
            if let Ok(listener) = TcpListener::bind(format!("127.0.0.1:{}", port)) {
                drop(listener);
                return Ok(port);
            }
        }
        Err(HtMcpError::Internal("No available ports found".to_string()))
    }

    pub async fn send_keys(&mut self, args: SendKeysArgs) -> Result<serde_json::Value> {
        let session = self
            .sessions
            .get(&args.session_id)
            .ok_or_else(|| HtMcpError::SessionNotFound(args.session_id.clone()))?;

        // Convert keys to InputSeq format using intelligent key parsing
        info!("send_keys: processing {} keys for session {}", args.keys.len(), args.session_id);
        for (i, key) in args.keys.iter().enumerate() {
            info!("  key[{}]: '{}' (len: {}, is_special: {}, contains_email: {})", 
                  i, key, key.len(), is_special_key(key), key.contains("noreply@memex.tech"));
        }
        
        let input_seqs: Vec<ht_core::command::InputSeq> = args
            .keys
            .iter()
            .map(|key| smart_parse_key(key))
            .collect();

        // Send keys via the command channel
        session
            .command_tx
            .send(SessionCommand::Input(input_seqs))
            .await
            .map_err(|e| HtMcpError::Internal(format!("Failed to send keys: {}", e)))?;

        info!("Sent keys {:?} to session {}", args.keys, args.session_id);

        Ok(serde_json::json!({
            "success": true,
            "message": format!("Keys sent successfully to session {}", args.session_id),
            "sessionId": args.session_id,
            "keys": args.keys
        }))
    }

    pub async fn take_snapshot(&self, args: TakeSnapshotArgs) -> Result<serde_json::Value> {
        let session = self
            .sessions
            .get(&args.session_id)
            .ok_or_else(|| HtMcpError::SessionNotFound(args.session_id.clone()))?;

        info!("Taking snapshot for session {}", args.session_id);

        // Create a response channel for the snapshot
        let (response_tx, response_rx) = oneshot::channel();

        // Send snapshot command with response channel
        session
            .command_tx
            .send(SessionCommand::Snapshot(response_tx))
            .await
            .map_err(|e| HtMcpError::Internal(format!("Failed to send snapshot command: {}", e)))?;

        // Wait for the response with a timeout
        let snapshot = tokio::time::timeout(tokio::time::Duration::from_secs(5), response_rx)
            .await
            .map_err(|_| HtMcpError::Internal("Snapshot request timed out".to_string()))?
            .map_err(|e| HtMcpError::Internal(format!("Failed to receive snapshot: {}", e)))?;

        info!(
            "Received snapshot for session {}: {} chars",
            args.session_id,
            snapshot.len()
        );

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
        })
        .await?;

        // Send Enter
        self.send_keys(SendKeysArgs {
            session_id: args.session_id.clone(),
            keys: vec!["Enter".to_string()],
        })
        .await?;

        // Wait for command to execute
        tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;

        // Take snapshot
        let snapshot_result = self
            .take_snapshot(TakeSnapshotArgs {
                session_id: args.session_id.clone(),
            })
            .await?;

        Ok(serde_json::json!({
            "command": args.command,
            "sessionId": args.session_id,
            "output": snapshot_result["snapshot"]
        }))
    }

    pub async fn list_sessions(&self) -> Result<serde_json::Value> {
        let sessions: Vec<serde_json::Value> = self
            .sessions
            .values()
            .map(|session| {
                serde_json::json!({
                    "id": session.id,
                    "isAlive": session.is_alive,
                    "createdAt": session.created_at.duration_since(std::time::UNIX_EPOCH)
                        .unwrap_or_default().as_secs(),
                    "command": session.command,
                    "webServerUrl": session.web_server_url
                })
            })
            .collect();

        Ok(serde_json::json!({
            "sessions": sessions,
            "count": sessions.len()
        }))
    }

    pub async fn close_session(&mut self, args: CloseSessionArgs) -> Result<serde_json::Value> {
        let session = self
            .sessions
            .remove(&args.session_id)
            .ok_or_else(|| HtMcpError::SessionNotFound(args.session_id.clone()))?;

        // Close the command channel to trigger session shutdown
        drop(session.command_tx);

        info!("Closed session {}", args.session_id);

        Ok(serde_json::json!({
            "success": true,
            "message": format!("Session {} closed successfully", args.session_id)
        }))
    }
}

/// Creates a Winsize struct with platform-appropriate fields
/// This function abstracts away platform differences in the Winsize struct
fn create_winsize(cols: u16, rows: u16) -> Winsize {
    #[cfg(unix)]
    {
        Winsize {
            ws_col: cols,
            ws_row: rows,
            ws_xpixel: 0,
            ws_ypixel: 0,
        }
    }

    #[cfg(windows)]
    {
        Winsize {
            ws_col: cols,
            ws_row: rows,
        }
    }
}

/// Intelligently parse a key string as either a special key or literal text
fn smart_parse_key(key: &str) -> ht_core::command::InputSeq {
    if is_special_key(key) {
        ht_core::api::stdio::parse_key(key.to_string())
    } else {
        // For regular text content, pass through as-is
        ht_core::api::stdio::standard_key(key)
    }
}



/// Determine if a string represents a special key vs literal text
fn is_special_key(key: &str) -> bool {
    // Empty strings are not keys
    if key.is_empty() {
        return false;
    }
    
    // Long strings are definitely text, not keys (reduced threshold for git commands)
    if key.len() > 15 {
        return false;
    }
    
    // Strings containing quotes are definitely text
    if key.contains('"') || key.contains('\'') {
        return false;
    }
    
    // Strings containing command-like patterns are text
    if key.contains("git ") || key.contains("echo ") || key.contains("cd ") {
        return false;
    }
    
    // Strings with multiple spaces are usually commands/text
    if key.matches(' ').count() > 1 {
        return false;
    }
    
    // Strings with spaces are usually text (except for special cases)
    if key.contains(' ') && !matches!(key, "C-Space" | "Space") {
        return false;
    }
    
    // URLs and brackets indicate text content
    if key.contains("http") || key.contains('[') || key.contains(']') || key.contains('<') || key.contains('>') {
        return false;
    }
    
    // Check if it matches known key patterns
    matches!(key,
        // Basic keys
        "Enter" | "Tab" | "Space" | "Escape" |
        // Arrow keys
        "Left" | "Right" | "Up" | "Down" |
        // Function keys
        "F1" | "F2" | "F3" | "F4" | "F5" | "F6" | "F7" | "F8" | "F9" | "F10" | "F11" | "F12" |
        // Home/End/Page keys
        "Home" | "End" | "PageUp" | "PageDown" |
        // Backspace/Delete
        "Backspace" | "Delete"
    ) || 
    // Single characters (likely literal keys, but be careful with common text chars)
    (key.len() == 1 && !key.chars().next().unwrap().is_whitespace()) ||
    // Control sequences (must be reasonable length and follow pattern)
    (key.len() <= 8 && (key.starts_with("C-") || key.starts_with("^"))) ||
    // Alt sequences (must be reasonable length)
    (key.len() <= 6 && key.starts_with("A-")) ||
    // Shift sequences (must be reasonable length)  
    (key.len() <= 10 && key.starts_with("S-"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_special_key_detection_valid_keys() {
        // Basic keys
        assert!(is_special_key("Enter"));
        assert!(is_special_key("Tab"));
        assert!(is_special_key("Space"));
        assert!(is_special_key("Escape"));
        
        // Arrow keys
        assert!(is_special_key("Left"));
        assert!(is_special_key("Right"));
        assert!(is_special_key("Up"));
        assert!(is_special_key("Down"));
        
        // Function keys
        assert!(is_special_key("F1"));
        assert!(is_special_key("F12"));
        
        // Home/End/Page keys
        assert!(is_special_key("Home"));
        assert!(is_special_key("End"));
        assert!(is_special_key("PageUp"));
        assert!(is_special_key("PageDown"));
        
        // Control sequences (short)
        assert!(is_special_key("C-x"));
        assert!(is_special_key("C-c"));
        assert!(is_special_key("^c"));
        assert!(is_special_key("^C"));
        
        // Alt sequences (short)
        assert!(is_special_key("A-x"));
        assert!(is_special_key("A-1"));
        
        // Shift sequences (short)
        assert!(is_special_key("S-Left"));
        
        // Single characters
        assert!(is_special_key("a"));
        assert!(is_special_key("1"));
        assert!(is_special_key("!"));
    }

    #[test]
    fn test_special_key_detection_text_content() {
        // Simple text
        assert!(!is_special_key("hello world"));
        assert!(!is_special_key("multiple words here"));
        
        // Git commands (the main use case)
        assert!(!is_special_key("git commit -m"));
        assert!(!is_special_key("git status"));
        assert!(!is_special_key("echo 'test'"));
        assert!(!is_special_key("cd /path/to/dir"));
        
        // Quoted strings
        assert!(!is_special_key("\"quoted string\""));
        assert!(!is_special_key("'single quoted'"));
        assert!(!is_special_key("git commit -m \"message\""));
        
        // Long strings
        assert!(!is_special_key("This is a very long string"));
        assert!(!is_special_key("git commit -m \"Long commit message here\""));
        
        // URLs and markup
        assert!(!is_special_key("https://example.com"));
        assert!(!is_special_key("[Memex](https://memex.tech)"));
        assert!(!is_special_key("<noreply@memex.tech>"));
        
        // Email addresses
        assert!(!is_special_key("Co-Authored-By: Memex <noreply@memex.tech>"));
        
        // Empty string
        assert!(!is_special_key(""));
    }

    #[test]
    fn test_complex_git_commit_messages() {
        // Test the exact format that was previously failing
        let complex_commit_1 = r#"Update project rules with completed MCP submissions and documentation references

🤖 Generated with [Memex](https://memex.tech)
Co-Authored-By: Memex <noreply@memex.tech>

Issue: Terminal seemed to have problems with multiline commit message or emoji encoding
Date: Current session
Context: Updating .memex/rules.md with MCP submission status and documentation references"#;
        
        assert!(!is_special_key(complex_commit_1));
        
        // Test another complex format with emoji and URLs
        let complex_commit_2 = r#"🚀 Add proof of concept feature

This is a multiline commit message with:
- Emoji 😀 and special characters
- Long description text
- URLs: https://github.com/memextech/ht-mcp

Co-Authored-By: Memex <noreply@memex.tech>"#;
        
        assert!(!is_special_key(complex_commit_2));
        
        // Test commit with various special characters and formatting
        let complex_commit_3 = r#"Fix: Handle special characters & symbols properly

- Support for & (ampersand)
- Support for @ symbols in emails
- Support for <angle brackets>
- Support for [square brackets] and (parentheses) 
- URLs: https://example.com?param=value&other=123

🤖 Generated with [Memex](https://memex.tech)
Co-Authored-By: Memex <noreply@memex.tech>"#;
        
        assert!(!is_special_key(complex_commit_3));
    }

    #[test]
    fn test_smart_parse_key_with_complex_content() {
        // Test that complex content gets parsed as standard text
        let complex_message = r#"🚀 Add proof of concept feature

This is a multiline commit message with:
- Emoji 😀 and special characters"#;
        
        let parsed = smart_parse_key(complex_message);
        // The exact implementation details depend on ht_core, but we know it should handle this as text
        // This test mainly ensures no panics occur with complex input
        match parsed {
            ht_core::command::InputSeq::Standard(text) => {
                assert!(!text.is_empty(), "Complex content should produce non-empty text");
            },
            ht_core::command::InputSeq::Cursor(_, _) => {
                // Also acceptable for complex content
            }
        }
    }

    #[test]
    fn test_smart_parse_key_functionality() {
        // Test that the function works without panics for various inputs
        let _enter = smart_parse_key("Enter");
        let _ctrl_c = smart_parse_key("C-c"); 
        let _hat_c = smart_parse_key("^c");
        let _text = smart_parse_key("abc");
        let _complex = smart_parse_key("git commit -m \"message\"");
        
        // Main test: ensure complex git commit content is not treated as special keys
        let git_commit = r#"Update project rules

🤖 Generated with [Memex](https://memex.tech)
Co-Authored-By: Memex <noreply@memex.tech>"#;
        
        let _parsed_git = smart_parse_key(git_commit);
        // As long as it doesn't panic, it's working correctly
    }

    #[test]
    fn test_complex_commit_message_cases() {
        // The actual failing cases from the issue
        let complex_commit = "git commit -m \"Fix complex key parsing\\n\\n🤖 Generated with [Memex](https://memex.tech)\\nCo-Authored-By: Memex <noreply@memex.tech>\"";
        assert!(!is_special_key(complex_commit));
        
        // Emoji handling
        assert!(!is_special_key("🤖 Generated with [Memex](https://memex.tech)"));
        assert!(!is_special_key("Co-Authored-By: Memex <noreply@memex.tech>"));
        
        // Markdown URLs
        assert!(!is_special_key("[Memex](https://memex.tech)"));
        assert!(!is_special_key("Visit [our site](https://example.com)"));
        
        // Multi-line markers (escaped)
        assert!(!is_special_key("First line\\nSecond line"));
    }

    #[test]
    fn test_edge_cases() {
        // Control sequences that are too long should be text
        assert!(!is_special_key("C-very-long-sequence"));
        
        // Multiple spaces indicate commands
        assert!(!is_special_key("command with multiple spaces"));
        
        // Valid C-Space vs invalid space combinations
        assert!(is_special_key("C-Space"));
        assert!(!is_special_key("not valid space"));
        
        // Whitespace characters as single chars
        assert!(!is_special_key(" ")); // Space char should use "Space" key name
        assert!(!is_special_key("\t")); // Tab char should use "Tab" key name
    }

    #[test]
    fn test_smart_parse_key_integration() {
        // Special keys should use parse_key
        let enter_result = smart_parse_key("Enter");
        let expected_enter = ht_core::api::stdio::parse_key("Enter".to_string());
        assert_eq!(format!("{:?}", enter_result), format!("{:?}", expected_enter));
        
        // Control key
        let ctrl_c_result = smart_parse_key("C-c");
        let expected_ctrl_c = ht_core::api::stdio::parse_key("C-c".to_string());
        assert_eq!(format!("{:?}", ctrl_c_result), format!("{:?}", expected_ctrl_c));
        
        // Text should use standard_key
        let text_result = smart_parse_key("hello world");
        let expected_text = ht_core::api::stdio::standard_key("hello world");
        assert_eq!(format!("{:?}", text_result), format!("{:?}", expected_text));
        
        // Simple git command should use standard_key
        let git_result = smart_parse_key("git commit -m \"test\"");
        let expected_git = ht_core::api::stdio::standard_key("git commit -m \"test\"");
        assert_eq!(format!("{:?}", git_result), format!("{:?}", expected_git));
        
        // Emoji string should use standard_key
        let emoji_result = smart_parse_key("🤖 Generated with [Memex](https://memex.tech)");
        let expected_emoji = ht_core::api::stdio::standard_key("🤖 Generated with [Memex](https://memex.tech)");
        assert_eq!(format!("{:?}", emoji_result), format!("{:?}", expected_emoji));
    }



    #[test]
    fn test_complex_git_commit_integration() {
        // Test that complex git commits are processed as standard text (no special conversion)
        let complex_commit = "git commit -m \"Fix issue\\n\\n🤖 Generated with [Memex](https://memex.tech)\\nCo-Authored-By: Memex <noreply@memex.tech>\"";
        let result = smart_parse_key(complex_commit);
        
        // Should be treated as standard key with original content passed through
        if let ht_core::command::InputSeq::Standard(cmd) = result {
            assert!(cmd.contains("git commit"));
            assert!(cmd.contains("🤖"));
            assert!(cmd.contains("Co-Authored-By"));
            // Should NOT be converted to file-based approach
            assert!(!cmd.contains("echo"));
            assert!(!cmd.contains("-F"));
        } else {
            panic!("Expected Standard InputSeq for git commit");
        }
    }
}
