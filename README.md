# Bitcoin Substreams Analytics

A simple framework for extracting Bitcoin blockchain data using Substreams technology, with support for UTXO tracking and token balances in ClickHouse.

## Overview

This project provides a comprehensive solution for Bitcoin blockchain analytics, focusing on:

- Extraction of key metrics from Bitcoin blocks
- UTXO tracking and token balance calculation
- Integration with ClickHouse for efficient querying and analytics
- Processing using Substreams technology
- Simple and clean implementation following best practices

## Project Structure

```
bitcoin-substreams-analytics/
├── proto/                      # Protocol Buffer definitions
│   ├── analytics.proto         # Data models for block analytics
│   └── utxo.proto              # Data models for UTXO tracking
├── src/                        # Rust source code
│   ├── lib.rs                  # Main library implementation
│   ├── pb/                     # Generated Protocol Buffer code
│   ├── utils/                  # Utility functions
│   │   ├── mod.rs              # Module definitions
│   │   └── bitcoin_utils.rs    # Bitcoin-specific utilities
│   └── mappers/                # Data mapping modules
│       ├── mod.rs              # Module definitions
│       ├── block.rs            # Block data extraction
│       └── utxo.rs             # UTXO tracking implementation
├── clickhouse-bitcoin/         # Modular ClickHouse integration
│   ├── src/                    # ClickHouse-specific Rust code
│   ├── schema.sql              # ClickHouse schema definition
│   ├── sink_config.yaml        # ClickHouse sink configuration
│   ├── substreams.yaml         # ClickHouse-specific Substreams manifest
│   ├── Cargo.toml              # ClickHouse-specific dependencies
│   ├── Makefile                # ClickHouse-specific build commands
│   └── README.md               # ClickHouse integration documentation
├── substreams.yaml             # Substreams manifest
├── simple-substreams.yaml      # Simplified Substreams manifest
├── build.rs                    # Build script for protobuf generation
├── Cargo.toml                  # Rust package definition
├── Makefile                    # Build and run commands
├── clickhouse_schema.sql       # ClickHouse schema for token balances
├── sink_config.yaml            # Substreams sink configuration
├── CLICKHOUSE_SETUP.md         # ClickHouse setup documentation
└── README.md                   # This file
```

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (1.65+)
- [Substreams CLI](https://substreams.streamingfast.io/getting-started/installing-the-cli)
- [Pinax API Key](https://app.pinax.network) for accessing Substreams endpoints

## Getting Started

### Installation

1. Clone this repository:
   ```sh
   git clone https://github.com/PaulieB14/bitcoin-substreams-analytics.git
   cd bitcoin-substreams-analytics
   ```

2. Build the project (this will automatically generate Protocol Buffer code):
   ```sh
   make build
   ```

3. Package the Substreams module:
   ```sh
   make pack
   ```

### Running the Substreams

1. Set up your Pinax API key in the `.env.local` file:
   ```sh
   SUBSTREAMS_API_TOKEN=your_pinax_api_key
   ```
   The Makefile will automatically load this environment variable.

2. Run the Substreams with GUI:
   ```sh
   make gui
   ```

3. Get information about the Substreams package:
   ```sh
   make info
   ```

### Setting Up ClickHouse Integration

There are two ways to use ClickHouse with this project:

#### Option 1: Using the main project's ClickHouse integration

1. Install ClickHouse:
   ```sh
   # For Docker
   docker run -d --name clickhouse-server -p 8123:8123 -p 9000:9000 clickhouse/clickhouse-server
   ```

2. Create the ClickHouse schema:
   ```sh
   # Using the ClickHouse client
   cat clickhouse_schema.sql | clickhouse-client -h localhost
   
   # Or using the HTTP interface
   curl -X POST http://localhost:8123/ --data-binary @clickhouse_schema.sql
   ```

3. Configure the Substreams sink:
   ```sh
   # Edit the sink_config.yaml file to match your ClickHouse connection details
   
   # Run the sink (requires substreams-sink-sql)
   substreams-sink-sql run \
     pinax.firehose.xyz:443 \
     sink_config.yaml \
     substreams.spkg \
     map_utxos
   ```

#### Option 2: Using the modular clickhouse-bitcoin integration

The `clickhouse-bitcoin` directory contains a modular implementation of the ClickHouse integration that can be used independently:

1. Navigate to the clickhouse-bitcoin directory:
   ```sh
   cd clickhouse-bitcoin
   ```

2. Set up the ClickHouse schema:
   ```sh
   make setup
   ```

3. Run the integration:
   ```sh
   make run
   ```

For more detailed instructions on either approach, see the `CLICKHOUSE_SETUP.md` file or the `clickhouse-bitcoin/README.md` file.

4. Query token balances:
   ```sql
   -- Example: Get top 10 addresses by balance
   SELECT address, balance, utxo_count
   FROM bitcoin_token_balances_latest
   ORDER BY balance DESC
   LIMIT 10;
   ```

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

### UTXO Tracking

The UTXO tracking module processes Bitcoin transactions to:

- Track all unspent transaction outputs (UTXOs)
- Record when UTXOs are created and spent
- Extract Bitcoin addresses from output scripts
- Calculate token balances for each address

### ClickHouse Integration

The project includes a comprehensive ClickHouse schema for:

- Storing UTXO data
- Calculating and tracking token balances
- Maintaining address transaction history
- Generating rich lists and statistics

There are two implementations of the ClickHouse integration:

1. **Main project integration**: Uses the files in the root directory (`clickhouse_schema.sql`, `sink_config.yaml`, etc.)
2. **Modular integration**: Located in the `clickhouse-bitcoin` directory, this is a more modular implementation that can be used independently

See the `CLICKHOUSE_SETUP.md` file for detailed information on the main integration, or the `clickhouse-bitcoin/README.md` file for information on the modular integration.

## Development

### Protobuf Generation

This project uses `prost-build` to automatically generate Rust code from Protocol Buffer definitions. The generation happens during the build process via the `build.rs` script.

To manually regenerate the Protocol Buffer code:
```sh
make protogen
```

### Creating Custom Modules

To create your own custom Bitcoin Substreams modules:

1. Define your custom protocol buffer messages in `proto/analytics.proto`
2. Implement mapping functions in `src/lib.rs` and the appropriate mapper files
3. Update the Substreams manifest in `substreams.yaml`
4. Rebuild and repackage

### Testing

Run tests with:
```sh
make test
```

### Cleaning

To clean the build artifacts:
```sh
make clean
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- [Substreams](https://substreams.streamingfast.io/) for providing the streaming data engine
- [Pinax Network](https://www.pinax.network/) for Bitcoin Substreams endpoints
