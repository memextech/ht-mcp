use serde_json::json;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::Command;

#[tokio::test]
#[cfg(not(ci))] // Skip in CI environments
async fn test_server_metadata_reporting() {
    // Test that the server properly reports its metadata during initialization
    let mut child = Command::new("cargo")
        .arg("run")
        .arg("--")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to start ht-mcp server");

    let mut stdin = child.stdin.take().expect("Failed to get stdin");
    let stdout = child.stdout.take().expect("Failed to get stdout");
    let mut reader = BufReader::new(stdout);

    // Send initialize request
    let init_request = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "initialize",
        "params": {
            "protocolVersion": "2024-11-05",
            "capabilities": {},
            "clientInfo": {
                "name": "test-client",
                "version": "1.0.0"
            }
        }
    });

    let request_str = serde_json::to_string(&init_request).unwrap();
    stdin.write_all(request_str.as_bytes()).await.unwrap();
    stdin.write_all(b"\n").await.unwrap();
    stdin.flush().await.unwrap();

    // Read response
    let mut response_line = String::new();
    reader.read_line(&mut response_line).await.unwrap();

    let response: serde_json::Value = serde_json::from_str(&response_line.trim()).unwrap();

    // Verify response structure
    assert_eq!(response["jsonrpc"], "2.0");
    assert_eq!(response["id"], 1);
    assert!(response["result"].is_object());

    // Verify server metadata
    let server_info = &response["result"]["serverInfo"];
    assert!(server_info.is_object());

    // Check required fields
    assert_eq!(server_info["name"], "ht-mcp");
    assert_eq!(server_info["title"], "Headless Terminal MCP Server");
    assert_eq!(server_info["version"], env!("CARGO_PKG_VERSION"));

    // Verify protocol version
    assert_eq!(response["result"]["protocolVersion"], "2024-11-05");

    // Verify capabilities
    assert!(response["result"]["capabilities"]["tools"].is_object());

    // Clean up
    drop(stdin);
    let _ = child.kill().await;
}

#[test]
fn test_server_info_structure() {
    // Test that the ServerInfo structure has all required fields
    use ht_mcp::mcp::server::HtMcpServer;

    let server = HtMcpServer::new();
    let info = server.server_info();

    // Verify fields
    assert_eq!(info.name, "ht-mcp");
    assert_eq!(info.title, "Headless Terminal MCP Server");
    assert_eq!(info.version, env!("CARGO_PKG_VERSION"));
}
