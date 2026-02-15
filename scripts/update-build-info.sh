#!/bin/bash
# Update build info and cache-busting timestamps in HTML files
# Run this before deploying or committing UI changes

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

# Get build info
GIT_HASH=$(git -C "$PROJECT_ROOT" rev-parse --short HEAD)
HOSTNAME=$(hostname)
TIMESTAMP=$(date -u +"%Y-%m-%d %H:%M:%S UTC")
YEAR=$(date +"%Y")
EPOCH_MS=$(date +%s)000

# Copyright year range (started in 2025)
if [ "$YEAR" = "2025" ]; then
    COPYRIGHT="Copyright (c) 2025 Michael A. Wright"
else
    COPYRIGHT="Copyright (c) 2025-$YEAR Michael A. Wright"
fi

echo "Updating build info:"
echo "  Git Hash: $GIT_HASH"
echo "  Hostname: $HOSTNAME"
echo "  Timestamp: $TIMESTAMP"
echo "  Cache-bust: ts=$EPOCH_MS"
echo "  Copyright: $COPYRIGHT"
echo ""

# Files to update
HTML_FILES=(
    "$PROJECT_ROOT/neural-net-server/static/index.html"
    "$PROJECT_ROOT/docs/index.html"
)

for file in "${HTML_FILES[@]}"; do
    if [ -f "$file" ]; then
        echo "Updating: $file"

        # Update build info line (using @ as delimiter to avoid escaping issues)
        sed -i.bak "s@Build: [a-f0-9]* | Host: [^|]* | [0-9-]* [0-9:]* UTC@Build: $GIT_HASH | Host: $HOSTNAME | $TIMESTAMP@g" "$file"

        # Update copyright
        sed -i.bak "s@Copyright (c) [0-9-]* Michael A. Wright@$COPYRIGHT@g" "$file"

        # Update cache-busting for app.js (replace any existing ?v= or ?ts= params)
        sed -i.bak -E "s@app\.js\?[^\"]+@app.js?ts=$EPOCH_MS@g" "$file"

        # Clean up backup files
        rm -f "$file.bak"
    else
        echo "Warning: File not found: $file"
    fi
done

# Update WASM import in app.js
APP_JS="$PROJECT_ROOT/neural-net-server/static/app.js"
if [ -f "$APP_JS" ]; then
    echo "Updating: $APP_JS"
    sed -i.bak -E "s@neural_net_wasm\.js\?[^']+@neural_net_wasm.js?ts=$EPOCH_MS@g" "$APP_JS"
    rm -f "$APP_JS.bak"
fi

# Also update docs app.js if it exists
DOCS_APP_JS="$PROJECT_ROOT/docs/app.js"
if [ -f "$DOCS_APP_JS" ]; then
    echo "Updating: $DOCS_APP_JS"
    sed -i.bak -E "s@neural_net_wasm\.js\?[^']+@neural_net_wasm.js?ts=$EPOCH_MS@g" "$DOCS_APP_JS"
    rm -f "$DOCS_APP_JS.bak"
fi

echo ""
echo "Build info updated successfully!"
echo "Remember to commit these changes."
