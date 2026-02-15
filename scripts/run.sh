#!/bin/bash
# Run the Neural Network Server
# Default port: 2421

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

# Build WASM if needed
echo "Building WASM module..."
cd "$PROJECT_ROOT/neural-net-wasm"
wasm-pack build --target web --out-dir pkg

# Copy WASM to server static directory
echo "Copying WASM to server static directory..."
mkdir -p "$PROJECT_ROOT/neural-net-server/static/wasm"
cp -r "$PROJECT_ROOT/neural-net-wasm/pkg"/* "$PROJECT_ROOT/neural-net-server/static/wasm/"

# Copy WASM to docs directory (for GitHub Pages)
echo "Copying WASM to docs directory..."
mkdir -p "$PROJECT_ROOT/docs/wasm"
cp -r "$PROJECT_ROOT/neural-net-wasm/pkg"/* "$PROJECT_ROOT/docs/wasm/"

# Update build info with cache-busting timestamps
echo "Updating build info..."
"$SCRIPT_DIR/update-build-info.sh"

# Run the server (from neural-net-server directory so static/ is found)
echo ""
echo "Starting server on http://127.0.0.1:2421"
echo ""
cd "$PROJECT_ROOT/neural-net-server"
cargo run --bin neural-net-server -- "$@"
