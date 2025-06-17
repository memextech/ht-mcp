#!/bin/bash
set -euo pipefail

echo "🧪 Testing Homebrew formula manually with local binary..."

# Create a temporary directory for our test tap
TEST_TAP_DIR="/tmp/homebrew-test-tap"
rm -rf "$TEST_TAP_DIR"
mkdir -p "$TEST_TAP_DIR/Formula"

# Build the binary if not already built
if [ ! -f "target/release/ht-mcp" ]; then
    echo "🔨 Building release binary..."
    cargo build --release
fi

# Create a local file URL for the binary
BINARY_PATH="$(pwd)/target/release/ht-mcp"
BINARY_SHA=$(shasum -a 256 "$BINARY_PATH" | cut -d' ' -f1)

echo "📝 Creating test formula with local binary..."
cat > "$TEST_TAP_DIR/Formula/ht-mcp.rb" << EOF
class HtMcp < Formula
  desc "Headless Terminal MCP Server - Control terminal sessions via Model Context Protocol"
  homepage "https://github.com/memextech/ht-mcp"
  version "0.1.3-test-local"
  url "file://$BINARY_PATH"
  sha256 "$BINARY_SHA"

  def install
    bin.install "ht-mcp"
  end

  test do
    # Test that the binary exists and shows help
    assert_match "Pure Rust MCP server", shell_output("#{bin}/ht-mcp --help 2>&1")
  end
end
EOF

echo "✅ Formula created!"
echo "📄 Formula content:"
cat "$TEST_TAP_DIR/Formula/ht-mcp.rb"
echo ""

# Check if we can add this tap
echo "🔧 Adding temporary tap..."
if brew tap-new test/local || true; then
    echo "Tap created or already exists"
fi

# Copy our formula to the tap
LOCAL_TAP_DIR="$(brew --repository)/Library/Taps/test/homebrew-local"
if [ -d "$LOCAL_TAP_DIR" ]; then
    cp "$TEST_TAP_DIR/Formula/ht-mcp.rb" "$LOCAL_TAP_DIR/Formula/"
    echo "✅ Formula copied to local tap"
    
    echo "🧪 Testing formula installation..."
    # Uninstall if already installed
    brew uninstall test/local/ht-mcp 2>/dev/null || true
    
    # Install from our local tap
    if brew install test/local/ht-mcp; then
        echo "✅ Installation successful!"
        
        echo "🧪 Testing installed binary..."
        if ht-mcp --help | grep -q "Pure Rust MCP server"; then
            echo "✅ Binary test successful!"
        else
            echo "❌ Binary test failed"
            exit 1
        fi
        
        echo "🧪 Running brew test..."
        if brew test test/local/ht-mcp; then
            echo "✅ Brew test successful!"
        else
            echo "❌ Brew test failed"
            exit 1
        fi
        
        echo ""
        echo "🎉 ALL TESTS PASSED!"
        echo "✅ Homebrew formula works correctly"
        echo ""
        echo "Clean up with: brew uninstall test/local/ht-mcp && brew untap test/local"
    else
        echo "❌ Installation failed"
        exit 1
    fi
else
    echo "❌ Could not create local tap directory"
    exit 1
fi