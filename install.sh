#!/bin/bash

# Install script for ht-mcp
# Usage: curl -fsSL https://install.ht-mcp.dev/install.sh | sh

set -e

# Configuration
BINARY_NAME="ht-mcp"
GITHUB_REPO="memextech/ht-mcp"
BASE_URL="https://github.com/memextech/ht-mcp/releases"
INSTALL_DIR="/usr/local/bin"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Logging functions
log_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Detect platform
detect_platform() {
    local os="$(uname -s)"
    local arch="$(uname -m)"
    
    case "$os" in
        Linux*)
            case "$arch" in
                x86_64) echo "linux-x86_64" ;;
                arm64|aarch64) echo "linux-aarch64" ;;
                *) log_error "Unsupported architecture: $arch"; exit 1 ;;
            esac
            ;;
        Darwin*)
            case "$arch" in
                x86_64) echo "macos-x86_64" ;;
                arm64|aarch64) echo "macos-aarch64" ;;
                *) log_error "Unsupported architecture: $arch"; exit 1 ;;
            esac
            ;;
        CYGWIN*|MINGW*|MSYS*)
            echo "windows-x86_64.exe"
            ;;
        *)
            log_error "Unsupported OS: $os"
            exit 1
            ;;
    esac
}

# Check if binary exists
check_binary_exists() {
    if command -v "$BINARY_NAME" >/dev/null 2>&1; then
        local current_version
        current_version=$("$BINARY_NAME" --version 2>/dev/null | head -n1 || echo "unknown")
        log_warn "$BINARY_NAME is already installed: $current_version"
        echo -n "Do you want to overwrite it? [y/N] "
        read -r response
        case "$response" in
            [yY][eE][sS]|[yY]) 
                log_info "Proceeding with installation..."
                ;;
            *)
                log_info "Installation cancelled."
                exit 0
                ;;
        esac
    fi
}

# Download and install
install_binary() {
    local platform="$1"
    local tmp_dir
    tmp_dir=$(mktemp -d)
    local binary_url="$BASE_URL/$BINARY_NAME/latest/$platform"
    local tmp_file="$tmp_dir/$BINARY_NAME"
    
    log_info "Downloading $BINARY_NAME for $platform..."
    
    # Download binary
    if command -v curl >/dev/null 2>&1; then
        curl -fsSL "$binary_url" -o "$tmp_file"
    elif command -v wget >/dev/null 2>&1; then
        wget -q "$binary_url" -O "$tmp_file"
    else
        log_error "Neither curl nor wget found. Please install one of them."
        exit 1
    fi
    
    # Make executable
    chmod +x "$tmp_file"
    
    # Install to system
    if [ -w "$INSTALL_DIR" ]; then
        mv "$tmp_file" "$INSTALL_DIR/$BINARY_NAME"
    else
        log_info "Installing to $INSTALL_DIR (requires sudo)..."
        sudo mv "$tmp_file" "$INSTALL_DIR/$BINARY_NAME"
    fi
    
    # Cleanup
    rm -rf "$tmp_dir"
    
    log_info "✅ $BINARY_NAME installed successfully!"
    log_info "Run '$BINARY_NAME --help' to get started."
}

# Verify installation
verify_installation() {
    if command -v "$BINARY_NAME" >/dev/null 2>&1; then
        local version
        version=$("$BINARY_NAME" --version 2>/dev/null | head -n1 || echo "unknown")
        log_info "Installed version: $version"
    else
        log_error "Installation failed: $BINARY_NAME not found in PATH"
        exit 1
    fi
}

# Main installation flow
main() {
    log_info "Installing $BINARY_NAME..."
    
    # Check for existing installation
    check_binary_exists
    
    # Detect platform
    local platform
    platform=$(detect_platform)
    log_info "Detected platform: $platform"
    
    # Install
    install_binary "$platform"
    
    # Verify
    verify_installation
}

# Run main function
main "$@"