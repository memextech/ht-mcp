// Standard I/O transport for MCP communication
// This will be implemented when we integrate with the MCP SDK

use crate::error::Result;

pub struct StdioTransport;

impl StdioTransport {
    pub fn new() -> Self {
        Self
    }

    // Placeholder for stdio transport
    pub async fn start(&self) -> Result<()> {
        tracing::info!("Starting stdio transport");
        Ok(())
    }
}
