use axum::{
    extract::{State, WebSocketUpgrade},
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Json, Router,
};
use serde_json::json;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::{broadcast, RwLock};
use tower_http::cors::CorsLayer;
use tracing::{error, info, warn};

use crate::error::{HtMcpError, Result};

/// Manages web servers for terminal sessions
#[derive(Clone)]
pub struct WebServerManager {
    servers: Arc<RwLock<HashMap<String, WebServerInfo>>>,
}

/// Information about a running web server
#[derive(Debug, Clone)]
pub struct WebServerInfo {
    pub session_id: String,
    pub port: u16,
    pub url: String,
    pub shutdown_tx: broadcast::Sender<()>,
}

/// Web server for a specific terminal session
pub struct WebServer {
    session_id: String,
    port: u16,
    snapshot_provider: Arc<dyn SnapshotProvider + Send + Sync>,
}

/// Trait for providing terminal snapshots to the web server
pub trait SnapshotProvider {
    fn get_snapshot(&self, session_id: &str) -> Result<String>;
    fn get_session_info(&self, session_id: &str) -> Result<SessionInfo>;
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct SessionInfo {
    pub id: String,
    pub is_alive: bool,
    pub created_at: String,
    pub command: Vec<String>,
}

impl WebServerManager {
    pub fn new() -> Self {
        Self {
            servers: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Start a web server for a terminal session
    pub async fn start_server(
        &self,
        session_id: String,
        snapshot_provider: Arc<dyn SnapshotProvider + Send + Sync>,
    ) -> Result<String> {
        let port = self.find_available_port().await?;
        
        let web_server = WebServer {
            session_id: session_id.clone(),
            port,
            snapshot_provider,
        };

        let (shutdown_tx, _) = broadcast::channel(1);
        let shutdown_rx = shutdown_tx.subscribe();
        
        let url = format!("http://127.0.0.1:{}", port);
        
        let server_info = WebServerInfo {
            session_id: session_id.clone(),
            port,
            url: url.clone(),
            shutdown_tx,
        };

        // Store server info
        self.servers.write().await.insert(session_id.clone(), server_info);

        // Start the server in background
        let servers_clone = self.servers.clone();
        let session_id_clone = session_id.clone();
        tokio::spawn(async move {
            if let Err(e) = web_server.run(shutdown_rx).await {
                error!("Web server for session {} failed: {}", session_id_clone, e);
                // Remove from active servers on failure
                servers_clone.write().await.remove(&session_id_clone);
            }
        });

        info!("Started web server for session {} on {}", session_id, url);
        Ok(url)
    }

    /// Stop a web server for a session
    pub async fn stop_server(&self, session_id: &str) -> Result<()> {
        let mut servers = self.servers.write().await;
        
        if let Some(server_info) = servers.remove(session_id) {
            // Send shutdown signal
            if let Err(e) = server_info.shutdown_tx.send(()) {
                warn!("Failed to send shutdown signal to web server: {}", e);
            }
            info!("Stopped web server for session {}", session_id);
        }

        Ok(())
    }

    /// Get web server URL for a session
    pub async fn get_server_url(&self, session_id: &str) -> Option<String> {
        self.servers.read().await
            .get(session_id)
            .map(|info| info.url.clone())
    }

    /// Find an available port for the web server
    async fn find_available_port(&self) -> Result<u16> {
        // Start from port 3000 and find first available
        for port in 3000..4000 {
            if let Ok(listener) = TcpListener::bind(format!("127.0.0.1:{}", port)).await {
                drop(listener); // Close the listener
                return Ok(port);
            }
        }
        Err(HtMcpError::Internal("No available ports found".to_string()))
    }
}

impl WebServer {
    /// Run the web server
    async fn run(self, mut shutdown_rx: broadcast::Receiver<()>) -> Result<()> {
        let snapshot_provider = self.snapshot_provider.clone();
        let session_id = self.session_id.clone();

        // Create shared state for handlers
        let app_state = AppState {
            session_id: session_id.clone(),
            snapshot_provider,
        };

        // Build the router
        let app = Router::new()
            .route("/", get(terminal_page))
            .route("/api/snapshot", get(get_snapshot))
            .route("/api/session", get(get_session_info))
            .route("/ws", get(websocket_handler))
            .layer(CorsLayer::permissive())
            .with_state(app_state);

        // Bind to the port
        let addr = SocketAddr::from(([127, 0, 0, 1], self.port));
        let listener = TcpListener::bind(addr).await
            .map_err(|e| HtMcpError::Internal(format!("Failed to bind to port {}: {}", self.port, e)))?;

        info!("Web server listening on {}", addr);

        // Run the server with graceful shutdown
        tokio::select! {
            result = axum::serve(listener, app) => {
                if let Err(e) = result {
                    error!("Web server error: {}", e);
                    return Err(HtMcpError::Internal(format!("Web server error: {}", e)));
                }
            }
            _ = shutdown_rx.recv() => {
                info!("Web server for session {} shutting down", session_id);
            }
        }

        Ok(())
    }
}

/// Shared state for web server handlers
#[derive(Clone)]
struct AppState {
    session_id: String,
    snapshot_provider: Arc<dyn SnapshotProvider + Send + Sync>,
}

/// Handler for the main terminal page
async fn terminal_page() -> impl IntoResponse {
    Html(include_str!("../assets/terminal.html"))
}

/// Handler for getting terminal snapshot
async fn get_snapshot(State(state): State<AppState>) -> impl IntoResponse {
    match state.snapshot_provider.get_snapshot(&state.session_id) {
        Ok(snapshot) => Json(json!({
            "success": true,
            "snapshot": snapshot,
            "sessionId": state.session_id
        })).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "success": false,
                "error": e.to_string()
            }))
        ).into_response()
    }
}

/// Handler for getting session information
async fn get_session_info(State(state): State<AppState>) -> impl IntoResponse {
    match state.snapshot_provider.get_session_info(&state.session_id) {
        Ok(info) => Json(json!({
            "success": true,
            "session": info
        })).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "success": false,
                "error": e.to_string()
            }))
        ).into_response()
    }
}

/// WebSocket handler for real-time terminal updates
async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> axum::response::Response {
    ws.on_upgrade(|socket| async move {
        // TODO: Implement WebSocket communication for real-time updates
        // This would stream terminal output changes to the browser
        info!("WebSocket connection established for session {}", state.session_id);
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    struct MockSnapshotProvider;

    impl SnapshotProvider for MockSnapshotProvider {
        fn get_snapshot(&self, _session_id: &str) -> Result<String> {
            Ok("bash-3.2$ echo test\ntest\nbash-3.2$ ".to_string())
        }

        fn get_session_info(&self, session_id: &str) -> Result<SessionInfo> {
            Ok(SessionInfo {
                id: session_id.to_string(),
                is_alive: true,
                created_at: "2025-01-01T00:00:00Z".to_string(),
                command: vec!["bash".to_string()],
            })
        }
    }

    #[tokio::test]
    async fn test_web_server_manager() {
        let manager = WebServerManager::new();
        let provider = Arc::new(MockSnapshotProvider);
        
        let url = manager.start_server("test-session".to_string(), provider).await.unwrap();
        assert!(url.starts_with("http://127.0.0.1:"));
        
        let retrieved_url = manager.get_server_url("test-session").await;
        assert_eq!(retrieved_url, Some(url));
        
        manager.stop_server("test-session").await.unwrap();
        
        let after_stop = manager.get_server_url("test-session").await;
        assert_eq!(after_stop, None);
    }
}