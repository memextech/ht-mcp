use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug, Deserialize)]
pub struct CreateSessionArgs {
    pub command: Option<Vec<String>>,
    #[serde(rename = "enableWebServer")]
    pub enable_web_server: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct CreateSessionResult {
    #[serde(rename = "sessionId")]
    pub session_id: String,
    pub message: String,
    #[serde(rename = "webServerEnabled")]
    pub web_server_enabled: bool,
    #[serde(rename = "webServerUrl")]
    pub web_server_url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SendKeysArgs {
    #[serde(rename = "sessionId")]
    pub session_id: String,
    pub keys: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct TakeSnapshotArgs {
    #[serde(rename = "sessionId")]
    pub session_id: String,
}

#[derive(Debug, Serialize)]
pub struct SnapshotResult {
    #[serde(rename = "sessionId")]
    pub session_id: String,
    pub snapshot: String,
}

#[derive(Debug, Deserialize)]
pub struct ExecuteCommandArgs {
    #[serde(rename = "sessionId")]
    pub session_id: String,
    pub command: String,
}

#[derive(Debug, Deserialize)]
pub struct CloseSessionArgs {
    #[serde(rename = "sessionId")]
    pub session_id: String,
}

#[derive(Debug, Deserialize)]
pub struct ResizeArgs {
    #[serde(rename = "sessionId")]
    pub session_id: String,
    pub cols: usize,
    pub rows: usize,
}

// Schema generation functions
pub fn create_session_schema() -> Value {
    let default_command = if cfg!(windows) {
        "[\"powershell.exe\"]"
    } else {
        "[\"bash\"]"
    };

    json!({
        "type": "object",
        "properties": {
            "command": {
                "type": "array",
                "items": {"type": "string"},
                "description": format!("Command to run in the terminal (default: {})", default_command)
            },
            "enableWebServer": {
                "type": "boolean",
                "description": "Enable HT web server for live terminal preview (default: false)"
            }
        },
        "additionalProperties": false
    })
}

pub fn send_keys_schema() -> Value {
    json!({
        "type": "object",
        "properties": {
            "sessionId": {
                "type": "string",
                "description": "HT session ID"
            },
            "keys": {
                "type": "array",
                "items": {"type": "string"},
                "description": "Array of keys to send (can include text and special keys like \"Enter\", \"Down\", etc.)"
            }
        },
        "required": ["sessionId", "keys"],
        "additionalProperties": false
    })
}

pub fn take_snapshot_schema() -> Value {
    json!({
        "type": "object",
        "properties": {
            "sessionId": {
                "type": "string",
                "description": "HT session ID"
            }
        },
        "required": ["sessionId"],
        "additionalProperties": false
    })
}

pub fn execute_command_schema() -> Value {
    json!({
        "type": "object",
        "properties": {
            "sessionId": {
                "type": "string",
                "description": "HT session ID"
            },
            "command": {
                "type": "string",
                "description": "Command to execute in the terminal"
            }
        },
        "required": ["sessionId", "command"],
        "additionalProperties": false
    })
}

pub fn list_sessions_schema() -> Value {
    json!({
        "type": "object",
        "properties": {},
        "additionalProperties": false
    })
}

pub fn close_session_schema() -> Value {
    json!({
        "type": "object",
        "properties": {
            "sessionId": {
                "type": "string",
                "description": "HT session ID to close"
            }
        },
        "required": ["sessionId"],
        "additionalProperties": false
    })
}

pub fn resize_schema() -> Value {
    json!({
        "type": "object",
        "properties": {
            "sessionId": {
                "type": "string",
                "description": "HT session ID"
            },
            "cols": {
                "type": "integer",
                "minimum": 1,
                "maximum": 1000,
                "description": "Number of columns (width) for the terminal"
            },
            "rows": {
                "type": "integer",
                "minimum": 1,
                "maximum": 1000,
                "description": "Number of rows (height) for the terminal"
            }
        },
        "required": ["sessionId", "cols", "rows"],
        "additionalProperties": false
    })
}
