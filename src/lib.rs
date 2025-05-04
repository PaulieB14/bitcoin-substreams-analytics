mod pb;
mod utils;
mod mappers;

use pb::bitcoin::analytics::v1::{Events, BlockData, TransactionData, AddressActivity, UTXOData, MemPoolStats};
use substreams::errors::Error;
use substreams_bitcoin::pb::sf::bitcoin::type::v1::{Block, Transaction};

/// Maps Bitcoin blocks to analytics events
#[substreams::handlers::map]
pub fn map_events(block: Block) -> Result<Events, Error> {
    let mut events = Events::default();
    
    // Process block-level data
    let block_data = mappers::block::extract_block_data(&block)?;
    events.blocks.push(block_data);
    
    // Process transaction-level data
    for transaction in block.transactions() {
        let tx_data = mappers::transaction::extract_transaction_data(transaction, block.height as u64)?;
        events.transactions.push(tx_data);
        
        // Process UTXO and address data from this transaction
        let (utxos, addresses) = mappers::address::extract_address_and_utxo_data(
            transaction, 
            block.height as u64, 
            block.timestamp as u64
        )?;
        
        events.utxos.extend(utxos);
        events.address_activities.extend(addresses);
    }
    
    // Add mempool statistics if available
    if let Some(mempool_stats) = mappers::mempool::extract_mempool_stats(&block)? {
        events.mempool_stats.push(mempool_stats);
    }
    
    Ok(events)
}

/// Maps blocks to a collection of aggregated block data
#[substreams::handlers::map]
pub fn map_block_stats(block: Block) -> Result<pb::bitcoin::analytics::v1::BlockStats, Error> {
    let mut stats = pb::bitcoin::analytics::v1::BlockStats::default();
    let block_data = mappers::block::extract_block_data(&block)?;
    stats.blocks.insert(block.height as u64, block_data);
    Ok(stats)
}

/// Maps transactions to a collection of aggregated transaction data
#[substreams::handlers::map]
pub fn map_transaction_stats(block: Block) -> Result<pb::bitcoin::analytics::v1::TransactionStats, Error> {
    let mut stats = pb::bitcoin::analytics::v1::TransactionStats::default();
    
    for transaction in block.transactions() {
        let tx_data = mappers::transaction::extract_transaction_data(transaction, block.height as u64)?;
        let tx_hash_hex = utils::to_hex_string(&transaction.hash);
        stats.transactions.insert(tx_hash_hex, tx_data);
    }
    
    Ok(stats)
}

/// Maps to address activity statistics
#[substreams::handlers::map]
pub fn map_address_stats(block: Block) -> Result<pb::bitcoin::analytics::v1::AddressStats, Error> {
    let mut stats = pb::bitcoin::analytics::v1::AddressStats::default();
    
    for transaction in block.transactions() {
        let (_, addresses) = mappers::address::extract_address_and_utxo_data(
            transaction, 
            block.height as u64, 
            block.timestamp as u64
        )?;
        
        for activity in addresses {
            let key = format!("{}-{}", activity.address, activity.transaction_hash);
            stats.activities.insert(key, activity);
        }
    }
    
    Ok(stats)
}

/// Maps to UTXO statistics
#[substreams::handlers::map]
pub fn map_utxo_stats(block: Block) -> Result<pb::bitcoin::analytics::v1::UTXOStats, Error> {
    let mut stats = pb::bitcoin::analytics::v1::UTXOStats::default();
    
    for transaction in block.transactions() {
        let (utxos, _) = mappers::address::extract_address_and_utxo_data(
            transaction, 
            block.height as u64, 
            block.timestamp as u64
        )?;
        
        for utxo in utxos {
            let key = format!("{}-{}", utxo.transaction_hash, utxo.output_index);
            stats.utxos.insert(key, utxo);
        }
    }
    
    Ok(stats)
}

/// Maps to mempool statistics
#[substreams::handlers::map]
pub fn map_mempool_stats(block: Block) -> Result<pb::bitcoin::analytics::v1::MempoolData, Error> {
    let mut stats = pb::bitcoin::analytics::v1::MempoolData::default();
    
    if let Some(mempool_stats) = mappers::mempool::extract_mempool_stats(&block)? {
        stats.stats.insert(block.timestamp as u64, mempool_stats);
    }
    
    Ok(stats)
}
