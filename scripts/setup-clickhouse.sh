#!/bin/bash

# Script to set up ClickHouse for Bitcoin analytics

# Check if clickhouse-client is installed
if ! command -v clickhouse-client &> /dev/null; then
    echo "Error: clickhouse-client is not installed. Please install ClickHouse first."
    echo "Visit https://clickhouse.com/docs/en/install/ for installation instructions."
    exit 1
fi

# Default connection parameters - modify as needed
HOST="localhost"
PORT="9000"
USER="default"
PASSWORD=""
DATABASE="bitcoin_analytics"

# Parse command-line arguments
while [[ $# -gt 0 ]]; do
  case $1 in
    --host)
      HOST="$2"
      shift 2
      ;;
    --port)
      PORT="$2"
      shift 2
      ;;
    --user)
      USER="$2"
      shift 2
      ;;
    --password)
      PASSWORD="$2"
      shift 2
      ;;
    --database)
      DATABASE="$2"
      shift 2
      ;;
    *)
      echo "Unknown option: $1"
      exit 1
      ;;
  esac
done

# Connection string
CONNECTION=""
if [ -z "$PASSWORD" ]; then
    CONNECTION="--host $HOST --port $PORT --user $USER"
else
    CONNECTION="--host $HOST --port $PORT --user $USER --password $PASSWORD"
fi

echo "Setting up ClickHouse database for Bitcoin analytics..."

# Create database if it doesn't exist
echo "Creating database $DATABASE if it doesn't exist..."
clickhouse-client $CONNECTION -q "CREATE DATABASE IF NOT EXISTS $DATABASE"

# Switch to the database
CONNECTION="$CONNECTION --database $DATABASE"

# Create tables - Using the same schema as in clickhouse-sink.yaml
echo "Creating tables..."

# Bitcoin blocks table
clickhouse-client $CONNECTION -q "
CREATE TABLE IF NOT EXISTS bitcoin_blocks (
    block_height UInt32,
    block_hash String,
    timestamp DateTime,
    size UInt32,
    weight UInt32,
    tx_count UInt16,
    difficulty Float64,
    miner_name String,
    block_time_seconds UInt16,
    block_reward UInt64,
    total_fees UInt64,
    version UInt16,
    nonce UInt32,
    bits UInt32
) ENGINE = MergeTree()
PARTITION BY toYYYYMM(timestamp)
ORDER BY (block_height)
"

# Bitcoin transactions table
clickhouse-client $CONNECTION -q "
CREATE TABLE IF NOT EXISTS bitcoin_transactions (
    tx_hash String,
    block_height UInt32,
    block_timestamp DateTime,
    input_count UInt16,
    output_count UInt16,
    fee UInt64,
    fee_rate Float32,
    size UInt32,
    weight UInt32,
    is_coinbase UInt8,
    version UInt8,
    has_witness UInt8,
    locktime UInt32,
    input_value UInt64,
    output_value UInt64,
    tx_type String
) ENGINE = MergeTree()
PARTITION BY toYYYYMM(block_timestamp)
ORDER BY (block_height, tx_hash)
"

# Bitcoin addresses table
clickhouse-client $CONNECTION -q "
CREATE TABLE IF NOT EXISTS bitcoin_addresses (
    address String,
    tx_hash String,
    block_height UInt32,
    block_timestamp DateTime,
    is_input UInt8,
    value UInt64,
    script_type String,
    address_tag String,
    address_category String
) ENGINE = MergeTree()
PARTITION BY toYYYYMM(block_timestamp)
ORDER BY (address, block_height)
"

# Bitcoin network daily metrics table
clickhouse-client $CONNECTION -q "
CREATE TABLE IF NOT EXISTS bitcoin_network_daily (
    date Date,
    avg_block_time Float32,
    total_tx_count UInt32,
    total_tx_volume UInt64,
    avg_block_size UInt32,
    avg_tx_per_block Float32,
    avg_fee_rate Float32,
    mempool_tx_count UInt32,
    segwit_tx_percent Float32,
    taproot_tx_percent Float32,
    avg_difficulty Float64,
    active_addresses UInt32
) ENGINE = MergeTree()
ORDER BY (date)
"

echo "ClickHouse database setup complete!"
echo
echo "Next steps:"
echo "1. Update clickhouse-sink.yaml with your connection details"
echo "2. Run the Substreams sink:"
echo "   substreams-sink-clickhouse run clickhouse-sink.yaml"
echo
echo "You can now connect to your ClickHouse database and run queries:"
echo "clickhouse-client $CONNECTION"
