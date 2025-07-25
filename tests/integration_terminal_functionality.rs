#[cfg(not(ci))]
use serde_json::{json, Value};
#[cfg(not(ci))]
use std::io::{BufRead, BufReader};
#[cfg(not(ci))]
use std::process::{Command, Stdio};

#[cfg(not(ci))]
mod integration_tests {
    use super::*;
    use std::io::Write;
    use std::time::Duration;

    /// Helper struct for MCP testing
    struct McpClient {
        child: std::process::Child,
        stdin: std::process::ChildStdin,
        reader: BufReader<std::process::ChildStdout>,
        message_id: u64,
    }

    impl McpClient {
        fn new() -> Self {
            eprintln!("DEBUG: Building ht-mcp server");
            let build_output = Command::new("cargo")
                .args(["build"])
                .output()
                .expect("Failed to build ht-mcp");

            if !build_output.status.success() {
                panic!(
                    "Failed to build ht-mcp: {}",
                    String::from_utf8_lossy(&build_output.stderr)
                );
            }

            eprintln!("DEBUG: Starting ht-mcp server from binary");
            let mut child = Command::new("./target/debug/ht-mcp")
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .expect("Failed to start ht-mcp server");

            let stdin = child.stdin.take().expect("Failed to get stdin");
            let stdout = child.stdout.take().expect("Failed to get stdout");
            let reader = BufReader::new(stdout);

            let mut client = Self {
                child,
                stdin,
                reader,
                message_id: 0,
            };

            // Give the server a moment to start
            std::thread::sleep(Duration::from_millis(100));

            // Check if child is still running
            if let Some(exit_status) = client
                .child
                .try_wait()
                .expect("Failed to check child status")
            {
                panic!(
                    "Server terminated during startup with exit code: {:?}",
                    exit_status
                );
            }

            // Initialize the server
            client.initialize();
            client
        }

        fn initialize(&mut self) {
            eprintln!("DEBUG: Starting initialization");

            // Send initialize
            let init_msg = json!({
                "jsonrpc": "2.0",
                "id": self.next_id(),
                "method": "initialize",
                "params": {
                    "protocolVersion": "2024-11-05",
                    "capabilities": {},
                    "clientInfo": {"name": "test-client", "version": "1.0.0"}
                }
            });

            eprintln!("DEBUG: Sending initialize message");
            self.send_message(init_msg);
            eprintln!("DEBUG: Reading initialize response");
            let response = self.read_response();
            eprintln!("DEBUG: Initialize response received: {:?}", response);

            // Send initialized notification
            let initialized = json!({
                "jsonrpc": "2.0",
                "method": "notifications/initialized"
            });
            eprintln!("DEBUG: Sending initialized notification");
            self.send_message(initialized);
            eprintln!("DEBUG: Initialization complete");
        }

        fn next_id(&mut self) -> u64 {
            self.message_id += 1;
            self.message_id
        }

        fn send_message(&mut self, msg: Value) {
            let msg_str = serde_json::to_string(&msg).unwrap() + "\n";
            self.stdin.write_all(msg_str.as_bytes()).unwrap();
            self.stdin.flush().unwrap();
        }

        fn read_response(&mut self) -> Value {
            // Check if the child process is still alive
            if let Some(exit_status) = self.child.try_wait().expect("Failed to check child status")
            {
                panic!(
                    "Server process terminated with exit code: {:?}",
                    exit_status
                );
            }

            let mut line = String::new();
            let bytes_read = self
                .reader
                .read_line(&mut line)
                .expect("Failed to read line");

            if bytes_read == 0 {
                panic!("EOF reached while reading response - server may have terminated");
            }

            let trimmed = line.trim();
            eprintln!("DEBUG: Read line: {:?}", trimmed);

            if trimmed.is_empty() {
                panic!("Empty line received from server");
            }

            serde_json::from_str(trimmed).expect("Failed to parse JSON response")
        }

        fn call_tool(&mut self, tool_name: &str, arguments: Value) -> Value {
            let msg = json!({
                "jsonrpc": "2.0",
                "id": self.next_id(),
                "method": "tools/call",
                "params": {
                    "name": tool_name,
                    "arguments": arguments
                }
            });

            eprintln!("DEBUG: Calling tool {} with args: {}", tool_name, arguments);
            self.send_message(msg);
            let response = self.read_response();
            eprintln!("DEBUG: Tool {} response received", tool_name);
            response
        }

        fn extract_text_response(&self, response: &Value) -> String {
            response["result"]["content"][0]["text"]
                .as_str()
                .unwrap_or("")
                .to_string()
        }

        fn extract_session_id(&self, create_response: &Value) -> String {
            let text = self.extract_text_response(create_response);
            // Extract session ID from text like "Session ID: abc123..."
            text.lines()
                .find(|line| line.starts_with("Session ID:"))
                .and_then(|line| line.split(": ").nth(1))
                .unwrap_or("")
                .to_string()
        }
    }

    impl Drop for McpClient {
        fn drop(&mut self) {
            let _ = self.child.kill();
        }
    }

    #[tokio::test]
    #[cfg(not(ci))]
    async fn test_complete_terminal_workflow() {
        let mut client = McpClient::new();

        eprintln!("=== Starting complete terminal workflow test ===");

        // Test 1: Create session
        eprintln!("=== Test 1: Create session ===");
        let create_response = client.call_tool(
            "ht_create_session",
            json!({
                "command": ["bash"],
                "enableWebServer": false
            }),
        );

        assert!(create_response["result"]["content"][0]["text"]
            .as_str()
            .unwrap()
            .contains("Session ID:"));
        let session_id = client.extract_session_id(&create_response);
        assert!(!session_id.is_empty());
        eprintln!("=== Session created with ID: {} ===", session_id);

        // Test 2: List sessions
        eprintln!("=== Test 2: List sessions ===");
        let list_response = client.call_tool("ht_list_sessions", json!({}));
        let list_text = client.extract_text_response(&list_response);
        assert!(list_text.contains("Active HT Sessions (1)"));
        assert!(list_text.contains(&session_id));

        // Test 3: Send keys
        eprintln!("=== Test 3: Send keys ===");
        let send_keys_response = client.call_tool(
            "ht_send_keys",
            json!({
                "sessionId": session_id,
                "keys": ["echo 'test command'", "Enter"]
            }),
        );
        let keys_text = client.extract_text_response(&send_keys_response);
        assert!(keys_text.contains("Keys sent successfully"));
        assert!(keys_text.contains(&session_id));

        // Wait for command to execute
        tokio::time::sleep(Duration::from_millis(1000)).await;

        // Test 4: Take snapshot
        eprintln!("=== Test 4: Take snapshot ===");
        let snapshot_response = client.call_tool(
            "ht_take_snapshot",
            json!({
                "sessionId": session_id
            }),
        );
        let snapshot_text = client.extract_text_response(&snapshot_response);
        assert!(snapshot_text.contains("Terminal Snapshot"));
        assert!(snapshot_text.contains("```")); // Should have markdown code blocks
        assert!(snapshot_text.contains("test command")); // Should show our command

        // Test 5: Execute command
        eprintln!("=== Test 5: Execute command ===");
        let execute_response = client.call_tool(
            "ht_execute_command",
            json!({
                "sessionId": session_id,
                "command": "whoami"
            }),
        );
        let execute_text = client.extract_text_response(&execute_response);
        assert!(execute_text.contains("Command executed: whoami"));
        assert!(execute_text.contains("Terminal Output:"));
        assert!(execute_text.contains("```"));

        // Test 6: Close session
        eprintln!("=== Test 6: Close session ===");
        let close_response = client.call_tool(
            "ht_close_session",
            json!({
                "sessionId": session_id
            }),
        );
        let close_text = client.extract_text_response(&close_response);
        assert!(close_text.contains("closed successfully"));
        eprintln!("Session close response: {}", close_text);

        // Test 7: Verify session is closed
        eprintln!("=== Test 7: Verify session is closed ===");
        let final_list = client.call_tool("ht_list_sessions", json!({}));
        let final_text = client.extract_text_response(&final_list);
        assert!(
            final_text.contains("Active HT Sessions (0)")
                || final_text.contains("No active sessions")
        );
        eprintln!("Session list verification successful: {}", final_text);

        eprintln!("=== Test completed successfully! ===");
    }

    #[tokio::test]
    #[cfg(not(ci))]
    async fn test_web_server_enabled() {
        let mut client = McpClient::new();

        // Create session with web server enabled
        let create_response = client.call_tool(
            "ht_create_session",
            json!({
                "command": ["bash"],
                "enableWebServer": true
            }),
        );

        let response_text = client.extract_text_response(&create_response);

        // Should contain session ID
        assert!(response_text.contains("Session ID:"));

        // Should contain web server info with emoji
        assert!(response_text.contains("🌐 Web server enabled!"));
        assert!(response_text.contains("http://127.0.0.1:"));

        // Clean up
        let session_id = client.extract_session_id(&create_response);
        client.call_tool("ht_close_session", json!({"sessionId": session_id}));
    }

    #[tokio::test]
    #[cfg(not(ci))]
    async fn test_error_handling() {
        let mut client = McpClient::new();

        // Test 1: Invalid session ID
        let invalid_snapshot = client.call_tool(
            "ht_take_snapshot",
            json!({
                "sessionId": "invalid-session-id"
            }),
        );

        // Should return error
        assert!(invalid_snapshot.get("error").is_some());

        // Test 2: Missing required parameters
        let missing_params = client.call_tool(
            "ht_send_keys",
            json!({
                "sessionId": "test"
                // Missing "keys" parameter
            }),
        );

        assert!(missing_params.get("error").is_some());
    }

    #[tokio::test]
    #[cfg(not(ci))]
    async fn test_response_format_consistency() {
        let mut client = McpClient::new();

        // Create session
        let create_response = client.call_tool(
            "ht_create_session",
            json!({
                "command": ["bash"],
                "enableWebServer": false
            }),
        );

        let session_id = client.extract_session_id(&create_response);

        // Test that all responses follow the expected format
        let tests = vec![
            ("ht_list_sessions", json!({})),
            (
                "ht_send_keys",
                json!({"sessionId": &session_id, "keys": ["echo test"]}),
            ),
            ("ht_take_snapshot", json!({"sessionId": &session_id})),
            (
                "ht_execute_command",
                json!({"sessionId": &session_id, "command": "echo test"}),
            ),
        ];

        for (tool_name, args) in tests {
            let response = client.call_tool(tool_name, args);

            // All responses should have the expected MCP structure
            assert_eq!(response["jsonrpc"], "2.0");
            assert!(response["id"].is_number());
            assert!(response["result"].is_object());
            assert!(response["result"]["content"].is_array());
            assert_eq!(response["result"]["content"][0]["type"], "text");
            assert!(response["result"]["content"][0]["text"].is_string());

            // All text responses should be human-readable (not JSON)
            let text = response["result"]["content"][0]["text"].as_str().unwrap();
            assert!(!text.starts_with('{')); // Should not be JSON
            assert!(!text.starts_with('[')); // Should not be JSON array
        }

        // Clean up
        client.call_tool("ht_close_session", json!({"sessionId": session_id}));
    }
} // End of integration_tests module
