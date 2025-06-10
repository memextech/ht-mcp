use thiserror::Error;

pub type Result<T> = std::result::Result<T, HtMcpError>;

#[derive(Error, Debug)]
pub enum HtMcpError {
    #[error("MCP error: {0}")]
    Mcp(String),
    
    #[error("HT library error: {0}")]
    HtLibrary(String),
    
    #[error("Session not found: {0}")]
    SessionNotFound(String),
    
    #[error("Invalid request: {0}")]
    InvalidRequest(String),
    
    #[error("Internal error: {0}")]
    Internal(String),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}