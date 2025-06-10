#!/bin/bash

echo "Starting manual test of HT MCP server..."

# Start the server in the background
./target/debug/ht-mcp-rust &
SERVER_PID=$!

# Give the server time to start
sleep 1

echo "Server started with PID: $SERVER_PID"

# Send initialize request
echo '{"jsonrpc": "2.0", "id": 1, "method": "initialize", "params": {"protocolVersion": "2024-11-05", "capabilities": {}, "clientInfo": {"name": "test-client", "version": "1.0.0"}}}' | ./target/debug/ht-mcp-rust

# Clean up
kill $SERVER_PID 2>/dev/null || true
wait $SERVER_PID 2>/dev/null || true

echo "Test completed."