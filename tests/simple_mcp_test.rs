use serde_json::{json, Value};
use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};
use std::time::Duration;

#[tokio::test]
#[cfg(not(ci))]
async fn test_simple_mcp_initialization() {
    // Build the server first
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

    // Start the server
    let mut child = Command::new("./target/debug/ht-mcp")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start ht-mcp server");

    let mut stdin = child.stdin.take().expect("Failed to get stdin");
    let stdout = child.stdout.take().expect("Failed to get stdout");
    let mut reader = BufReader::new(stdout);

    // Give the server a moment to start
    tokio::time::sleep(Duration::from_millis(100)).await;

    // Send initialize message
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

    println!("Sending initialize message...");
    let msg_str = serde_json::to_string(&init_msg).unwrap() + "\n";
    stdin.write_all(msg_str.as_bytes()).unwrap();
    stdin.flush().unwrap();

    // Read response with timeout
    println!("Reading response...");
    let mut line = String::new();
    let bytes_read = reader
        .read_line(&mut line)
        .expect("Failed to read response");

    println!("Response received: {} bytes", bytes_read);
    println!("Response content: {}", line.trim());

    // Parse and validate response
    let response: Value = serde_json::from_str(line.trim()).expect("Failed to parse JSON");

    assert_eq!(response["jsonrpc"], "2.0");
    assert_eq!(response["id"], 1);
    assert!(response["result"].is_object());
    assert_eq!(response["result"]["protocolVersion"], "2024-11-05");
    assert!(response["result"]["serverInfo"]["name"]
        .as_str()
        .unwrap()
        .contains("ht-mcp"));

    // Clean up - kill and wait to prevent zombie processes
    let _ = child.kill();
    let _ = child.wait();

    println!("Test passed!");
}
