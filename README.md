# Bitcoin Substreams Analytics

A simple framework for extracting Bitcoin blockchain data using Substreams technology.

## Overview

This project provides a basic solution for Bitcoin blockchain analytics, focusing on:

- Extraction of key metrics from Bitcoin blocks
- Processing using Substreams technology
- Optional integration with ClickHouse for analytics

## Project Structure

```
bitcoin-substreams-analytics/
├── proto/                      # Protocol Buffer definitions
│   └── analytics.proto         # Data models for analytics
├── src/                        # Rust source code
│   ├── lib.rs                  # Main library implementation
│   ├── pb/                     # Generated Protocol Buffer code
│   ├── utils/                  # Utility functions
│   │   ├── mod.rs              # Module definitions
│   │   └── bitcoin_utils.rs    # Bitcoin-specific utilities
│   └── mappers/                # Data mapping modules
│       ├── mod.rs              # Module definitions
│       └── block.rs            # Block data extraction
├── substreams.yaml             # Substreams manifest
├── simple-substreams.yaml      # Simplified Substreams manifest
├── Cargo.toml                  # Rust package definition
├── Makefile                    # Build and run commands
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

2. Generate Protocol Buffer code:
   ```sh
   make protogen
   ```

3. Build the Substreams module:
   ```sh
   make build
   ```

4. Package the Substreams module:
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

Or run specific modules:
   ```sh
   make block_metrics
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

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- [Substreams](https://substreams.streamingfast.io/) for providing the streaming data engine
- [Pinax Network](https://www.pinax.network/) for Bitcoin Substreams endpoints
