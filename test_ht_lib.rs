// Simple test to verify HT library works directly
use ht_core::{HtLibrary, SessionConfig, InputSeq};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Testing HT library directly...");
    
    let mut ht_lib = HtLibrary::new();
    
    // Create a session
    let config = SessionConfig {
        command: vec!["bash".to_string()],
        size: (80, 24),
        enable_web_server: false,
    };
    
    println!("Creating session...");
    let session_id = ht_lib.create_session(config).await?;
    println!("Session created: {}", session_id);
    
    // Send a command
    println!("Sending command...");
    let input = vec![InputSeq::Standard("echo 'Hello from HT!'".to_string())];
    ht_lib.send_input(session_id, input).await?;
    
    // Send Enter
    let enter = vec![InputSeq::Standard("\n".to_string())];
    ht_lib.send_input(session_id, enter).await?;
    
    // Wait a bit for command to execute
    tokio::time::sleep(Duration::from_millis(1000)).await;
    
    // Take snapshot
    println!("Taking snapshot...");
    let snapshot = ht_lib.take_snapshot(session_id).await?;
    println!("Snapshot: {}", snapshot);
    
    // Close session
    println!("Closing session...");
    ht_lib.close_session(session_id).await?;
    println!("Session closed");
    
    Ok(())
}