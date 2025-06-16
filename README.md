# ht-mcp

[![Rust](https://img.shields.io/badge/rust-1.75+-orange.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

A high-performance Rust implementation of a Model Context Protocol (MCP) server for headless terminal [ht](https://github.com/andyk/ht).

## Features

- 🚀 **Pure Rust**: Single binary, no external dependencies
- 🔗 **Direct Integration**: Embedded [ht](https://github.com/andyk/ht) library for optimal performance  
- 📡 **MCP Compliant**: Full compatibility with MCP clients
- 🖥️ **Multi-Session**: Concurrent terminal session management
- 🌐 **Web Interface**: Optional live terminal preview
- 💨 **Fast**: 40x faster startup than Node.js equivalent

## Installation

### From Git (Recommended)

```bash
cargo install --git https://github.com/memextech/ht-mcp ht-mcp
```

### From Source

```bash
git clone https://github.com/memextech/ht-mcp.git
cd ht-mcp
cargo install --path .
```

## MCP Tools

| Tool | Description | Parameters |
|------|-------------|------------|
| `ht_create_session` | Create new terminal session | `command?`, `enableWebServer?` |
| `ht_send_keys` | Send keystrokes to session | `sessionId`, `keys[]` |
| `ht_take_snapshot` | Capture terminal state | `sessionId` |
| `ht_execute_command` | Execute command and get output | `sessionId`, `command` |
| `ht_list_sessions` | List all active sessions | None |
| `ht_close_session` | Close terminal session | `sessionId` |

> **Note**: Parameters use camelCase (e.g., `sessionId`, `enableWebServer`) for MCP compatibility.

## Configuration

Add to your MCP client configuration:

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

For custom installation paths:

```json
{
  "mcpServers": {
    "ht-mcp": {
      "command": "/path/to/ht-mcp",
      "args": []
    }
  }
}
```

## Usage Example

```bash
# Start the MCP server
ht-mcp

# With debug logging
ht-mcp --debug
```

Once configured in your MCP client:

1. **Create session**: `ht_create_session` → Returns session ID
2. **Run commands**: `ht_execute_command` with session ID and command
3. **Interactive input**: `ht_send_keys` for multi-step interactions
4. **Check state**: `ht_take_snapshot` to see current terminal
5. **Clean up**: `ht_close_session` when finished

## Response Format

This server returns **human-readable text responses** (not JSON), designed for natural language interaction:

```text
# Create session response
HT session created successfully!

Session ID: abc123-def456-789...

🌐 Web server enabled! View live terminal at: http://127.0.0.1:3000

# Terminal snapshot response
Terminal Snapshot (Session: abc123...)

```
bash-3.2$ ls -la
total 16
drwxr-xr-x  4 user staff  128 Jun 13 10:30 .
-rw-r--r--  1 user staff   45 Jun 13 10:30 file.txt
bash-3.2$ 
```
```

## Requirements

- **Rust**: 1.75+ (install via [rustup](https://rustup.rs/))
- **Unix-like OS**: Linux, macOS (Windows not supported)
- **Git**: For submodule handling during build

## Development

```bash
# Clone with submodules
git clone --recursive https://github.com/memextech/ht-mcp.git
cd ht-mcp

# Build
cargo build

# Run
cargo run

# Test
cargo test
```

## Troubleshooting

**Installation Issues:**
- Ensure Rust 1.75+ is installed
- Check internet connection for git submodules
- Verify `~/.cargo/bin` is in PATH

**Runtime Issues:**
- Use `ht-mcp --debug` for verbose logging
- Check MCP client configuration syntax
- Verify binary path: `which ht-mcp`

## Performance

Compared to the original TypeScript implementation:
- **40x faster startup** (~50ms vs ~2s)
- **70% less memory** (~15MB vs ~50MB)
- **Single binary** (4.7MB vs ~200MB Node.js)
- **Zero subprocess overhead**

## License

MIT License

Copyright (c) 2025 Atlas Futures Inc.

See [LICENSE](LICENSE) for details.

## Contributing

Contributions welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

---

Built with ❤️ by [Memex](https://memex.tech)
