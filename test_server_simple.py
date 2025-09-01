#!/usr/bin/env python3
"""
Simple test for the Multi-Vendor Delivery Server
Tests basic functionality without complex dependencies
"""

import requests
import time
import subprocess
import sys
import threading
import signal
import os

def test_health_endpoint():
    """Test the health check endpoint"""
    print("🏥 Testing health endpoint...")
    try:
        response = requests.get("http://localhost:8443/health", timeout=5)
        if response.status_code == 200 and response.text.strip() == "OK":
            print("✅ Health check passed!")
            return True
        else:
            print(f"❌ Health check failed: {response.status_code} - {response.text}")
            return False
    except requests.exceptions.RequestException as e:
        print(f"❌ Health check failed: {e}")
        return False

def test_protected_endpoint():
    """Test a protected endpoint (should fail without auth)"""
    print("🔒 Testing protected endpoint without auth...")
    try:
        response = requests.post("http://localhost:8443/orders", 
                               json={"test": "data"}, 
                               timeout=5)
        if response.status_code == 401:
            print("✅ Protected endpoint correctly rejected unauthorized request")
            return True
        else:
            print(f"⚠️  Unexpected response: {response.status_code}")
            return False
    except requests.exceptions.RequestException as e:
        print(f"❌ Request failed: {e}")
        return False

def run_server():
    """Run the server in a subprocess"""
    print("🚀 Starting server...")
    try:
        # Set environment variables for the server
        env = os.environ.copy()
        env.update({
            'RUST_LOG': 'info',
            'SERVER_HOST': '127.0.0.1',
            'SERVER_PORT': '8443',
            'FIREBASE_PROJECT_ID': 'test-project',
            'FIREBASE_SERVICE_ACCOUNT_KEY': 'test-key.json'
        })
        
        process = subprocess.Popen(
            ['cargo', 'run'],
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            env=env,
            text=True
        )
        return process
    except Exception as e:
        print(f"❌ Failed to start server: {e}")
        return None

def main():
    print("🧪 Multi-Vendor Delivery Server Test")
    print("====================================")
    
    # Start the server
    server_process = run_server()
    if not server_process:
        print("❌ Could not start server")
        return False
    
    try:
        # Wait for server to start
        print("⏳ Waiting for server to start...")
        time.sleep(5)
        
        # Check if server is still running
        if server_process.poll() is not None:
            stdout, stderr = server_process.communicate()
            print(f"❌ Server exited early:")
            print(f"STDOUT: {stdout}")
            print(f"STDERR: {stderr}")
            return False
        
        print("✅ Server appears to be running")
        
        # Run tests
        health_ok = test_health_endpoint()
        time.sleep(1)
        
        if health_ok:
            auth_ok = test_protected_endpoint()
            
            if auth_ok:
                print("\n🎉 All basic tests passed!")
                print("✅ Server is working correctly")
                return True
            else:
                print("\n⚠️  Some tests failed")
                return False
        else:
            print("\n❌ Server is not responding properly")
            return False
            
    finally:
        # Clean up
        print("\n🧹 Cleaning up...")
        if server_process and server_process.poll() is None:
            server_process.terminate()
            try:
                server_process.wait(timeout=5)
            except subprocess.TimeoutExpired:
                server_process.kill()
        print("✅ Cleanup complete")

if __name__ == "__main__":
    success = main()
    sys.exit(0 if success else 1)