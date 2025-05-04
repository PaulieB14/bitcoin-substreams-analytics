# Bitcoin Substreams Analytics

A powerful framework for extracting, processing, and visualizing Bitcoin blockchain data using Substreams technology and ClickHouse integration.

![Bitcoin Analytics Dashboard](https://raw.githubusercontent.com/PaulieB14/bitcoin-substreams-analytics/main/dashboard/screenshot.png)

## Overview

This project provides a comprehensive solution for Bitcoin blockchain analytics, including:

- Extraction of key metrics from Bitcoin blocks and transactions
- Processing and aggregation using Substreams technology
- Storage in ClickHouse for high-performance analytics
- Interactive dashboard for visualization

## Key Features

- **Block Analytics**: Block size, weight, mining pools, timestamps, and other metrics
- **Transaction Analytics**: Fee market analysis, volume trends, UTXO tracking
- **Address Analytics**: Activity patterns, balance distribution, whale detection
- **Network Analytics**: Mempool metrics, protocol adoption (SegWit, Taproot)

## Project Structure

```
bitcoin-substreams-analytics/
├── proto/                      # Protocol Buffer definitions
│   └── analytics.proto         # Data models for analytics
├── src/                        # Rust source code
│   ├── lib.rs                  # Main library implementation
│   ├── pb/                     # Generated Protocol Buffer code
│   ├── utils.rs                # Utility functions
│   └── mappers/                # Data mapping modules
│       ├── mod.rs              # Module definitions
│       ├── block.rs            # Block data extraction
│       ├── transaction.rs      # Transaction data extraction
│       ├── address.rs          # Address data extraction
│       └── mempool.rs          # Mempool data extraction
├── dashboard/                  # Dashboard implementation
│   ├── index.html              # Dashboard HTML
│   ├── styles/                 # CSS styles
│   │   └── main.css            # Main stylesheet
│   └── scripts/                # JavaScript for dashboard
│       ├── api.js              # API client
│       ├── charts.js           # Chart visualizations
│       └── main.js             # Main dashboard logic
├── scripts/                    # Helper scripts
│   ├── run-substreams.sh       # Run Substreams processing
│   └── setup-clickhouse.sh     # Set up ClickHouse database
├── substreams.yaml             # Substreams manifest
├── clickhouse-sink.yaml        # ClickHouse sink configuration
├── Cargo.toml                  # Rust package definition
├── build.rs                    # Build script for Protocol Buffers
└── README.md                   # This file
```

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (1.65+)
- [Substreams CLI](https://substreams.streamingfast.io/getting-started/installing-the-cli)
- [ClickHouse](https://clickhouse.com/) instance
- [Pinax API Key](https://app.pinax.network) for accessing Substreams endpoints

## Getting Started

### Installation

1. Clone this repository:
   ```sh
   git clone https://github.com/PaulieB14/bitcoin-substreams-analytics.git
   cd bitcoin-substreams-analytics
   ```

2. Build the Substreams module:
   ```sh
   cargo build --target wasm32-unknown-unknown --release
   ```

3. Package the Substreams module:
   ```sh
   substreams pack
   ```

### Running the Substreams

1. Set up your Pinax API key:
   ```sh
   export SUBSTREAMS_API_TOKEN=your_pinax_api_key
   ```

2. Run the Substreams ClickHouse sink:
   ```sh
   substreams-sink-clickhouse run clickhouse-sink.yaml
   ```

Alternatively, you can use the provided script:
   ```sh
   chmod +x scripts/run-substreams.sh
   ./scripts/run-substreams.sh
   ```

### Setting up ClickHouse

1. Install ClickHouse locally or use a cloud instance.

2. Run the setup script to create the necessary tables:
   ```sh
   chmod +x scripts/setup-clickhouse.sh
   ./scripts/setup-clickhouse.sh --host=localhost --user=default --password=password
   ```

### Viewing the Dashboard

1. Start a simple HTTP server in the dashboard directory:
   ```sh
   cd dashboard
   python -m http.server 8000
   ```

2. Open your browser and navigate to `http://localhost:8000`

## Data Models

### Block Analytics

The system extracts the following data from Bitcoin blocks:

- Block number and hash
- Timestamp
- Size and weight
- Transaction count
- Miner identification
- Version and difficulty
- Protocol feature adoption metrics (SegWit, Taproot)

### Transaction Analytics

For each transaction, the system extracts:

- Transaction hash
- Size, weight, and virtual size
- Fee and fee rate
- Input and output counts
- Total value
- Transaction type (standard, SegWit, Taproot, etc.)
- Input and output addresses

### Address Analytics

The system tracks address activity:

- Address balance changes
- Transaction history
- Activity patterns
- UTXO management

### Mempool Analytics

Real-time mempool statistics:

- Transaction count
- Fee market analysis
- Fee rate distribution
- Mempool size trends

## Development

### Creating Custom Modules

To create your own custom Bitcoin Substreams modules:

1. Define your custom protocol buffer messages in `proto/analytics.proto`
2. Implement mapping functions in `src/lib.rs` and the appropriate mapper files
3. Update the Substreams manifest in `substreams.yaml`
4. Rebuild and repackage

### Testing

Run tests with:

```sh
cargo test
```

### Dashboard Customization

The dashboard is built with:

- HTML5, CSS3, and vanilla JavaScript
- Bootstrap 5 for styling
- ApexCharts for data visualization

To customize the dashboard:

1. Modify the HTML structure in `dashboard/index.html`
2. Update the styles in `dashboard/styles/main.css`
3. Adjust the chart configurations in `dashboard/scripts/charts.js`
4. Extend the API client in `dashboard/scripts/api.js` for new endpoints

## ClickHouse Integration

The ClickHouse sink configuration in `clickhouse-sink.yaml` defines:

- Table schemas for blocks, transactions, addresses, UTXOs, and mempool data
- Mappings from Substreams data to ClickHouse tables
- Data transformation rules

### Sample Queries

Here are some sample ClickHouse queries to get you started:

```sql
-- Get recent blocks
SELECT block_number, block_hash, timestamp, transaction_count, miner
FROM blocks
ORDER BY block_number DESC
LIMIT 10;

-- Get fee market analysis
SELECT
    toStartOfHour(timestamp) AS hour,
    avg(fee_rate) AS avg_fee_rate,
    max(fee_rate) AS max_fee_rate,
    min(fee_rate) AS min_fee_rate
FROM transactions
WHERE timestamp >= now() - INTERVAL 1 DAY
GROUP BY hour
ORDER BY hour DESC;

-- Get address balance distribution
SELECT
    multiIf(
        balance < 1000000, '< 0.01 BTC',
        balance < 10000000, '0.01-0.1 BTC',
        balance < 100000000, '0.1-1 BTC',
        balance < 1000000000, '1-10 BTC',
        balance < 10000000000, '10-100 BTC',
        balance < 100000000000, '100-1000 BTC',
        '1000+ BTC'
    ) AS balance_range,
    count() AS address_count
FROM (
    SELECT address, max(balance) AS balance
    FROM address_activities
    GROUP BY address
)
GROUP BY balance_range
ORDER BY indexOf(['< 0.01 BTC', '0.01-0.1 BTC', '0.1-1 BTC', '1-10 BTC', '10-100 BTC', '100-1000 BTC', '1000+ BTC'], balance_range);
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- [Substreams](https://substreams.streamingfast.io/) for providing the streaming data engine
- [ClickHouse](https://clickhouse.com/) for high-performance analytics
- [Pinax Network](https://www.pinax.network/) for Bitcoin Substreams endpoints
