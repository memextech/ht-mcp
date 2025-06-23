#!/usr/bin/env python3

import json
import subprocess
import sys
import time

def debug_mcp_server():
    """Simple diagnostic client to test MCP server communication"""
    
    print("Starting ht-mcp server...")
    
    # Start the server
    proc = subprocess.Popen(
        ["./target/debug/ht-mcp"],
        stdin=subprocess.PIPE,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
        bufsize=0
    )
    
    try:
        # Send initialize message
        init_msg = {
            "jsonrpc": "2.0",
            "id": 1,
            "method": "initialize",
            "params": {
                "protocolVersion": "2024-11-05",
                "capabilities": {},
                "clientInfo": {"name": "debug-client", "version": "1.0.0"}
            }
        }
        
        print(f"Sending: {json.dumps(init_msg)}")
        proc.stdin.write(json.dumps(init_msg) + "\n")
        proc.stdin.flush()
        
        # Try to read response with timeout
        print("Waiting for response...")
        time.sleep(1)
        
        # Check if server is still running
        if proc.poll() is not None:
            print(f"Server terminated with code: {proc.poll()}")
            stderr = proc.stderr.read()
            print(f"Server stderr: {stderr}")
            return
        
        # Try to read stdout
        try:
            response_line = proc.stdout.readline()
            print(f"Server response: {response_line.strip()}")
            
            if response_line.strip():
                response = json.loads(response_line.strip())
                print(f"Parsed response: {json.dumps(response, indent=2)}")
            
            # Send initialized notification
            print("Sending initialized notification...")
            initialized = {
                "jsonrpc": "2.0",
                "method": "notifications/initialized"
            }
            proc.stdin.write(json.dumps(initialized) + "\n")
            proc.stdin.flush()
            
            # Now test creating and closing a session
            print("Testing session creation and closure...")
            
            # Create session
            create_msg = {
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
            }
            
            print("Creating session...")
            proc.stdin.write(json.dumps(create_msg) + "\n")
            proc.stdin.flush()
            
            create_response = proc.stdout.readline()
            print(f"Create response: {create_response.strip()}")
            
            # Parse session ID
            create_data = json.loads(create_response.strip())
            response_text = create_data["result"]["content"][0]["text"]
            session_id = None
            for line in response_text.split('\n'):
                if line.startswith("Session ID:"):
                    session_id = line.split(": ")[1]
                    break
            
            print(f"Extracted session ID: {session_id}")
            
            if session_id:
                # Close session
                close_msg = {
                    "jsonrpc": "2.0",
                    "id": 3,
                    "method": "tools/call",
                    "params": {
                        "name": "ht_close_session",
                        "arguments": {
                            "sessionId": session_id
                        }
                    }
                }
                
                print("Closing session...")
                proc.stdin.write(json.dumps(close_msg) + "\n")
                proc.stdin.flush()
                
                # Check if server is still alive
                time.sleep(0.5)
                if proc.poll() is not None:
                    print(f"WARNING: Server terminated after close_session call with exit code: {proc.poll()}")
                    stderr = proc.stderr.read()
                    print(f"Server stderr: {stderr}")
                else:
                    close_response = proc.stdout.readline()
                    print(f"Close response: {close_response.strip()}")
            
        except Exception as e:
            print(f"Error reading response: {e}")
            if proc.poll() is not None:
                print(f"Server terminated with exit code: {proc.poll()}")
                stderr = proc.stderr.read()
                print(f"Server stderr: {stderr}")
            
    except Exception as e:
        print(f"Error communicating with server: {e}")
        
    finally:
        print("Terminating server...")
        proc.terminate()
        proc.wait()

if __name__ == "__main__":
    debug_mcp_server()