mod pb;
mod utils;
mod mappers;

use substreams::errors::Error;
use substreams_bitcoin::pb::sf::bitcoin::r#type::v1::Block;

/// Maps blocks to block metrics
#[substreams::handlers::map]
pub fn map_block_metrics(block: Block) -> Result<pb::bitcoin::analytics::v1::BlockMetricsRecord, Error> {
    let block_data = mappers::block::extract_block_metrics(&block)?;
    Ok(pb::bitcoin::analytics::v1::BlockMetricsRecord {
        table: "blocks".to_string(),
        block: Some(block_data),
    })
}

/// Maps blocks to UTXO records
#[substreams::handlers::map]
pub fn map_utxos(block: Block) -> Result<pb::bitcoin::utxo::v1::UTXORecords, Error> {
    let records = mappers::utxo::process_utxos(&block)?;
    Ok(pb::bitcoin::utxo::v1::UTXORecords {
        records,
    })
}
