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

## Installation

### From Git (Recommended)

Install directly from the public repository:

```bash
cargo install --git https://github.com/memextech/ht-mcp --branch feature/oss-setup ht-mcp
```

### From Crates.io (Future)

Once the official MCP SDK publishes to crates.io:

```bash
cargo install ht-mcp
```

## Usage

Start the MCP server:

```bash
ht-mcp
```

With debug logging:

```bash
ht-mcp --debug
```

## Development Status

✅ **Production Ready** 

This project is feature-complete and production-ready:

- ✅ HT library integration via embedded ht-core
- ✅ Full MCP protocol implementation (6 tools)
- ✅ Session management with real HT library
- ✅ stdio transport layer
- ✅ Comprehensive CI/CD pipeline
- ✅ Cross-platform support (Linux/macOS)

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