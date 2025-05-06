//! Bitcoin ClickHouse schema for Substreams
//!
//! This crate provides the ClickHouse schema and integration for Bitcoin Substreams.
//! It is designed to work with the `substreams-sink-sql` tool to load Bitcoin UTXO data
//! into ClickHouse for efficient querying and analytics.

/// Version of the crate
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Documentation URL
pub const DOCUMENTATION_URL: &str = "https://github.com/PaulieB14/bitcoin-substreams-analytics/tree/main/clickhouse-bitcoin";

/// Helper function to get the ClickHouse DSN from environment variables
pub fn get_clickhouse_dsn() -> String {
    std::env::var("CLICKHOUSE_DSN")
        .unwrap_or_else(|_| "clickhouse://default:default@localhost:9000/default".to_string())
}

/// Helper function to check if a ClickHouse table exists
pub fn table_exists(dsn: &str, table_name: &str) -> bool {
    // This is a placeholder function that would use clickhouse-rs to check if a table exists
    // In a real implementation, this would connect to ClickHouse and run a query
    println!("Checking if table {} exists in {}", table_name, dsn);
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_clickhouse_dsn() {
        let dsn = get_clickhouse_dsn();
        assert!(!dsn.is_empty());
    }
}
