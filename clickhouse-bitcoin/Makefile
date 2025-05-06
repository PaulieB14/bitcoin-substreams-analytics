.PHONY: setup run query clean

# Default ClickHouse connection string
CLICKHOUSE_DSN ?= clickhouse://default:default@localhost:9000/default

# Substreams endpoint
ENDPOINT ?= bitcoin.substreams.pinax.network:443

# Substreams package
SPKG ?= ../substreams.spkg

# Module to run
MODULE ?= map_utxos

# Block range
START_BLOCK ?= 800000
STOP_BLOCK ?= +10

setup:
	substreams-sink-sql setup $(CLICKHOUSE_DSN) schema.sql

run:
	substreams-sink-sql run $(CLICKHOUSE_DSN) sink_config.yaml $(SPKG) $(MODULE) -s $(START_BLOCK) -t $(STOP_BLOCK)

run-forever:
	substreams-sink-sql run $(CLICKHOUSE_DSN) sink_config.yaml $(SPKG) $(MODULE) -s $(START_BLOCK)

query:
	@echo "-- Select top 10 addresses by balance"
	@clickhouse-client --query "SELECT address, balance, utxo_count FROM bitcoin_token_balances_latest ORDER BY balance DESC LIMIT 10;"

clean:
	@echo "-- Dropping all tables"
	@clickhouse-client --query "DROP TABLE IF EXISTS bitcoin_utxos;"
	@clickhouse-client --query "DROP TABLE IF EXISTS bitcoin_token_balances;"
	@clickhouse-client --query "DROP TABLE IF EXISTS bitcoin_token_balances_mv;"
	@clickhouse-client --query "DROP TABLE IF EXISTS bitcoin_token_balances_latest;"
	@clickhouse-client --query "DROP TABLE IF EXISTS bitcoin_transactions;"
	@clickhouse-client --query "DROP TABLE IF EXISTS bitcoin_address_transactions;"
	@clickhouse-client --query "DROP TABLE IF EXISTS bitcoin_address_transactions_mv;"
	@clickhouse-client --query "DROP TABLE IF EXISTS bitcoin_rich_list;"
	@clickhouse-client --query "DROP TABLE IF EXISTS bitcoin_daily_stats;"
	@clickhouse-client --query "DROP TABLE IF EXISTS bitcoin_daily_stats_mv;"
