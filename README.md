# HT-MCP-Rust

A pure Rust implementation of a Model Context Protocol (MCP) server that provides headless terminal functionality.

## Overview

HT-MCP-Rust replaces the existing Node.js/TypeScript implementation with a more efficient, self-contained Rust solution that directly integrates the `ht` (headless terminal) library instead of using subprocess communication.

## Features

- **Pure Rust Implementation**: No external dependencies, single binary deployment
- **Direct HT Integration**: Library-level integration for better performance
- **MCP Protocol Compliance**: Full compatibility with MCP clients
- **Session Management**: Multiple concurrent terminal sessions
- **Web Server Support**: Optional web interface for terminal preview

## Tools Provided

- `ht_create_session`: Create new terminal sessions
- `ht_send_keys`: Send keystrokes to sessions
- `ht_take_snapshot`: Capture terminal state
- `ht_execute_command`: Execute commands and get output
- `ht_list_sessions`: List active sessions
- `ht_close_session`: Close sessions

## Project Structure

```
ht-mcp-rust/
├── src/
│   ├── mcp/                 # MCP protocol implementation
│   ├── ht_integration/      # HT library integration
│   ├── transport/           # Communication transport
│   └── error.rs             # Error handling
├── examples/                # Usage examples
└── tests/                   # Test suites
```

## Development Status

🚧 **Work in Progress** 🚧

This project is currently under development. The basic project structure has been set up, but the following components are still being implemented:

- [ ] HT library fork and integration
- [ ] MCP protocol handlers
- [ ] Session management with real HT library
- [ ] Transport layer implementation
- [ ] Comprehensive testing

## Building

```bash
cargo build
```

## Running

```bash
cargo run
```

## License

Apache 2.0

## Contributing

This project is part of the Memex headless MCP setup. Please see the main project documentation for contribution guidelines.