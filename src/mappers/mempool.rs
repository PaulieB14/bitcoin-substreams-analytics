use crate::pb::bitcoin::analytics::v1::{MemPoolStats, FeeDistribution};
use crate::utils;
use substreams::errors::Error;
use substreams_bitcoin::pb::sf::bitcoin::type::v1::Block;

pub fn extract_mempool_stats(block: &Block) -> Result<Option<MemPoolStats>, Error> {
    // In a real implementation, this would extract mempool data from block extras
    // Since the Bitcoin Substreams may not include mempool data, this is a placeholder
    // that simulates mempool statistics based on the transactions in the block
    
    if block.transactions.is_empty() {
        return Ok(None);
    }
    
    // Calculate total fee and fee rates
    let mut total_fee = 0u64;
    let mut fee_rates = Vec::new();
    
    for tx in block.transactions() {
        // Skip coinbase transactions
        if tx.inputs.len() == 1 && tx.inputs[0].outpoint.is_none() {
            continue;
        }
        
        // Calculate input value
        let mut input_value = 0u64;
        for input in &tx.inputs {
            if let Some(prev_out) = &input.outpoint {
                input_value += prev_out.value as u64;
            }
        }
        
        // Calculate output value
        let mut output_value = 0u64;
        for output in &tx.outputs {
            output_value += output.value as u64;
        }
        
        // Calculate fee
        let fee = input_value.saturating_sub(output_value);
        total_fee += fee;
        
        // Calculate fee rate
        let vsize = utils::calculate_transaction_vsize(tx);
        let fee_rate = utils::calculate_fee_rate(fee, vsize);
        fee_rates.push(fee_rate);
    }
    
    // Sort fee rates to calculate median and percentiles
    fee_rates.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    
    let median_fee_rate = if fee_rates.is_empty() {
        0.0
    } else if fee_rates.len() % 2 == 0 {
        (fee_rates[fee_rates.len() / 2 - 1] + fee_rates[fee_rates.len() / 2]) / 2.0
    } else {
        fee_rates[fee_rates.len() / 2]
    };
    
    // Calculate fee rate distribution percentiles
    let high_priority_fee_rate = if fee_rates.len() >= 10 {
        fee_rates[fee_rates.len() - fee_rates.len() / 10] // 90th percentile
    } else if !fee_rates.is_empty() {
        fee_rates[fee_rates.len() - 1] // Maximum
    } else {
        0.0
    };
    
    let medium_priority_fee_rate = median_fee_rate;
    
    let low_priority_fee_rate = if fee_rates.len() >= 10 {
        fee_rates[fee_rates.len() / 10] // 10th percentile
    } else if !fee_rates.is_empty() {
        fee_rates[0] // Minimum
    } else {
        0.0
    };
    
    // Create fee distribution object
    let fee_distribution = FeeDistribution {
        high_priority_fee_rate,
        medium_priority_fee_rate,
        low_priority_fee_rate,
    };
    
    // Create mempool stats object
    let mempool_stats = MemPoolStats {
        timestamp: block.timestamp as u64,
        transaction_count: (block.transactions.len() - 1) as u32, // Exclude coinbase
        total_fee,
        median_fee_rate,
        fee_distribution: Some(fee_distribution),
        total_mempool_size: block.size as u64, // Using block size as an approximation
    };
    
    Ok(Some(mempool_stats))
}
