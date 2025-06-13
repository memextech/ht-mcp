// Simple test to verify HT library works directly

#![allow(dead_code)] // Allow dead code during development
#![allow(clippy::new_without_default)] // Allow new() without Default during development
#![allow(clippy::to_string_in_format_args)] // Allow to_string in format args
#![allow(clippy::collapsible_if)] // Allow nested if statements for clarity
#![allow(clippy::collapsible_match)] // Allow nested match statements for clarity
use ht_core::{HtLibrary, InputSeq, SessionConfig};
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
