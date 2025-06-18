#!/bin/bash
set -euo pipefail

# Create a test scoop bucket for ht-mcp
BUCKET_NAME="ht-mcp-scoop-test"
BUCKET_REPO="memextech/$BUCKET_NAME"

echo "🪣 Creating test scoop bucket: $BUCKET_NAME"

# Check if gh cli is available
if ! command -v gh &> /dev/null; then
    echo "❌ gh CLI is required. Install it first."
    exit 1
fi

# Create repository
echo "📦 Creating repository $BUCKET_REPO..."
gh repo create "$BUCKET_REPO" \
    --description "Test scoop bucket for ht-mcp package manager testing" \
    --private

# Clone the repository
echo "📥 Cloning repository..."
gh repo clone "$BUCKET_REPO" "/tmp/$BUCKET_NAME"
cd "/tmp/$BUCKET_NAME"

# Initialize bucket structure
echo "🏗️  Setting up bucket structure..."
mkdir -p bucket

# Copy our manifest to the bucket
cp "$(dirname "$0")/../scoop/ht-mcp.json" "bucket/ht-mcp.json"

# Create README for the test bucket
cat > README.md << 'EOF'
# HT-MCP Test Scoop Bucket

This is a test bucket for the ht-mcp package.

## Usage

```powershell
# Add the test bucket
scoop bucket add ht-mcp-test https://github.com/memextech/ht-mcp-scoop-test

# Install ht-mcp from test bucket
scoop install ht-mcp-test/ht-mcp

# Remove test bucket when done
scoop bucket rm ht-mcp-test
```

## Testing

This bucket is used for testing the scoop manifest before submitting to the main scoop bucket.
EOF

# Commit and push
git add .
git commit -m "Initial bucket setup for ht-mcp testing

- Add ht-mcp.json manifest
- Add usage instructions

🤖 Generated with Memex
Co-Authored-By: Memex <noreply@memex.tech>"

git push origin main

echo "✅ Test bucket created successfully!"
echo ""
echo "🎯 Test commands:"
echo "scoop bucket add ht-mcp-test https://github.com/$BUCKET_REPO"
echo "scoop install ht-mcp-test/ht-mcp"
echo ""
echo "🧹 Cleanup when done:"
echo "scoop uninstall ht-mcp"
echo "scoop bucket rm ht-mcp-test"
echo "gh repo delete $BUCKET_REPO --confirm"