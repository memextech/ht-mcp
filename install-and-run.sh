#!/bin/bash
set -e

echo "🚀 Installing ht-mcp from GitHub..."
cargo install --git https://github.com/memextech/ht-mcp ht-mcp

echo "✅ Installation complete!"
echo "📍 Binary location: $(which ht-mcp)"
echo ""
echo "🔧 To add to MCP client config:"
echo '  "ht-mcp": {'
echo '    "command": "ht-mcp",'
echo '    "args": ["--debug"]'
echo '  }'
echo ""
echo "🎯 Starting ht-mcp server..."
exec ht-mcp "$@"