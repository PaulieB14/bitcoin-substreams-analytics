#!/bin/bash

if [ -z "$SUBSTREAMS_API_TOKEN" ]; then
  echo "Error: SUBSTREAMS_API_TOKEN environment variable not set."
  echo "Please set your Pinax API key with: export SUBSTREAMS_API_TOKEN=your_pinax_api_key"
  exit 1
fi

# Build the substreams module
echo "Building Substreams module..."
cargo build --target wasm32-unknown-unknown --release

# Package the substreams module
echo "Packaging Substreams module..."
substreams pack

# Run the specified module or default to map_events
MODULE=${1:-map_events}
echo "Running Substreams module: $MODULE"

substreams run \
  --substreams-endpoint https://bitcoin.substreams.pinax.network:443/v1/substreams \
  substreams.yaml \
  $MODULE \
  --start-block 800000 \
  --production-mode \
  --stop-block +100
