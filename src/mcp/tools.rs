use crate::mcp::types::*;

pub fn get_tool_definitions() -> Vec<serde_json::Value> {
    vec![
        serde_json::json!({
            "name": "ht_create_session",
            "description": "Create a new HT session",
            "inputSchema": create_session_schema()
        }),
        serde_json::json!({
            "name": "ht_send_keys",
            "description": "Send keys to an HT session",
            "inputSchema": send_keys_schema()
        }),
        serde_json::json!({
            "name": "ht_take_snapshot",
            "description": "Take a snapshot of the terminal state",
            "inputSchema": take_snapshot_schema()
        }),
        serde_json::json!({
            "name": "ht_execute_command",
            "description": "Execute a command and return output",
            "inputSchema": execute_command_schema()
        }),
        serde_json::json!({
            "name": "ht_list_sessions",
            "description": "List all active sessions",
            "inputSchema": list_sessions_schema()
        }),
        serde_json::json!({
            "name": "ht_close_session",
            "description": "Close an HT session",
            "inputSchema": close_session_schema()
        }),
        serde_json::json!({
            "name": "ht_resize",
            "description": "Resize the terminal window",
            "inputSchema": resize_schema()
        }),
    ]
}
