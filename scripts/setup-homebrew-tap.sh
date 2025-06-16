#!/bin/bash
set -euo pipefail

# Setup Homebrew Tap for ht-mcp
# This script creates a homebrew tap repository and sets up the formula

REPO_NAME="homebrew-tap"
ORG="memextech"
FORMULA_NAME="ht-mcp"

echo "🍺 Setting up Homebrew tap for $FORMULA_NAME..."

# Get the script directory and project root before changing directories
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

# Check if gh CLI is installed
if ! command -v gh &> /dev/null; then
    echo "❌ GitHub CLI (gh) is required. Install with: brew install gh"
    exit 1
fi

# Check if we're authenticated with GitHub
if ! gh auth status &> /dev/null; then
    echo "❌ Please authenticate with GitHub: gh auth login"
    exit 1
fi

# Create temporary directory for tap setup
TEMP_DIR=$(mktemp -d)
cd "$TEMP_DIR"

echo "📁 Creating tap repository structure..."

# Create tap repository structure
mkdir -p Formula
cp "$PROJECT_ROOT/homebrew/$FORMULA_NAME.rb" "Formula/$FORMULA_NAME.rb"

# Create README for tap
cat > README.md << EOF
# Memex Tech Homebrew Tap

This is the official Homebrew tap for Memex Tech tools.

## Usage

\`\`\`bash
# Add the tap
brew tap $ORG/tap

# Install tools
brew install $FORMULA_NAME
\`\`\`

## Available Formulas

- **$FORMULA_NAME**: Headless Terminal MCP Server - Control terminal sessions via Model Context Protocol

## Manual Installation

If you prefer not to use the tap:

\`\`\`bash
brew install $ORG/tap/$FORMULA_NAME
\`\`\`
EOF

# Create git repository
git init
git add .
git commit -m "Initial homebrew tap setup

🤖 Generated with [Memex](https://memex.tech)
Co-Authored-By: Memex <noreply@memex.tech>"

echo "🚀 Creating GitHub repository..."

# Create repository on GitHub
gh repo create "$ORG/$REPO_NAME" \
    --public \
    --description "Homebrew tap for Memex Tech tools" \
    --clone=false

# Add remote and push
git remote add origin "git@github.com:$ORG/$REPO_NAME.git"
git branch -M main
git push -u origin main

echo "✅ Homebrew tap created successfully!"
echo ""
echo "🎯 Next steps:"
echo "1. Update Formula/$FORMULA_NAME.rb with actual SHA256 checksums from releases"
echo "2. Test the formula: brew install $ORG/tap/$FORMULA_NAME"
echo "3. Users can now install with: brew tap $ORG/tap && brew install $FORMULA_NAME"
echo ""
echo "📍 Repository: https://github.com/$ORG/$REPO_NAME"

# Cleanup
cd /
rm -rf "$TEMP_DIR"