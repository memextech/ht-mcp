# Installation Guide

Multiple installation options are available for ht-mcp.

## 🍺 Homebrew (Recommended for macOS/Linux)

### Quick Install
```bash
# Add the tap and install
brew tap memextech/tap
brew install ht-mcp

# Verify installation
ht-mcp --version
```

### Manual Homebrew Install
```bash
# Install directly without tapping
brew install memextech/tap/ht-mcp
```

## 📦 Pre-built Binaries

Download the appropriate binary for your platform from the [latest release](https://github.com/memextech/ht-mcp/releases/latest):

### macOS
```bash
# Intel Macs
curl -L https://github.com/memextech/ht-mcp/releases/latest/download/ht-mcp-x86_64-apple-darwin -o ht-mcp
chmod +x ht-mcp
sudo mv ht-mcp /usr/local/bin/

# Apple Silicon Macs  
curl -L https://github.com/memextech/ht-mcp/releases/latest/download/ht-mcp-aarch64-apple-darwin -o ht-mcp
chmod +x ht-mcp
sudo mv ht-mcp /usr/local/bin/
```

### Linux
```bash
# Standard glibc version
curl -L https://github.com/memextech/ht-mcp/releases/latest/download/ht-mcp-x86_64-unknown-linux-gnu -o ht-mcp
chmod +x ht-mcp
sudo mv ht-mcp /usr/local/bin/

# Static musl version (works everywhere)
curl -L https://github.com/memextech/ht-mcp/releases/latest/download/ht-mcp-x86_64-unknown-linux-musl -o ht-mcp
chmod +x ht-mcp
sudo mv ht-mcp /usr/local/bin/
```

## 🦀 Cargo (Rust Package Manager)

```bash
# From crates.io (once published)
cargo install ht-mcp

# From GitHub (latest)
cargo install --git https://github.com/memextech/ht-mcp --branch main

# From specific version
cargo install --git https://github.com/memextech/ht-mcp --tag v0.1.0
```

## 🔧 Build from Source

### Prerequisites
- Rust 1.70+ (install via [rustup.rs](https://rustup.rs/))
- Git

### Build Steps
```bash
# Clone the repository
git clone https://github.com/memextech/ht-mcp.git
cd ht-mcp

# Initialize submodules
git submodule update --init --recursive

# Build release binary
cargo build --release

# Install to ~/.cargo/bin
cargo install --path .
```

## 🐳 Docker

```bash
# Run directly
docker run --rm -it ghcr.io/memextech/ht-mcp:latest

# Build locally
git clone https://github.com/memextech/ht-mcp.git
cd ht-mcp
docker build -t ht-mcp .
docker run --rm -it ht-mcp
```

## Platform Support

| Platform | Architecture | Status | Notes |
|----------|-------------|--------|-------|
| macOS | x86_64 (Intel) | ✅ Supported | Via Homebrew or binary |
| macOS | aarch64 (Apple Silicon) | ✅ Supported | Via Homebrew or binary |
| Linux | x86_64 | ✅ Supported | glibc and musl variants |
| Linux | aarch64 | ⚠️ Experimental | Cross-compiled |
| Windows | x86_64 | ❌ Not Supported | ht-core is Unix-only |

## Verification

After installation, verify everything works:

```bash
# Check version
ht-mcp --version

# Run basic help
ht-mcp --help

# Test MCP connection (should show available tools)
echo '{"jsonrpc":"2.0","method":"tools/list","id":1}' | ht-mcp
```

## Troubleshooting

### Permission Denied (macOS)
If you get a "cannot be opened because the developer cannot be verified" error:
```bash
sudo xattr -rd com.apple.quarantine /usr/local/bin/ht-mcp
```

### Library Errors (Linux)
If you get library errors, try the musl version:
```bash
curl -L https://github.com/memextech/ht-mcp/releases/latest/download/ht-mcp-x86_64-unknown-linux-musl -o ht-mcp
```

### Build Errors
Make sure you have the latest Rust toolchain:
```bash
rustup update
```

## Next Steps

- See [README.md](../README.md) for usage examples
- Check [examples/](../examples/) for configuration samples
- Read [CONTRIBUTING.md](../CONTRIBUTING.md) to contribute