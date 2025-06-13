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

## MCP Tools Provided

This server provides 6 MCP tools for headless terminal automation:

| Tool | Description | Parameters |
|------|-------------|------------|
| `ht_create_session` | Create new terminal sessions | `session_id?`, `enable_web_server?` |
| `ht_send_keys` | Send keystrokes to sessions | `session_id`, `keys[]` |
| `ht_take_snapshot` | Capture terminal state | `session_id` |
| `ht_execute_command` | Execute commands and get output | `session_id`, `command` |
| `ht_list_sessions` | List active sessions | None |
| `ht_close_session` | Close sessions | `session_id` |

### Example Usage

Once configured in your MCP client, you can:

1. **Create a session**: `ht_create_session` 
2. **Run commands**: `ht_execute_command` with `session_id` and `command: "ls -la"`
3. **Send interactive input**: `ht_send_keys` with `keys: ["y", "Enter"]`
4. **Capture state**: `ht_take_snapshot` to see current terminal output
5. **Clean up**: `ht_close_session` when done

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
cargo install --git https://github.com/memextech/ht-mcp ht-mcp
```

### From Crates.io (Future)

Once the official MCP SDK publishes to crates.io:

```bash
cargo install ht-mcp
```

## Usage

### Quick Install & Run

**Option 1: One-liner**
```bash
cargo install --git https://github.com/memextech/ht-mcp ht-mcp && ht-mcp
```

**Option 2: Install script**
```bash
curl -sSL https://raw.githubusercontent.com/memextech/ht-mcp/main/install-and-run.sh | bash
```

### Manual Usage

Start the MCP server:

```bash
ht-mcp
```

With debug logging:

```bash
ht-mcp --debug
```

### MCP Client Configuration

After installation, add to your MCP client configuration:

#### Standard Configuration

Add to your MCP client config:

```json
{
  "mcpServers": {
    "ht-mcp": {
      "command": "ht-mcp",
      "args": ["--debug"]
    }
  }
}
```

#### Alternative with Full Path

If the binary isn't in your PATH, use the full path:

```json
{
  "mcpServers": {
    "ht-mcp": {
      "command": "/Users/yourusername/.cargo/bin/ht-mcp",
      "args": ["--debug"]
    }
  }
}
```

#### Finding Your Installation Path

```bash
which ht-mcp
# or
ls ~/.cargo/bin/ht-mcp
```

## Troubleshooting

### Installation Issues

- **Rust not installed**: Install via [rustup.rs](https://rustup.rs/)
- **Git submodule errors**: Ensure good internet connection, retry installation
- **Permission errors**: Check `~/.cargo/bin` is in your PATH

### Runtime Issues

- **"Command not found"**: Add `~/.cargo/bin` to your PATH:
  ```bash
  export PATH="$HOME/.cargo/bin:$PATH"
  ```
- **MCP connection issues**: Verify the binary path in your MCP client config
- **Debug mode**: Use `--debug` flag for verbose logging

### System Requirements

- **Platform**: Linux or macOS (Windows not supported)
- **Rust**: 1.70+ (automatically handled by cargo install)
- **Memory**: Minimal, each terminal session uses ~1-2MB

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