# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

ht-mcp is a high-performance Rust MCP (Model Context Protocol) server that provides headless terminal interactions. It embeds the HT library for optimal performance and offers both direct terminal control and optional web interface for live previews.

## Key Architecture

### Core Components
- **MCP Server** (`src/mcp/`): JSON-RPC 2.0 protocol implementation with 6 tools for terminal management
- **HT Integration** (`src/ht_integration/`): Session management, command execution, and web server integration  
- **Transport Layer** (`src/transport/`): STDIO-based MCP communication
- **HT Core Library** (`ht-core/`): Embedded headless terminal library (git submodule)

### Tool Set
The server provides 7 MCP tools with camelCase parameters:
- `ht_create_session` - Create new terminal session (optionally with web server)
- `ht_send_keys` - Send keystrokes to session
- `ht_take_snapshot` - Capture current terminal state  
- `ht_execute_command` - Execute command and return output
- `ht_list_sessions` - List all active sessions
- `ht_close_session` - Close terminal session
- `ht_resize` - Resize terminal window dimensions (cols, rows)

### Response Format
All tool responses are human-readable text (not JSON), designed for natural language interaction with clear formatting and helpful context.

## Development Commands

### Building
```bash
# Standard build
cargo build

# Release build (optimized, stripped)
cargo build --release

# Build with submodules (required for fresh clone)
git submodule update --init --recursive
cargo build
```

### Testing
```bash
# Run all tests
cargo test

# Run specific test categories
cargo test --test integration_mcp_protocol
cargo test --test integration_terminal_functionality
cargo test --lib  # unit tests only

# Run with debug output
RUST_LOG=debug cargo test -- --nocapture

# Run examples
cargo run --example mcp_client_usage
```

### Development Tools
```bash
# Run server locally
cargo run

# Run with debug logging
cargo run -- --debug

# Manual protocol testing
./manual_test.sh

# Check formatting (if rustfmt is configured)
cargo fmt --check

# Lint (if clippy is configured)
cargo clippy
```

## Important File Locations

### Core Implementation
- `src/main.rs` - MCP server entry point and JSON-RPC handling
- `src/mcp/server.rs` - Main MCP server implementation
- `src/mcp/tools.rs` - Tool definitions and schemas
- `src/ht_integration/session_manager.rs` - Terminal session lifecycle

### Configuration & Build
- `Cargo.toml` - Main package configuration (workspace root)
- `ht-core/Cargo.toml` - Embedded HT library configuration
- `build-release.sh` - Cross-platform release build script

### Documentation
- `README.md` - Installation, usage, and configuration guide
- `tests/README.md` - Comprehensive testing documentation
- `docs/INSTALLATION.md` - Detailed installation options

## Key Dependencies

- **rmcp**: Official MCP Rust SDK for protocol implementation
- **ht-core**: Embedded headless terminal library (local path dependency)
- **tokio**: Async runtime for MCP server operations
- **axum**: Web server for optional live terminal preview
- **serde_json**: JSON handling for MCP protocol and tool responses

## Development Notes

### Submodule Management
The ht-core library is included as a git submodule. Always run `git submodule update --init --recursive` after cloning or when submodule updates are needed.

### Platform Support
- Primary: macOS, Linux
- Experimental: Windows
- CI testing configured for Unix platforms

### Response Formatting
Tool responses use `format_tool_response()` in `src/main.rs:241` to convert JSON results into human-readable text matching the TypeScript implementation's format.

### Web Server Integration
Optional web server provides live terminal preview at random ports. Enable with `enableWebServer: true` parameter in `ht_create_session`.