#!/bin/bash
set -euo pipefail

# Update Homebrew formula with release checksums
# Usage: ./update-homebrew-formula.sh <version>

if [ $# -eq 0 ]; then
    echo "❌ Usage: $0 <version>"
    echo "   Example: $0 v0.1.0"
    exit 1
fi

VERSION="$1"
# Remove 'v' prefix if present
VERSION_NO_V="${VERSION#v}"

ORG="memextech"
REPO="ht-mcp"
TAP_REPO="homebrew-tap"
FORMULA_NAME="ht-mcp"

echo "🔄 Updating Homebrew formula for $FORMULA_NAME $VERSION..."

# Check if gh CLI is installed
if ! command -v gh &> /dev/null; then
    echo "❌ GitHub CLI (gh) is required. Install with: brew install gh"
    exit 1
fi

# Function to get SHA256 from GitHub release
get_sha256() {
    local asset_name="$1"
    local sha_url="https://github.com/$ORG/$REPO/releases/download/$VERSION/$asset_name.sha256"
    
    echo "📥 Fetching SHA256 for $asset_name..."
    curl -sL "$sha_url" | cut -d' ' -f1
}

# Get checksums for all platforms
echo "🔍 Fetching release checksums..."

ARM64_SHA=$(get_sha256 "ht-mcp-aarch64-apple-darwin")
X86_64_SHA=$(get_sha256 "ht-mcp-x86_64-apple-darwin") 
LINUX_SHA=$(get_sha256 "ht-mcp-x86_64-unknown-linux-gnu")

echo "✅ Checksums retrieved:"
echo "  ARM64 (Apple Silicon): $ARM64_SHA"
echo "  x86_64 (Intel Mac):   $X86_64_SHA"
echo "  Linux x86_64:         $LINUX_SHA"

# Create temporary directory and clone tap
TEMP_DIR=$(mktemp -d)
cd "$TEMP_DIR"

echo "📂 Cloning tap repository..."
gh repo clone "$ORG/$TAP_REPO"
cd "$TAP_REPO"

# Update the formula
echo "✏️  Updating formula..."

cat > "Formula/$FORMULA_NAME.rb" << EOF
class HtMcp < Formula
  desc "Headless Terminal MCP Server - Control terminal sessions via Model Context Protocol"
  homepage "https://github.com/$ORG/$REPO"
  version "$VERSION_NO_V"

  if OS.mac?
    if Hardware::CPU.arm?
      url "https://github.com/$ORG/$REPO/releases/download/$VERSION/ht-mcp-aarch64-apple-darwin"
      sha256 "$ARM64_SHA"
    else
      url "https://github.com/$ORG/$REPO/releases/download/$VERSION/ht-mcp-x86_64-apple-darwin"
      sha256 "$X86_64_SHA"
    end
  else
    url "https://github.com/$ORG/$REPO/releases/download/$VERSION/ht-mcp-x86_64-unknown-linux-gnu"
    sha256 "$LINUX_SHA"
  end

  def install
    bin.install Dir["*"].first => "ht-mcp"
  end

  test do
    # Test that the binary exists and shows version/help
    output = shell_output("#{bin}/ht-mcp --version 2>&1", 1)
    assert_match "ht-mcp", output
  end
end
EOF

# Commit and push changes
git add "Formula/$FORMULA_NAME.rb"
git commit -m "Update $FORMULA_NAME to $VERSION

- ARM64 (Apple Silicon): $ARM64_SHA
- x86_64 (Intel Mac): $X86_64_SHA  
- Linux x86_64: $LINUX_SHA

🤖 Generated with [Memex](https://memex.tech)
Co-Authored-By: Memex <noreply@memex.tech>"

git push origin main

echo "✅ Formula updated successfully!"
echo ""
echo "🎯 Next steps:"
echo "1. Test the updated formula:"
echo "   brew uninstall $FORMULA_NAME 2>/dev/null || true"
echo "   brew untap $ORG/tap 2>/dev/null || true"
echo "   brew tap $ORG/tap"
echo "   brew install $FORMULA_NAME"
echo ""
echo "2. Verify installation:"
echo "   ht-mcp --version"
echo ""
echo "📍 Updated formula: https://github.com/$ORG/$TAP_REPO/blob/main/Formula/$FORMULA_NAME.rb"

# Cleanup
cd /
rm -rf "$TEMP_DIR"