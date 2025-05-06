-- ClickHouse schema for Bitcoin UTXO tracking and token balances

-- Create a table to store raw UTXO data
CREATE TABLE IF NOT EXISTS bitcoin_utxos (
    tx_id String,                -- Transaction ID
    vout_index UInt32,           -- Output index in the transaction
    value UInt64,                -- Value in satoshis
    script_type String,          -- Type of script (P2PKH, P2SH, P2WPKH, etc.)
    address String,              -- Bitcoin address (if can be derived)
    block_height UInt64,         -- Block height when this UTXO was created
    block_time DateTime64(3),    -- Block timestamp when this UTXO was created
    is_spent UInt8 DEFAULT 0,    -- Flag to indicate if the UTXO has been spent
    spent_in_tx_id String DEFAULT '', -- Transaction ID that spent this UTXO
    spent_at_block_height UInt64 DEFAULT 0, -- Block height when this UTXO was spent
    spent_at_block_time DateTime64(3) DEFAULT '1970-01-01 00:00:00.000' -- Block timestamp when this UTXO was spent
) ENGINE = ReplacingMergeTree(block_height)
ORDER BY (address, tx_id, vout_index);

-- Create a table to store token balance snapshots
CREATE TABLE IF NOT EXISTS bitcoin_token_balances (
    address String,              -- Bitcoin address
    balance UInt64,              -- Balance in satoshis
    utxo_count UInt32,           -- Number of UTXOs for this address
    block_height UInt64,         -- Block height of this balance snapshot
    block_time DateTime64(3)     -- Block timestamp of this balance snapshot
) ENGINE = ReplacingMergeTree(block_height)
ORDER BY (address, block_height);

-- Create a materialized view to update token balances from UTXO changes
CREATE MATERIALIZED VIEW IF NOT EXISTS bitcoin_token_balances_mv
TO bitcoin_token_balances
AS
SELECT
    address,
    sum(value) AS balance,
    count() AS utxo_count,
    max(block_height) AS block_height,
    max(block_time) AS block_time
FROM bitcoin_utxos
WHERE is_spent = 0  -- Only include unspent outputs
GROUP BY address;

-- Create a view for the latest token balances
CREATE VIEW IF NOT EXISTS bitcoin_token_balances_latest AS
SELECT
    address,
    balance,
    utxo_count,
    block_height,
    block_time
FROM bitcoin_token_balances
FINAL  -- Use FINAL to get the latest version after deduplication
ORDER BY balance DESC;

-- Create a table to store transaction data for reference
CREATE TABLE IF NOT EXISTS bitcoin_transactions (
    tx_id String,                -- Transaction ID
    block_height UInt64,         -- Block height
    block_time DateTime64(3),    -- Block timestamp
    fee UInt64,                  -- Transaction fee in satoshis
    input_count UInt32,          -- Number of inputs
    output_count UInt32,         -- Number of outputs
    total_input_value UInt64,    -- Total value of inputs in satoshis
    total_output_value UInt64    -- Total value of outputs in satoshis
) ENGINE = ReplacingMergeTree(block_height)
ORDER BY (tx_id, block_height);

-- Create a table to store address transaction history
CREATE TABLE IF NOT EXISTS bitcoin_address_transactions (
    address String,              -- Bitcoin address
    tx_id String,                -- Transaction ID
    block_height UInt64,         -- Block height
    block_time DateTime64(3),    -- Block timestamp
    is_input UInt8,              -- 1 if address is an input, 0 if output
    value UInt64,                -- Value in satoshis
    balance_after UInt64         -- Balance after this transaction
) ENGINE = MergeTree()
ORDER BY (address, block_height, tx_id, is_input);

-- Create a materialized view to update address transaction history
CREATE MATERIALIZED VIEW IF NOT EXISTS bitcoin_address_transactions_mv
TO bitcoin_address_transactions
AS
WITH
    -- Get all outputs for addresses
    outputs AS (
        SELECT
            address,
            tx_id,
            block_height,
            block_time,
            0 AS is_input,
            value
        FROM bitcoin_utxos
    ),
    -- Get all inputs for addresses
    inputs AS (
        SELECT
            address,
            spent_in_tx_id AS tx_id,
            spent_at_block_height AS block_height,
            spent_at_block_time AS block_time,
            1 AS is_input,
            value
        FROM bitcoin_utxos
        WHERE is_spent = 1
    ),
    -- Combine inputs and outputs
    all_txs AS (
        SELECT * FROM outputs
        UNION ALL
        SELECT * FROM inputs
    )
SELECT
    address,
    tx_id,
    block_height,
    block_time,
    is_input,
    value,
    sum(if(is_input = 1, -value, value)) OVER (
        PARTITION BY address
        ORDER BY block_height, tx_id, is_input
        ROWS BETWEEN UNBOUNDED PRECEDING AND CURRENT ROW
    ) AS balance_after
FROM all_txs;

-- Create a table for rich list (top addresses by balance)
CREATE MATERIALIZED VIEW IF NOT EXISTS bitcoin_rich_list
ENGINE = ReplacingMergeTree
ORDER BY (balance DESC, address)
POPULATE AS
SELECT
    address,
    balance,
    utxo_count,
    block_height,
    block_time
FROM bitcoin_token_balances_latest
ORDER BY balance DESC
LIMIT 1000;

-- Create a table for daily statistics
CREATE TABLE IF NOT EXISTS bitcoin_daily_stats (
    date Date,                   -- Date
    active_addresses UInt64,     -- Number of active addresses
    transaction_count UInt64,    -- Number of transactions
    total_volume UInt64,         -- Total volume in satoshis
    avg_transaction_value UInt64, -- Average transaction value in satoshis
    new_addresses UInt64,        -- Number of new addresses
    total_addresses UInt64       -- Total number of addresses
) ENGINE = SummingMergeTree()
ORDER BY (date);

-- Create a materialized view to update daily statistics
CREATE MATERIALIZED VIEW IF NOT EXISTS bitcoin_daily_stats_mv
TO bitcoin_daily_stats
AS
SELECT
    toDate(block_time) AS date,
    uniqExact(address) AS active_addresses,
    uniqExact(tx_id) AS transaction_count,
    sum(if(is_input = 0, value, 0)) AS total_volume,
    if(uniqExact(tx_id) > 0, sum(if(is_input = 0, value, 0)) / uniqExact(tx_id), 0) AS avg_transaction_value,
    uniqExact(if(min(block_height) OVER (PARTITION BY address) = block_height, address, null)) AS new_addresses,
    uniqExact(address) AS total_addresses
FROM bitcoin_address_transactions
GROUP BY date;
