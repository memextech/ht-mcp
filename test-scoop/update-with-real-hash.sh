#!/bin/bash
set -euo pipefail

# Update the scoop manifest with a real hash from the latest release
echo "🔍 Fetching latest release info..."

# Get the latest release info
LATEST_RELEASE=$(gh release view --json tagName,assets)
VERSION=$(echo "$LATEST_RELEASE" | jq -r '.tagName')
WINDOWS_ASSET=$(echo "$LATEST_RELEASE" | jq -r '.assets[] | select(.name | endswith("windows-msvc.exe")) | .downloadUrl')

if [[ "$WINDOWS_ASSET" == "null" || -z "$WINDOWS_ASSET" ]]; then
    echo "❌ No Windows binary found in latest release"
    echo "Available assets:"
    echo "$LATEST_RELEASE" | jq -r '.assets[].name'
    exit 1
fi

echo "📦 Found Windows binary: $WINDOWS_ASSET"

# Download the checksum file
CHECKSUM_URL="${WINDOWS_ASSET}.sha256"
echo "📥 Downloading checksum from: $CHECKSUM_URL"

HASH=$(curl -sL "$CHECKSUM_URL" | cut -d' ' -f1)

if [[ -z "$HASH" ]]; then
    echo "❌ Failed to retrieve hash"
    exit 1
fi

echo "🔐 Hash: $HASH"

# Update the scoop manifest
echo "📝 Updating scoop manifest..."
sed -i.bak "s/\$HASH_PLACEHOLDER/$HASH/" scoop/ht-mcp.json
sed -i.bak "s/v0\.1\.0/$VERSION/" scoop/ht-mcp.json

# Clean up backup
rm -f scoop/ht-mcp.json.bak

echo "✅ Scoop manifest updated with real values:"
echo "   Version: $VERSION"
echo "   Hash: $HASH"
echo ""
echo "🎯 Ready for testing:"
echo "   scoop install ./scoop/ht-mcp.json"