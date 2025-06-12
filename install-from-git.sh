#!/bin/bash

# Install script for private Git repositories
# Usage: curl -fsSL https://your-domain.com/install-from-git.sh | sh -s -- [git-url] [token]

set -e

REPO_URL="${1:-}"
ACCESS_TOKEN="${2:-}"
BINARY_NAME="ht-mcp"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

log_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

# Check if Rust/Cargo is installed
check_rust() {
    if ! command -v cargo >/dev/null 2>&1; then
        log_error "Cargo is not installed. Please install Rust first:"
        echo "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
        exit 1
    fi
}

# Setup Git authentication
setup_git_auth() {
    if [[ -n "$ACCESS_TOKEN" ]]; then
        log_info "Setting up Git authentication..."
        
        # Configure Git to use token
        git config --global credential.helper store
        
        # Extract domain from URL
        local domain
        domain=$(echo "$REPO_URL" | sed -E 's#.*@([^:]+):.*#\1#' | sed -E 's#.*//([^/]+)/.*#\1#')
        
        # Store credentials
        echo "https://$ACCESS_TOKEN@$domain" >> ~/.git-credentials
        
        log_info "Git authentication configured"
    fi
}

# Install from Git
install_from_git() {
    log_info "Installing $BINARY_NAME from Git repository..."
    
    if [[ -z "$REPO_URL" ]]; then
        log_error "Repository URL is required"
        echo "Usage: $0 <git-url> [access-token]"
        exit 1
    fi
    
    # Install using cargo
    cargo install --git "$REPO_URL"
    
    log_info "✅ $BINARY_NAME installed successfully!"
    
    # Verify installation
    if command -v "$BINARY_NAME" >/dev/null 2>&1; then
        local version
        version=$("$BINARY_NAME" --version 2>/dev/null | head -n1 || echo "unknown")
        log_info "Installed version: $version"
    else
        log_warn "Binary not found in PATH. You may need to add ~/.cargo/bin to your PATH"
        echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
        echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.zshrc
    fi
}

# Main
main() {
    log_info "Installing $BINARY_NAME from private Git repository..."
    
    check_rust
    setup_git_auth
    install_from_git
    
    log_info "🎉 Installation complete!"
    echo "Run '$BINARY_NAME --help' to get started"
}

main "$@"