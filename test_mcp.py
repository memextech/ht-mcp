#!/usr/bin/env python3
"""
Simple test script to verify the HT MCP server works.
This sends MCP protocol messages to test basic functionality.
"""

import json
import subprocess
import time
import sys

def send_mcp_message(process, message):
    """Send an MCP message to the server process."""
    json_msg = json.dumps(message)
    print(f"→ Sending: {json_msg}")
    process.stdin.write(json_msg + "\n")
    process.stdin.flush()

def read_mcp_response(process, timeout=5):
    """Read an MCP response from the server process."""
    import select
    
    if select.select([process.stdout], [], [], timeout)[0]:
        line = process.stdout.readline()
        if line:
            line = line.strip()
            print(f"← Received: {line}")
            try:
                return json.loads(line)
            except json.JSONDecodeError as e:
                print(f"Failed to parse JSON: {e}")
                return None
    return None

def test_ht_mcp_server():
    """Test the HT MCP server basic functionality."""
    print("Starting HT MCP Server test...")
    
    # Start the server process
    try:
        process = subprocess.Popen(
            ["./target/debug/ht-mcp"],
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
            bufsize=0
        )
        print("✓ Server process started")
        
        # Test 1: Initialize
        print("\n1. Testing initialize...")
        init_message = {
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
        
        send_mcp_message(process, init_message)
        response = read_mcp_response(process)
        
        if response and response.get("id") == 1:
            print("✓ Initialize successful")
            
            # Send initialized notification (required by MCP protocol)
            print("   Sending initialized notification...")
            initialized_message = {
                "jsonrpc": "2.0",
                "method": "notifications/initialized"
            }
            send_mcp_message(process, initialized_message)
            time.sleep(0.1)  # Give server time to process
            
        else:
            print("✗ Initialize failed")
            return False
        
        # Test 2: List tools
        print("\n2. Testing list tools...")
        list_tools_message = {
            "jsonrpc": "2.0", 
            "id": 2,
            "method": "tools/list",
            "params": {}
        }
        
        send_mcp_message(process, list_tools_message)
        response = read_mcp_response(process, timeout=10)  # Increase timeout
        
        if response and response.get("id") == 2:
            if "result" in response:
                tools = response["result"].get("tools", [])
                print(f"✓ Listed {len(tools)} tools")
                for tool in tools:
                    print(f"  - {tool.get('name')}: {tool.get('description')}")
            elif "error" in response:
                print(f"✗ List tools failed with error: {response['error']}")
                return False
            else:
                print("✗ List tools failed - unexpected response format")
                print(f"Response: {response}")
                return False
        else:
            print("✗ List tools failed - no response or wrong ID")
            if response:
                print(f"Response: {response}")
            return False
            
        # Test 3: Create session
        print("\n3. Testing create session...")
        create_session_message = {
            "jsonrpc": "2.0",
            "id": 3, 
            "method": "tools/call",
            "params": {
                "name": "ht_create_session",
                "arguments": {
                    "command": ["bash"],
                    "enableWebServer": False
                }
            }
        }
        
        send_mcp_message(process, create_session_message)
        response = read_mcp_response(process)
        
        if response and response.get("id") == 3 and "result" in response:
            result_content = response["result"]["content"][0]["text"]
            result_data = json.loads(result_content)
            session_id = result_data.get("sessionId")  # Note: sessionId not session_id
            print(f"✓ Created session: {session_id}")
        else:
            print("✗ Create session failed")
            return False
            
        print("\n✓ All tests passed!")
        return True
        
    except Exception as e:
        print(f"✗ Test failed with error: {e}")
        return False
    finally:
        # Clean up
        if 'process' in locals():
            process.terminate()
            process.wait(timeout=5)
            print("✓ Server process terminated")

if __name__ == "__main__":
    success = test_ht_mcp_server()
    sys.exit(0 if success else 1)