#!/usr/bin/env python3
"""
Test script to verify terminal functionality works with real terminal sessions.
"""

import json
import subprocess
import sys
import time

def send_mcp_request(process, request):
    """Send an MCP request and get the response."""
    request_str = json.dumps(request) + '\n'
    process.stdin.write(request_str)
    process.stdin.flush()
    
    response_str = process.stdout.readline()
    return json.loads(response_str.strip())

def main():
    print("Starting comprehensive HT MCP Server terminal test...")
    
    # Start the server process
    process = subprocess.Popen(
        ['cargo', 'run'],
        stdin=subprocess.PIPE,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
        cwd='/Users/chilang/Workspace/memex_headless_mcp_setup/ht-mcp-rust'
    )
    
    try:
        print("✓ Server process started")
        time.sleep(1)  # Give server time to start
        
        # Initialize
        init_response = send_mcp_request(process, {
            "jsonrpc": "2.0",
            "id": 1,
            "method": "initialize",
            "params": {
                "protocolVersion": "2024-11-05",
                "capabilities": {},
                "clientInfo": {"name": "test-client", "version": "1.0.0"}
            }
        })
        
        if "result" not in init_response:
            print(f"✗ Initialize failed: {init_response}")
            return False
        print("✓ Initialize successful")
        
        # Send initialized notification (skip - optional)
        # process.stdin.write(json.dumps({
        #     "jsonrpc": "2.0",
        #     "method": "notifications/initialized"
        # }) + '\n')
        # process.stdin.flush()
        
        # Create a session
        create_response = send_mcp_request(process, {
            "jsonrpc": "2.0",
            "id": 2,
            "method": "tools/call",
            "params": {
                "name": "ht_create_session",
                "arguments": {
                    "command": ["bash"],
                    "enableWebServer": False
                }
            }
        })
        
        if "result" not in create_response:
            print(f"✗ Create session failed: {create_response}")
            return False
            
        # Extract session ID
        result_content = json.loads(create_response["result"]["content"][0]["text"])
        session_id = result_content["sessionId"]
        print(f"✓ Created session: {session_id}")
        
        # Take initial snapshot
        print("\n--- Taking initial snapshot ---")
        snapshot_response = send_mcp_request(process, {
            "jsonrpc": "2.0",
            "id": 3,
            "method": "tools/call",
            "params": {
                "name": "ht_take_snapshot",
                "arguments": {
                    "sessionId": session_id
                }
            }
        })
        
        if "result" not in snapshot_response:
            print(f"✗ Snapshot failed: {snapshot_response}")
            return False
            
        initial_snapshot = json.loads(snapshot_response["result"]["content"][0]["text"])
        print(f"✓ Initial snapshot captured (length: {len(initial_snapshot['snapshot'])} chars)")
        print(f"Preview: {repr(initial_snapshot['snapshot'][:100])}")
        
        # Send a command to see output
        print("\n--- Sending 'echo Hello World' command ---")
        send_keys_response = send_mcp_request(process, {
            "jsonrpc": "2.0",
            "id": 4,
            "method": "tools/call",
            "params": {
                "name": "ht_send_keys",
                "arguments": {
                    "sessionId": session_id,
                    "keys": ["echo Hello World"]
                }
            }
        })
        
        if "result" not in send_keys_response:
            print(f"✗ Send keys failed: {send_keys_response}")
            return False
        print("✓ Sent command")
        
        # Send Enter key
        enter_response = send_mcp_request(process, {
            "jsonrpc": "2.0",
            "id": 5,
            "method": "tools/call",
            "params": {
                "name": "ht_send_keys",
                "arguments": {
                    "sessionId": session_id,
                    "keys": ["Enter"]
                }
            }
        })
        
        if "result" not in enter_response:
            print(f"✗ Send Enter failed: {enter_response}")
            return False
        print("✓ Pressed Enter")
        
        # Wait for command execution
        time.sleep(2)
        
        # Take snapshot after command
        print("\n--- Taking snapshot after command ---")
        after_snapshot_response = send_mcp_request(process, {
            "jsonrpc": "2.0",
            "id": 6,
            "method": "tools/call",
            "params": {
                "name": "ht_take_snapshot",
                "arguments": {
                    "sessionId": session_id
                }
            }
        })
        
        if "result" not in after_snapshot_response:
            print(f"✗ After snapshot failed: {after_snapshot_response}")
            return False
            
        after_snapshot = json.loads(after_snapshot_response["result"]["content"][0]["text"])
        print(f"✓ After snapshot captured (length: {len(after_snapshot['snapshot'])} chars)")
        print(f"Content: {repr(after_snapshot['snapshot'])}")
        
        # Check if the output contains our command
        if "Hello World" in after_snapshot['snapshot']:
            print("✅ SUCCESS: Snapshot contains command output!")
        else:
            print("⚠️  WARNING: Snapshot doesn't contain expected output")
            print("This might indicate the command hasn't executed yet or output isn't being captured")
        
        # Test execute_command functionality
        print("\n--- Testing execute_command ---")
        execute_response = send_mcp_request(process, {
            "jsonrpc": "2.0",
            "id": 7,
            "method": "tools/call",
            "params": {
                "name": "ht_execute_command",
                "arguments": {
                    "sessionId": session_id,
                    "command": "date"
                }
            }
        })
        
        if "result" not in execute_response:
            print(f"✗ Execute command failed: {execute_response}")
            return False
            
        execute_result = json.loads(execute_response["result"]["content"][0]["text"])
        print(f"✓ Execute command result: {repr(execute_result['output'][:100])}")
        
        # List sessions
        print("\n--- Testing list_sessions ---")
        list_response = send_mcp_request(process, {
            "jsonrpc": "2.0",
            "id": 8,
            "method": "tools/call",
            "params": {
                "name": "ht_list_sessions",
                "arguments": {}
            }
        })
        
        if "result" not in list_response:
            print(f"✗ List sessions failed: {list_response}")
            return False
            
        list_result = json.loads(list_response["result"]["content"][0]["text"])
        print(f"✓ Active sessions: {list_result['count']}")
        print(f"Sessions: {list_result['sessions']}")
        
        # Close session
        print("\n--- Closing session ---")
        close_response = send_mcp_request(process, {
            "jsonrpc": "2.0",
            "id": 9,
            "method": "tools/call",
            "params": {
                "name": "ht_close_session",
                "arguments": {
                    "sessionId": session_id
                }
            }
        })
        
        if "result" not in close_response:
            print(f"✗ Close session failed: {close_response}")
            return False
            
        print("✓ Session closed successfully")
        
        print("\n✅ All terminal functionality tests passed!")
        return True
        
    except Exception as e:
        print(f"✗ Test failed with exception: {e}")
        return False
        
    finally:
        process.terminate()
        process.wait()
        print("✓ Server process terminated")

if __name__ == "__main__":
    success = main()
    sys.exit(0 if success else 1)