# Bitcoin Substreams Analytics

A powerful framework for extracting, processing, and visualizing Bitcoin blockchain data using Substreams technology and ClickHouse integration.

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
│   └── utils/                  # Utility modules
├── dashboard/                  # Dashboard implementation
│   ├── index.html             # Dashboard HTML
│   └── scripts/               # JavaScript for dashboard
├── scripts/                    # Helper scripts
│   ├── run-substreams.sh      # Run Substreams processing
│   └── setup-clickhouse.sh    # Set up ClickHouse database
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

### Viewing the Dashboard

1. Start a simple HTTP server in the dashboard directory:
   ```sh
   cd dashboard
   python -m http.server 8000
   ```

2. Open your browser and navigate to `http://localhost:8000`

## Development

### Creating Custom Modules

To create your own custom Bitcoin Substreams modules:

1. Define your custom protocol buffer messages in `proto/analytics.proto`
2. Implement mapping functions in `src/lib.rs`
3. Update the Substreams manifest in `substreams.yaml`
4. Rebuild and repackage

### Testing

Run tests with:

```sh
cargo test
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
