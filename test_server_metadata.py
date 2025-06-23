#!/usr/bin/env python3
"""
Test script to verify the ht-mcp server properly reports its metadata during initialization.
This demonstrates that the server follows the MCP specification for serverInfo reporting.
"""

import json
import subprocess
import sys
import time
from pathlib import Path

def test_server_metadata():
    """Test that the server reports correct metadata during initialization."""
    
    # Build the release binary first
    print("Building ht-mcp server...")
    build_result = subprocess.run(
        ["cargo", "build", "--release"],
        cwd=Path(__file__).parent,
        capture_output=True,
        text=True
    )
    
    if build_result.returncode != 0:
        print("❌ Failed to build server:")
        print(build_result.stderr)
        return False
    
    # Start the server
    print("Starting ht-mcp server...")
    server_path = Path(__file__).parent / "target" / "release" / "ht-mcp"
    
    process = subprocess.Popen(
        [str(server_path)],
        stdin=subprocess.PIPE,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True
    )
    
    try:
        # Send initialize request
        init_request = {
            "jsonrpc": "2.0",
            "id": 1,
            "method": "initialize",
            "params": {
                "protocolVersion": "2024-11-05",
                "capabilities": {},
                "clientInfo": {
                    "name": "test-client",
                    "version": "1.0.0"
                }
            }
        }
        
        print("Sending initialize request...")
        request_str = json.dumps(init_request) + "\n"
        process.stdin.write(request_str)
        process.stdin.flush()
        
        # Read response
        response_line = process.stdout.readline()
        if not response_line:
            print("❌ No response received from server")
            return False
            
        print(f"Received response: {response_line.strip()}")
        
        try:
            response = json.loads(response_line.strip())
        except json.JSONDecodeError as e:
            print(f"❌ Failed to parse JSON response: {e}")
            return False
        
        # Validate response structure
        if response.get("jsonrpc") != "2.0":
            print(f"❌ Invalid JSON-RPC version: {response.get('jsonrpc')}")
            return False
        
        if response.get("id") != 1:
            print(f"❌ Invalid response ID: {response.get('id')}")
            return False
        
        if "result" not in response:
            print(f"❌ No result in response: {response}")
            return False
        
        result = response["result"]
        
        # Validate server metadata
        if "serverInfo" not in result:
            print(f"❌ No serverInfo in response: {result}")
            return False
        
        server_info = result["serverInfo"]
        
        # Check required fields
        expected_name = "ht-mcp"
        expected_title = "Headless Terminal MCP Server"
        
        if server_info.get("name") != expected_name:
            print(f"❌ Wrong server name: expected '{expected_name}', got '{server_info.get('name')}'")
            return False
        
        if server_info.get("title") != expected_title:
            print(f"❌ Wrong server title: expected '{expected_title}', got '{server_info.get('title')}'")
            return False
        
        if not server_info.get("version"):
            print(f"❌ Missing server version: {server_info}")
            return False
        
        # Validate other required fields
        if result.get("protocolVersion") != "2024-11-05":
            print(f"❌ Wrong protocol version: {result.get('protocolVersion')}")
            return False
        
        if "capabilities" not in result:
            print(f"❌ Missing capabilities: {result}")
            return False
        
        # Success!
        print("✅ Server metadata validation successful!")
        print(f"   Name: {server_info['name']}")
        print(f"   Title: {server_info['title']}")
        print(f"   Version: {server_info['version']}")
        print(f"   Protocol Version: {result['protocolVersion']}")
        
        return True
        
    finally:
        # Clean up
        process.terminate()
        try:
            process.wait(timeout=5)
        except subprocess.TimeoutExpired:
            process.kill()

if __name__ == "__main__":
    print("🧪 Testing ht-mcp server metadata reporting...")
    success = test_server_metadata()
    
    if success:
        print("\n🎉 All tests passed! Server properly reports metadata per MCP specification.")
        sys.exit(0)
    else:
        print("\n💥 Tests failed!")
        sys.exit(1)