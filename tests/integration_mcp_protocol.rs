use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader, Write};
use serde_json::{json, Value};
#[cfg(not(ci))]
use std::time::Duration;

/// Integration test for MCP protocol compliance
#[tokio::test]
async fn test_mcp_protocol_basic_flow() {
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

    // Helper function to send MCP message
    let send_message = |stdin: &mut std::process::ChildStdin, msg: Value| {
        let msg_str = serde_json::to_string(&msg).unwrap() + "\n";
        stdin.write_all(msg_str.as_bytes()).unwrap();
        stdin.flush().unwrap();
    };

    // Helper function to read MCP response
    let read_response = |reader: &mut BufReader<std::process::ChildStdout>| -> Value {
        let mut line = String::new();
        reader.read_line(&mut line).unwrap();
        serde_json::from_str(&line.trim()).unwrap()
    };

    // Test 1: Initialize
    let init_msg = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "initialize",
        "params": {
            "protocolVersion": "2024-11-05",
            "capabilities": {},
            "clientInfo": {"name": "test-client", "version": "1.0.0"}
        }
    });

    send_message(&mut stdin, init_msg);
    let response = read_response(&mut reader);
    
    assert_eq!(response["jsonrpc"], "2.0");
    assert_eq!(response["id"], 1);
    assert!(response["result"].is_object());
    assert_eq!(response["result"]["protocolVersion"], "2024-11-05");

    // Test 2: Send initialized notification
    let initialized = json!({
        "jsonrpc": "2.0",
        "method": "notifications/initialized"
    });
    send_message(&mut stdin, initialized);

    // Test 3: List tools
    let list_tools = json!({
        "jsonrpc": "2.0",
        "id": 2,
        "method": "tools/list",
        "params": {}
    });

    send_message(&mut stdin, list_tools);
    let tools_response = read_response(&mut reader);

    assert_eq!(tools_response["id"], 2);
    let tools = &tools_response["result"]["tools"];
    assert!(tools.is_array());
    
    // Verify all 6 tools are present
    let tool_names: Vec<&str> = tools
        .as_array()
        .unwrap()
        .iter()
        .map(|t| t["name"].as_str().unwrap())
        .collect();
    
    assert!(tool_names.contains(&"ht_create_session"));
    assert!(tool_names.contains(&"ht_send_keys"));
    assert!(tool_names.contains(&"ht_take_snapshot"));
    assert!(tool_names.contains(&"ht_execute_command"));
    assert!(tool_names.contains(&"ht_list_sessions"));
    assert!(tool_names.contains(&"ht_close_session"));

    // Clean up
    child.kill().expect("Failed to kill child process");
}

#[tokio::test] 
async fn test_create_session_tool() {
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

    // Initialize first
    let init_and_notify = |stdin: &mut std::process::ChildStdin| {
        let init = json!({
            "jsonrpc": "2.0", "id": 1, "method": "initialize",
            "params": {"protocolVersion": "2024-11-05", "capabilities": {}, "clientInfo": {"name": "test", "version": "1.0"}}
        });
        let msg = serde_json::to_string(&init).unwrap() + "\n";
        stdin.write_all(msg.as_bytes()).unwrap();
        stdin.flush().unwrap();

        let notify = json!({"jsonrpc": "2.0", "method": "notifications/initialized"});
        let msg = serde_json::to_string(&notify).unwrap() + "\n";
        stdin.write_all(msg.as_bytes()).unwrap();
        stdin.flush().unwrap();
    };

    init_and_notify(&mut stdin);
    
    // Read init response
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();

    // Test create_session
    let create_session = json!({
        "jsonrpc": "2.0",
        "id": 2,
        "method": "tools/call",
        "params": {
            "name": "ht_create_session",
            "arguments": {
                "command": ["bash"],
                "enableWebServer": false
            }
        }
    });

    let msg = serde_json::to_string(&create_session).unwrap() + "\n";
    stdin.write_all(msg.as_bytes()).unwrap();
    stdin.flush().unwrap();

    let mut response_line = String::new();
    reader.read_line(&mut response_line).unwrap();
    let response: Value = serde_json::from_str(&response_line.trim()).unwrap();

    assert_eq!(response["id"], 2);
    assert!(response["result"]["content"][0]["text"].as_str().unwrap().contains("Session ID:"));

    // Clean up
    child.kill().expect("Failed to kill child process");
}