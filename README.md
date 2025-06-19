# ht-mcp

[![Rust](https://img.shields.io/badge/rust-1.75+-orange.svg)](https://www.rust-lang.org)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

A high-performance Rust implementation of a Model Context Protocol (MCP) server for headless terminal [ht](https://github.com/andyk/ht).

## Features

- 🚀 **Pure Rust**: Single binary MCP server, no external dependencies
- 🔗 **Direct Integration**: Embed excellent [ht](https://github.com/andyk/ht) headless terminal library for optimal performance
- 🖥️ **Multi-Session**: Concurrent terminal session management
- 🌐 **Web Interface**: Optional live terminal preview

## Demo

### ht-mcp in [Memex](https://memex.tech)

![ht-mcp in Memex](https://github.com/user-attachments/assets/6a1b6e76-5d5c-4ba4-87ee-70a31f0bc4ce)

### ht-mcp in [Claude Code](https://www.anthropic.com/claude-code)

![ht-mcp in Claude Code](https://github.com/user-attachments/assets/e70a3240-77f5-4ef2-953b-202b310dbf74)

## Installation

### 🍺 Homebrew (Recommended)

```bash
brew tap memextech/tap
brew install ht-mcp
```

### 📦 Pre-built Binaries

Download from [releases](https://github.com/memextech/ht-mcp/releases/latest):

```bash
# macOS Intel
curl -L https://github.com/memextech/ht-mcp/releases/latest/download/ht-mcp-x86_64-apple-darwin -o ht-mcp

# macOS Apple Silicon
curl -L https://github.com/memextech/ht-mcp/releases/latest/download/ht-mcp-aarch64-apple-darwin -o ht-mcp

# Linux
curl -L https://github.com/memextech/ht-mcp/releases/latest/download/ht-mcp-x86_64-unknown-linux-gnu -o ht-mcp

# Windows (PowerShell)
curl.exe -L https://github.com/memextech/ht-mcp/releases/latest/download/ht-mcp-x86_64-pc-windows-msvc -o ht-mcp.exe

# Make executable and install
chmod +x ht-mcp && sudo mv ht-mcp /usr/local/bin/
```

### 🦀 Cargo

```bash
# From crates.io (stable)
cargo install ht-mcp

# From git (latest)
cargo install --git https://github.com/memextech/ht-mcp
```

### 🔧 Build from Source

```bash
git clone https://github.com/memextech/ht-mcp.git
cd ht-mcp
git submodule update --init --recursive
cargo install --path .
```

See [docs/INSTALLATION.md](docs/INSTALLATION.md) for detailed installation options.

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

🌐 Web server enabled! View live terminal at: http://127.0.0.1:3618
```

```text
# Terminal snapshot response
Terminal Snapshot (Session: abc123...)

bash-3.2$ ls -la
total 16
drwxr-xr-x  4 user staff  128 Jun 13 10:30 .
-rw-r--r--  1 user staff   45 Jun 13 10:30 file.txt
bash-3.2$
```

## Requirements

- **Rust**: 1.75+ (install via [rustup](https://rustup.rs/))
- **Supported OS**: Linux, macOS, Windows (experimental)

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

Compared to the original [TypeScript implementation](https://github.com/memextech/headless-terminal-mcp):
- **40x faster startup** (~50ms vs ~2s)
- **70% less memory** (~15MB vs ~50MB)
- **Single binary** (4.7MB vs ~200MB Node.js)
- **Zero subprocess overhead**

## License

Apache 2.0 License

Copyright (c) 2025 Atlas Futures Inc.

See [LICENSE](LICENSE) for details.

## Contributing

Contributions welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

---

Built with [Memex](https://memex.tech)✨
# Fixed submodule commit reference
