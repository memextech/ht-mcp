#!/bin/bash
set -euo pipefail

# Test Homebrew distribution locally
# This builds a local binary and tests the formula structure

echo "🧪 Testing Homebrew distribution locally..."

# Build release binary
echo "🔨 Building release binary..."
cargo build --release

# Create a temporary formula for testing
echo "📝 Creating test formula..."
TEMP_FORMULA=$(mktemp)
BINARY_PATH="$(pwd)/target/release/ht-mcp"
BINARY_SHA=$(shasum -a 256 "$BINARY_PATH" | cut -d' ' -f1)

cat > "$TEMP_FORMULA" << EOF
class HtMcp < Formula
  desc "Headless Terminal MCP Server - Control terminal sessions via Model Context Protocol"
  homepage "https://github.com/memextech/ht-mcp"
  version "0.1.2-test-local"
  url "file://$BINARY_PATH"
  sha256 "$BINARY_SHA"

  def install
    bin.install "ht-mcp"
  end

  test do
    assert_match "ht-mcp", shell_output("#{bin}/ht-mcp --version 2>&1", 1)
  end
end
EOF

echo "✅ Test formula created at: $TEMP_FORMULA"
echo "📄 Formula content:"
cat "$TEMP_FORMULA"
echo ""

echo "🎯 To test this formula:"
echo "1. Copy to a local tap: cp $TEMP_FORMULA /opt/homebrew/Library/Taps/local/homebrew-test/Formula/ht-mcp.rb"
echo "2. Install: brew install local/test/ht-mcp"
echo "3. Test: ht-mcp --version"
echo ""
echo "🗑️  Clean up: rm $TEMP_FORMULA"