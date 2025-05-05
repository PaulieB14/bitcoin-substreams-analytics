use crate::pb::bitcoin::analytics::v1::BlockMetrics;
use crate::utils;
use substreams_bitcoin::pb::sf::bitcoin::r#type::v1::Block;
use substreams::errors::Error;

// Constants for Bitcoin block rewards by halving epoch
const INITIAL_BLOCK_REWARD: u64 = 50_0000_0000; // 50 BTC in satoshis
const BLOCKS_PER_HALVING: u64 = 210_000;

pub fn extract_block_metrics(block: &Block) -> Result<BlockMetrics, Error> {
    // Extract miner info
    let tx_count = block.tx.len() as usize;
    
    let miner = if tx_count > 0 && !block.tx.is_empty() {
        // Get the first transaction (coinbase)
        utils::extract_miner_name(&block.tx[0])
    } else {
        "Unknown".to_string()
    };

    // Count segwit and taproot transactions
    let mut segwit_count = 0;
    let mut taproot_count = 0;
    
    for tx in &block.tx {
        if utils::is_segwit_transaction(tx) {
            segwit_count += 1;
        }
        if utils::is_taproot_transaction(tx) {
            taproot_count += 1;
        }
    }

    // Calculate segwit and taproot percentages
    let _segwit_percent = if tx_count > 0 {
        segwit_count as f64 / tx_count as f64 * 100.0
    } else {
        0.0
    };
    
    let _taproot_percent = if tx_count > 0 {
        taproot_count as f64 / tx_count as f64 * 100.0
    } else {
        0.0
    };

    // Calculate block reward based on halving schedule
    let block_height = block.height as u64;
    let halvings = block_height / BLOCKS_PER_HALVING;
    let block_reward = if halvings >= 64 {
        // After 64 halvings, the reward is effectively 0
        0
    } else {
        INITIAL_BLOCK_REWARD >> halvings
    };

    // Calculate block time (in seconds)
    // In a real implementation, we would compare with the previous block's timestamp
    // For now, we'll use a placeholder of 10 minutes (600 seconds)
    let block_time = 600;

    // Calculate total fees
    // In a real implementation, we would calculate the difference between
    // inputs and outputs for each transaction
    // For now, we'll use a placeholder
    let total_fees = 0;

    // Create BlockMetrics
    let block_metrics = BlockMetrics {
        number: block.height as u64,
        hash: utils::to_hex_string(&block.hash),
        timestamp: block.time as u64,
        size: block.size as u32,
        weight: block.weight as u32,
        tx_count: tx_count as u32,
        difficulty: block.difficulty,
        miner,
        block_time: block_time,
        total_fees: total_fees,
        block_reward: block_reward,
        version: block.version as u32,
        nonce: block.nonce as u32,
        bits: block.bits.parse::<u32>().unwrap_or(0),
    };

    Ok(block_metrics)
}
