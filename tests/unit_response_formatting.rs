use serde_json::json;

// Since the format_tool_response function is in main.rs and not public,
// we'll create similar tests here for the response format logic

#[test]
fn test_create_session_response_format() {
    let mock_response = json!({
        "sessionId": "test-session-123",
        "webServerEnabled": true,
        "webServerUrl": "http://127.0.0.1:3618"
    });

    let formatted = format_create_session_response(&mock_response);

    assert!(formatted.contains("HT session created successfully!"));
    assert!(formatted.contains("Session ID: test-session-123"));
    assert!(formatted.contains("🌐 Web server enabled!"));
    assert!(formatted.contains("http://127.0.0.1:3618"));
}

#[test]
fn test_create_session_response_no_web_server() {
    let mock_response = json!({
        "sessionId": "test-session-456",
        "webServerEnabled": false,
        "webServerUrl": null
    });

    let formatted = format_create_session_response(&mock_response);

    assert!(formatted.contains("HT session created successfully!"));
    assert!(formatted.contains("Session ID: test-session-456"));
    assert!(!formatted.contains("🌐"));
    assert!(!formatted.contains("Web server"));
}

#[test]
fn test_snapshot_response_format() {
    let mock_response = json!({
        "sessionId": "snap-session-789",
        "snapshot": "bash-3.2$ echo hello\nhello\nbash-3.2$ "
    });

    let formatted = format_snapshot_response(&mock_response);

    assert!(formatted.contains("Terminal Snapshot (Session: snap-session-789)"));
    assert!(formatted.contains("```"));
    assert!(formatted.contains("bash-3.2$ echo hello"));
    assert!(formatted.contains("hello"));
}

#[test]
fn test_send_keys_response_format() {
    let mock_response = json!({
        "sessionId": "keys-session-abc",
        "keys": ["echo test", "Enter"]
    });

    let formatted = format_send_keys_response(&mock_response);

    assert!(formatted.contains("Keys sent successfully to session keys-session-abc"));
    assert!(formatted.contains("Keys: [\"echo test\",\"Enter\"]"));
}

#[test]
fn test_execute_command_response_format() {
    let mock_response = json!({
        "sessionId": "exec-session-def",
        "command": "ls -la",
        "output": "total 16\ndrwxr-xr-x  3 user  staff   96 Jun 13 10:00 .\ndrwxr-xr-x  4 user  staff  128 Jun 13 09:00 .."
    });

    let formatted = format_execute_command_response(&mock_response);

    assert!(formatted.contains("Command executed: ls -la"));
    assert!(formatted.contains("Terminal Output:"));
    assert!(formatted.contains("```"));
    assert!(formatted.contains("total 16"));
    assert!(formatted.contains("drwxr-xr-x"));
}

#[test]
fn test_list_sessions_response_format() {
    let mock_response = json!({
        "count": 2,
        "sessions": [
            {
                "id": "session-1",
                "isAlive": true,
                "createdAt": 1234567890
            },
            {
                "id": "session-2",
                "isAlive": false,
                "createdAt": 1234567891
            }
        ]
    });

    let formatted = format_list_sessions_response(&mock_response);

    assert!(formatted.contains("Active HT Sessions (2)"));
    assert!(formatted.contains("- session-1 (alive) - Created: 1234567890"));
    assert!(formatted.contains("- session-2 (dead) - Created: 1234567891"));
}

#[test]
fn test_list_sessions_empty_response_format() {
    let mock_response = json!({
        "count": 0,
        "sessions": []
    });

    let formatted = format_list_sessions_response(&mock_response);

    assert!(formatted.contains("Active HT Sessions (0)"));
    assert!(formatted.contains("No active sessions"));
}

#[test]
fn test_close_session_response_format() {
    let mock_response = json!({
        "sessionId": "close-session-ghi"
    });

    let formatted = format_close_session_response(&mock_response);

    assert!(formatted.contains("Session close-session-ghi closed successfully."));
}

// Helper functions that mirror the logic in main.rs
fn format_create_session_response(result: &serde_json::Value) -> String {
    let session_id = result["sessionId"].as_str().unwrap_or("unknown");
    let web_server_enabled = result["webServerEnabled"].as_bool().unwrap_or(false);
    let web_server_url = result["webServerUrl"].as_str();

    let web_server_info = if web_server_enabled {
        if let Some(url) = web_server_url {
            format!("\n\n🌐 Web server enabled! View live terminal at: {}", url)
        } else {
            "\n\n🌐 Web server enabled! Check console for URL.".to_string()
        }
    } else {
        String::new()
    };

    format!(
        "HT session created successfully!\n\nSession ID: {}\n\nYou can now use this session ID with other HT tools to send commands and take snapshots.{}",
        session_id, web_server_info
    )
}

fn format_snapshot_response(result: &serde_json::Value) -> String {
    let session_id = result["sessionId"].as_str().unwrap_or("unknown");
    let snapshot = result["snapshot"].as_str().unwrap_or("No snapshot data");

    format!(
        "Terminal Snapshot (Session: {})\n\n```\n{}\n```",
        session_id, snapshot
    )
}

fn format_send_keys_response(result: &serde_json::Value) -> String {
    let session_id = result["sessionId"].as_str().unwrap_or("unknown");
    let keys = result["keys"]
        .as_array()
        .map(|arr| {
            arr.iter()
                .map(|v| v.as_str().unwrap_or("").to_string())
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    format!(
        "Keys sent successfully to session {}\n\nKeys: {}",
        session_id,
        serde_json::to_string(&keys).unwrap_or_else(|_| "[]".to_string())
    )
}

fn format_execute_command_response(result: &serde_json::Value) -> String {
    let command = result["command"].as_str().unwrap_or("unknown");
    let output = result["output"].as_str().unwrap_or("No output");

    format!(
        "Command executed: {}\n\nTerminal Output:\n```\n{}\n```",
        command, output
    )
}

fn format_list_sessions_response(result: &serde_json::Value) -> String {
    let count = result["count"].as_u64().unwrap_or(0);
    let default_sessions = vec![];
    let sessions = result["sessions"].as_array().unwrap_or(&default_sessions);

    if sessions.is_empty() {
        format!("Active HT Sessions ({}):\n\nNo active sessions", count)
    } else {
        let session_list: Vec<String> = sessions
            .iter()
            .map(|session| {
                let id = session["id"].as_str().unwrap_or("unknown");
                let is_alive = session["isAlive"].as_bool().unwrap_or(false);
                let created_at = session["createdAt"].as_u64().unwrap_or(0);

                format!(
                    "- {} ({}) - Created: {}",
                    id,
                    if is_alive { "alive" } else { "dead" },
                    created_at
                )
            })
            .collect();

        format!(
            "Active HT Sessions ({}):\n\n{}",
            count,
            session_list.join("\n")
        )
    }
}

fn format_close_session_response(result: &serde_json::Value) -> String {
    let session_id = result["sessionId"].as_str().unwrap_or("unknown");
    format!("Session {} closed successfully.", session_id)
}

#[test]
fn test_resize_response_format() {
    let mock_response = json!({
        "sessionId": "resize-session-123",
        "cols": 120,
        "rows": 40
    });

    let formatted = format_resize_response(&mock_response);

    assert!(formatted.contains("Terminal resized successfully!"));
    assert!(formatted.contains("Session: resize-session-123"));
    assert!(formatted.contains("New size: 120x40 (columns x rows)"));
}

#[test]
fn test_resize_response_format_edge_cases() {
    // Test minimum size
    let min_response = json!({
        "sessionId": "min-session",
        "cols": 1,
        "rows": 1
    });

    let min_formatted = format_resize_response(&min_response);
    assert!(min_formatted.contains("Terminal resized successfully!"));
    assert!(min_formatted.contains("1x1"));

    // Test maximum size
    let max_response = json!({
        "sessionId": "max-session",
        "cols": 1000,
        "rows": 1000
    });

    let max_formatted = format_resize_response(&max_response);
    assert!(max_formatted.contains("Terminal resized successfully!"));
    assert!(max_formatted.contains("1000x1000"));

    // Test with unknown session ID
    let unknown_response = json!({
        "cols": 80,
        "rows": 24
    });

    let unknown_formatted = format_resize_response(&unknown_response);
    assert!(unknown_formatted.contains("Session: unknown"));
    assert!(unknown_formatted.contains("80x24"));
}

fn format_resize_response(result: &serde_json::Value) -> String {
    let session_id = result["sessionId"].as_str().unwrap_or("unknown");
    let cols = result["cols"].as_u64().unwrap_or(0);
    let rows = result["rows"].as_u64().unwrap_or(0);

    format!(
        "Terminal resized successfully!\n\nSession: {}\nNew size: {}x{} (columns x rows)",
        session_id, cols, rows
    )
}
