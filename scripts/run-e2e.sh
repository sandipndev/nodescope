#!/usr/bin/env bash

set -euo pipefail

echo "Starting nodescope-cli in background..."

# Start nodescope-cli in background
./target/debug/nodescope-cli &
CLI_PID=$!

# Function to cleanup background process
cleanup() {
    echo "Cleaning up..."
    if kill -0 $CLI_PID 2>/dev/null; then
        echo "Stopping nodescope-cli (PID: $CLI_PID)"
        kill $CLI_PID
        wait $CLI_PID 2>/dev/null || true
    fi
}

# Set trap to cleanup on script exit
trap cleanup EXIT

echo "nodescope-cli started with PID: $CLI_PID"

# Wait for service to start up
echo "Waiting for service to start..."
sleep 5

# Health check function
check_endpoint() {
    local url=$1
    local name=$2
    echo "Checking $name at $url..."

    for i in {1..30}; do
        if curl -s --max-time 5 --fail "$url" > /dev/null 2>&1; then
            echo "✓ $name is responding (200 OK)"
            return 0
        fi
        echo "Attempt $i: $name not ready, waiting..."
        sleep 2
    done

    echo "✗ $name failed to respond after 60 seconds"
    return 1
}

# Check both endpoints
check_endpoint "http://localhost:6789/" "Main endpoint"
check_endpoint "http://localhost:6789/graphql" "GraphQL endpoint"

echo "All health checks passed!"
