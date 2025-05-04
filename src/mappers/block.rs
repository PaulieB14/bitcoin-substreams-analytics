use crate::pb::bitcoin::analytics::v1::BlockMetrics;
use crate::utils;
use substreams_bitcoin::pb::sf::bitcoin::r#type::v1::Block;
use substreams::errors::Error;

pub fn extract_block_metrics(block: &Block) -> Result<BlockMetrics, Error> {
    // Extract miner info
    let transactions = block.transactions();
    let tx_count = transactions.count();
    
    let miner = if tx_count > 0 {
        // Get the first transaction (coinbase)
        let txs = block.transactions().collect::<Vec<_>>();
        if !txs.is_empty() {
            utils::extract_miner_name(&txs[0])
        } else {
            "Unknown".to_string()
        }
    } else {
        "Unknown".to_string()
    };

    // Count segwit and taproot transactions
    let mut segwit_count = 0;
    let mut taproot_count = 0;
    
    for tx in block.transactions() {
        if utils::is_segwit_transaction(tx) {
            segwit_count += 1;
        }
        if utils::is_taproot_transaction(tx) {
            taproot_count += 1;
        }
    }

    // Calculate segwit and taproot percentages
    let segwit_percent = if tx_count > 0 {
        segwit_count as f64 / tx_count as f64 * 100.0
    } else {
        0.0
    };
    
    let taproot_percent = if tx_count > 0 {
        taproot_count as f64 / tx_count as f64 * 100.0
    } else {
        0.0
    };

    // Create BlockMetrics
    let block_metrics = BlockMetrics {
        number: block.height as u64,
        hash: utils::to_hex_string(&block.hash),
        timestamp: block.height as u64, // Using height as timestamp for now
        size: block.size as u32,
        weight: block.weight as u32,
        tx_count: tx_count as u32,
        difficulty: block.bits.parse::<f64>().unwrap_or(0.0),
        miner,
        block_time: 0, // We don't have this information
        total_fees: 0, // We don't calculate this yet
        block_reward: 0, // We don't calculate this yet
        version: block.version as u32,
        nonce: block.nonce as u32,
        bits: block.bits.parse::<u32>().unwrap_or(0),
    };

    Ok(block_metrics)
}
