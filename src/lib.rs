//! HT-MCP: Pure Rust MCP server for headless terminal interactions
//!
//! This crate provides a Model Context Protocol (MCP) server implementation
//! for interacting with headless terminal sessions using the HT library.

#![allow(dead_code)] // Allow dead code during development
#![allow(clippy::new_without_default)] // Allow new() without Default during development
#![allow(clippy::to_string_in_format_args)] // Allow to_string in format args
#![allow(clippy::collapsible_if)] // Allow nested if statements for clarity
#![allow(clippy::collapsible_match)] // Allow nested match statements for clarity

pub mod error;
pub mod ht_integration;
pub mod mcp;
pub mod transport;

pub use error::{HtMcpError, Result};
