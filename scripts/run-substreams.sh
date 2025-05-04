#!/bin/bash

# Script to run Bitcoin Substreams and output data for ClickHouse

# Check if API key is set
if [ -z "$SUBSTREAMS_API_TOKEN" ]; then
  echo "Error: SUBSTREAMS_API_TOKEN environment variable is not set"
  echo "Please set it with: export SUBSTREAMS_API_TOKEN=your_api_key"
  exit 1
fi

# Set variables
ENDPOINT="bitcoin.substreams.pinax.network:443"
PACKAGE_VERSION="v0.1.0"

# Check for command-line arguments
START_BLOCK=800000
END_BLOCK=""
MODULE="db_out"

# Parse command-line arguments
while [[ $# -gt 0 ]]; do
  case $1 in
    --start-block)
      START_BLOCK="$2"
      shift 2
      ;;
    --stop-block)
      END_BLOCK="$2"
      shift 2
      ;;
    --module)
      MODULE="$2"
      shift 2
      ;;
    *)
      echo "Unknown option: $1"
      exit 1
      ;;
  esac
done

# Build and pack the Substreams module
echo "Building Substreams module..."
cargo build --target wasm32-unknown-unknown --release

echo "Packing Substreams module..."
substreams pack

# Run the Substreams
echo "Running Substreams from block $START_BLOCK..."
if [ -z "$END_BLOCK" ]; then
  # Run without an end block (stream to head)
  substreams run \
    -e "$ENDPOINT" \
    --start-block "$START_BLOCK" \
    -H "Authorization=Bearer $SUBSTREAMS_API_TOKEN" \
    --production-mode \
    "substreams.yaml" \
    "$MODULE"
else
  # Run with specified end block
  substreams run \
    -e "$ENDPOINT" \
    --start-block "$START_BLOCK" \
    --stop-block "$END_BLOCK" \
    -H "Authorization=Bearer $SUBSTREAMS_API_TOKEN" \
    --production-mode \
    "substreams.yaml" \
    "$MODULE"
fi
