# Bitcoin Token Balances in ClickHouse

This directory contains the ClickHouse schema and configuration for tracking Bitcoin token balances from UTXO data.

## Overview

The ClickHouse schema provides a comprehensive solution for tracking Bitcoin UTXOs and token balances. It includes:

1. **UTXO Tracking**: Store all unspent transaction outputs with their current status
2. **Token Balances**: Materialized view that calculates address balances from UTXOs
3. **Transaction History**: Track all transactions for each address
4. **Rich List**: Top addresses by balance
5. **Daily Statistics**: Aggregate statistics on a daily basis

## Quickstart

### 1. Install Prerequisites

First, make sure you have the required tools installed:

```bash
# Install substreams-sink-sql
brew install streamingfast/tap/substreams-sink-sql

# Verify installation
substreams-sink-sql --version
```

If you encounter issues with the brew installation, you can install it manually:

```bash
# Download the latest release
curl -LO https://github.com/streamingfast/substreams-sink-sql/releases/download/v1.1.0/substreams-sink-sql_darwin_arm64.tar.gz

# Extract the binary
tar -xzf substreams-sink-sql_darwin_arm64.tar.gz

# Move to a directory in your PATH
sudo mv substreams-sink-sql /usr/local/bin/

# Make it executable
chmod +x /usr/local/bin/substreams-sink-sql

# Verify installation
substreams-sink-sql --version
```

### 2. Setup SQL tables in ClickHouse

Make sure you have ClickHouse running locally. You can start it with Docker:

```bash
docker run -d --name clickhouse-server -p 8123:8123 -p 9000:9000 clickhouse/clickhouse-server
```

Then create the schema:

```bash
make setup
```

### Load ClickHouse data from Substreams

```bash
make run
```

### Perform SQL query with ClickHouse

```sql
-- Select all transactions from block 123456
SELECT
  *
FROM transactions
WHERE block_num = 123456;
```

## Schema Structure

The schema consists of several tables and materialized views:

- `bitcoin_utxos`: Stores raw UTXO data
- `bitcoin_token_balances`: Stores token balance snapshots
- `bitcoin_token_balances_mv`: Materialized view to update token balances
- `bitcoin_token_balances_latest`: View for the latest token balances
- `bitcoin_transactions`: Stores transaction data
- `bitcoin_address_transactions`: Stores address transaction history
- `bitcoin_address_transactions_mv`: Materialized view to update address transaction history
- `bitcoin_rich_list`: Materialized view for top addresses by balance
- `bitcoin_daily_stats`: Stores daily statistics
- `bitcoin_daily_stats_mv`: Materialized view to update daily statistics

## Querying Token Balances

Once your data is flowing into ClickHouse, you can query token balances:

```sql
-- Get top 10 addresses by balance
SELECT address, balance, utxo_count
FROM bitcoin_token_balances_latest
ORDER BY balance DESC
LIMIT 10;

-- Get balance for a specific address
SELECT address, balance, utxo_count, block_height, block_time
FROM bitcoin_token_balances_latest
WHERE address = 'bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh';

-- Get transaction history for an address
SELECT tx_id, block_height, block_time, is_input, value, balance_after
FROM bitcoin_address_transactions
WHERE address = 'bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh'
ORDER BY block_height, tx_id, is_input;

-- Get daily statistics
SELECT date, active_addresses, transaction_count, total_volume, avg_transaction_value
FROM bitcoin_daily_stats
ORDER BY date DESC
LIMIT 30;
```

## Performance Considerations

- The schema uses the `ReplacingMergeTree` engine for most tables to handle updates efficiently
- Indexes are created on frequently queried columns
- Materialized views are used to pre-compute aggregations
- Consider partitioning large tables by date or block range for better performance

## Maintenance

Regular maintenance tasks:

1. **Optimize Tables**: Run `OPTIMIZE TABLE` periodically to merge parts
2. **Monitor Disk Space**: UTXOs can grow significantly over time
3. **Backup Data**: Set up regular backups of your ClickHouse data

## Extending the Schema

You can extend this schema to track additional metrics:

- Token velocity
- HODL waves (age distribution of UTXOs)
- Network value to transactions ratio
- Realized capitalization
- Supply distribution by address size
