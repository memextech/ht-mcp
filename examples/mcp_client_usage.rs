//! Example demonstrating MCP protocol interaction with ht-mcp server
//!
//! This example shows how to interact with the ht-mcp server using
//! the MCP JSON-RPC protocol for terminal automation.
//!
//! Run with: `cargo run --example mcp_client_usage`

use serde_json::{json, Value};
use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Testing ht-mcp server via MCP protocol...");

    // Start the ht-mcp server
    let mut child = Command::new("cargo")
        .args(&["run", "--"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start ht-mcp server");

    let mut stdin = child.stdin.take().expect("Failed to get stdin");
    let stdout = child.stdout.take().expect("Failed to get stdout");
    let mut reader = BufReader::new(stdout);

    // Helper to send MCP message
    let send_message = |stdin: &mut std::process::ChildStdin, msg: Value| {
        let msg_str = serde_json::to_string(&msg).unwrap() + "\n";
        stdin.write_all(msg_str.as_bytes()).unwrap();
        stdin.flush().unwrap();
    };

    // Helper to read response
    let read_response = |reader: &mut BufReader<std::process::ChildStdout>| -> Value {
        let mut line = String::new();
        reader.read_line(&mut line).unwrap();
        serde_json::from_str(&line.trim()).unwrap()
    };

    println!("🤝 Initializing MCP connection...");

    // Initialize the server
    let init_msg = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "initialize",
        "params": {
            "protocolVersion": "2024-11-05",
            "capabilities": {},
            "clientInfo": {"name": "example-client", "version": "1.0.0"}
        }
    });

    send_message(&mut stdin, init_msg);
    let init_response = read_response(&mut reader);
    println!(
        "✅ Server initialized: {}",
        init_response["result"]["serverInfo"]["name"]
    );

    // Send initialized notification
    let initialized = json!({
        "jsonrpc": "2.0",
        "method": "notifications/initialized"
    });
    send_message(&mut stdin, initialized);

    println!("📝 Creating terminal session...");

    // Create a session
    let create_session = json!({
        "jsonrpc": "2.0",
        "id": 2,
        "method": "tools/call",
        "params": {
            "name": "ht_create_session",
            "arguments": {
                "command": ["bash"],
                "enableWebServer": true
            }
        }
    });

    send_message(&mut stdin, create_session);
    let create_response = read_response(&mut reader);
    let session_text = create_response["result"]["content"][0]["text"]
        .as_str()
        .unwrap();
    println!("✅ {}", session_text);

    // Extract session ID from response
    let session_id = session_text
        .lines()
        .find(|line| line.starts_with("Session ID:"))
        .and_then(|line| line.split(": ").nth(1))
        .unwrap_or("")
        .to_string();

    println!("⌨️  Sending commands to session {}...", session_id);

    // Send some keys
    let send_keys = json!({
        "jsonrpc": "2.0",
        "id": 3,
        "method": "tools/call",
        "params": {
            "name": "ht_send_keys",
            "arguments": {
                "sessionId": session_id,
                "keys": ["echo 'Hello from MCP example!'", "Enter"]
            }
        }
    });

    send_message(&mut stdin, send_keys);
    let keys_response = read_response(&mut reader);
    println!(
        "📤 {}",
        keys_response["result"]["content"][0]["text"]
            .as_str()
            .unwrap()
    );

    // Wait for command to execute
    tokio::time::sleep(Duration::from_millis(1000)).await;

    println!("📸 Taking terminal snapshot...");

    // Take snapshot
    let snapshot = json!({
        "jsonrpc": "2.0",
        "id": 4,
        "method": "tools/call",
        "params": {
            "name": "ht_take_snapshot",
            "arguments": {
                "sessionId": session_id
            }
        }
    });

    send_message(&mut stdin, snapshot);
    let snapshot_response = read_response(&mut reader);
    println!(
        "📋 {}",
        snapshot_response["result"]["content"][0]["text"]
            .as_str()
            .unwrap()
    );

    println!("🚀 Executing command directly...");

    // Execute a command
    let execute = json!({
        "jsonrpc": "2.0",
        "id": 5,
        "method": "tools/call",
        "params": {
            "name": "ht_execute_command",
            "arguments": {
                "sessionId": session_id,
                "command": "whoami"
            }
        }
    });

    send_message(&mut stdin, execute);
    let execute_response = read_response(&mut reader);
    println!(
        "💻 {}",
        execute_response["result"]["content"][0]["text"]
            .as_str()
            .unwrap()
    );

    println!("📋 Listing all sessions...");

    // List sessions
    let list = json!({
        "jsonrpc": "2.0",
        "id": 6,
        "method": "tools/call",
        "params": {
            "name": "ht_list_sessions",
            "arguments": {}
        }
    });

    send_message(&mut stdin, list);
    let list_response = read_response(&mut reader);
    println!(
        "📄 {}",
        list_response["result"]["content"][0]["text"]
            .as_str()
            .unwrap()
    );

    println!("🔒 Closing session...");

    // Close session
    let close = json!({
        "jsonrpc": "2.0",
        "id": 7,
        "method": "tools/call",
        "params": {
            "name": "ht_close_session",
            "arguments": {
                "sessionId": session_id
            }
        }
    });

    send_message(&mut stdin, close);
    let close_response = read_response(&mut reader);
    println!(
        "✅ {}",
        close_response["result"]["content"][0]["text"]
            .as_str()
            .unwrap()
    );

    // Clean up
    child.kill().expect("Failed to kill child process");

    println!("🎉 MCP example completed successfully!");

    Ok(())
}
