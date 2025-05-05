# Bitcoin Token Balances in ClickHouse

This document explains how to set up and use the ClickHouse schema for tracking Bitcoin token balances from UTXO data.

## Overview

The ClickHouse schema provides a comprehensive solution for tracking Bitcoin UTXOs and token balances. It includes:

1. **UTXO Tracking**: Store all unspent transaction outputs with their current status
2. **Token Balances**: Materialized view that calculates address balances from UTXOs
3. **Transaction History**: Track all transactions for each address
4. **Rich List**: Top addresses by balance
5. **Daily Statistics**: Aggregate statistics on a daily basis

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

## Integration with Substreams

To integrate this ClickHouse schema with your Bitcoin Substreams project:

1. **Set up ClickHouse**: Install and configure ClickHouse server
2. **Create Schema**: Run the `clickhouse_schema.sql` script to create the tables and views
3. **Modify Substreams**: Update your Substreams code to output UTXO data
4. **Use a Sink**: Configure a Substreams sink to send data to ClickHouse

### Substreams Modifications

You'll need to modify your Substreams code to track UTXOs. Here's a high-level approach:

1. Create a new module that processes transactions to track UTXOs
2. For each transaction:
   - Add new outputs to the UTXO set
   - Mark spent inputs as spent in the UTXO set
3. Output the UTXO changes to be consumed by the sink

### Example Sink Configuration

You can use the Substreams Sink SQL to send data to ClickHouse. Here's an example configuration:

```yaml
sink:
  type: sql
  dsn: clickhouse://user:password@localhost:9000/default
  schema: clickhouse_schema.sql
  tables:
    - name: bitcoin_utxos
      columns:
        - name: tx_id
          type: String
        - name: vout_index
          type: UInt32
        - name: value
          type: UInt64
        - name: script_type
          type: String
        - name: address
          type: String
        - name: block_height
          type: UInt64
        - name: block_time
          type: DateTime64(3)
        - name: is_spent
          type: UInt8
        - name: spent_in_tx_id
          type: String
        - name: spent_at_block_height
          type: UInt64
        - name: spent_at_block_time
          type: DateTime64(3)
```

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
