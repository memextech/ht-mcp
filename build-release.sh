#!/bin/bash

# Build script for cross-platform releases
set -e

BINARY_NAME="ht-mcp"
VERSION=${1:-"latest"}

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m'

log() {
    echo -e "${GREEN}[BUILD]${NC} $1"
}

# Targets to build for
TARGETS=(
    "x86_64-unknown-linux-gnu"
    "x86_64-unknown-linux-musl"
    "aarch64-unknown-linux-gnu"
    "x86_64-apple-darwin"
    "aarch64-apple-darwin"
    "x86_64-pc-windows-msvc"
)

# Platform names for output
PLATFORMS=(
    "ht-mcp-linux-x86_64"
    "ht-mcp-linux-x86_64-musl"
    "ht-mcp-linux-aarch64"
    "ht-mcp-macos-x86_64"
    "ht-mcp-macos-aarch64"
    "ht-mcp-windows-x86_64.exe"
)

# Create release directory
RELEASE_DIR="release/$VERSION"
mkdir -p "$RELEASE_DIR"

log "Building $BINARY_NAME version $VERSION"

# Check if cross is installed
if ! command -v cross >/dev/null 2>&1; then
    log "Installing cross for cross-compilation..."
    cargo install cross
fi

# Install targets
log "Installing compilation targets..."
for target in "${TARGETS[@]}"; do
    rustup target add "$target" 2>/dev/null || true
done

# Build for each target
for i in "${!TARGETS[@]}"; do
    target="${TARGETS[$i]}"
    platform="${PLATFORMS[$i]}"
    
    log "Building for $target ($platform)..."
    
    # Use cross for cross-compilation, cargo for native
    if [[ "$target" == *"linux"* ]] && [[ "$(uname)" != "Linux" ]]; then
        cross build --release --target "$target"
    else
        cargo build --release --target "$target"
    fi
    
    # Copy binary to release directory
    if [[ "$target" == *"windows"* ]]; then
        cp "target/$target/release/$BINARY_NAME.exe" "$RELEASE_DIR/$platform"
    else
        cp "target/$target/release/$BINARY_NAME" "$RELEASE_DIR/$platform"
    fi
    
    # Make executable
    chmod +x "$RELEASE_DIR/$platform"
    
    log "✅ Built $platform"
done

# Create checksums
log "Creating checksums..."
cd "$RELEASE_DIR"
sha256sum * > checksums.txt
cd - >/dev/null

log "📦 Release binaries created in $RELEASE_DIR/"
ls -la "$RELEASE_DIR/"

# Create archive if requested
if [[ "$2" == "--archive" ]]; then
    log "Creating release archive..."
    tar -czf "release/$BINARY_NAME-$VERSION.tar.gz" -C "release" "$VERSION"
    log "📦 Archive created: release/$BINARY_NAME-$VERSION.tar.gz"
fi

log "🎉 Build complete!"
echo -e "${BLUE}Next steps:${NC}"
echo "1. Test the binaries on target platforms"
echo "2. Upload to your hosting provider"
echo "3. Update download URLs in install.sh"
echo "4. Tag the release: git tag v$VERSION && git push origin v$VERSION"