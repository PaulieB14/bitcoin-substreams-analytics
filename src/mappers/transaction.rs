use crate::pb::bitcoin::analytics::v1::TransactionMetrics;
use crate::utils;
use substreams_bitcoin::pb::sf::bitcoin::r#type::v1::Transaction;
use substreams::errors::Error;

pub fn extract_transaction_metrics(tx: &Transaction, block_number: u64) -> Result<TransactionMetrics, Error> {
    // Calculate transaction metrics
    let vsize = utils::calculate_transaction_vsize(tx);
    
    let mut total_input_value = 0u64;
    let mut total_output_value = 0u64;
    
    // Check if this is a coinbase transaction - in a real implementation, we would check the transaction
    let is_coinbase = false;
    
    // Process inputs and outputs
    let input_count = 0; // Placeholder
    let output_count = 0; // Placeholder
    
    // For now, we don't have access to the output values directly
    // In a real implementation, we would need to parse the transaction data
    let total_output_value = 0;
    
    // Calculate fee (inputs - outputs)
    let fee = if is_coinbase {
        0
    } else {
        total_input_value.saturating_sub(total_output_value)
    };
    
    let fee_rate = utils::calculate_fee_rate(fee, vsize);
    
    // Determine if transaction has witness data - in a real implementation, we would check the transaction
    let has_witness = false; // Placeholder
    
    // Create TransactionMetrics
    let tx_metrics = TransactionMetrics {
        hash: utils::to_hex_string(&tx.hash),
        block_number,
        block_timestamp: block_number, // Using block number as timestamp for now
        input_count: input_count as u32,
        output_count: output_count as u32,
        fee,
        fee_rate,
        size: tx.size as u32,
        weight: tx.weight as u32,
        is_coinbase,
        version: tx.version as u32,
        has_witness,
        locktime: tx.locktime as u32,
        input_value: total_input_value,
        output_value: total_output_value,
        tx_type: if has_witness {
            if is_coinbase {
                "COINBASE".to_string()
            } else {
                "SEGWIT".to_string()
            }
        } else {
            if is_coinbase {
                "COINBASE".to_string()
            } else {
                "STANDARD".to_string()
            }
        },
    };

    Ok(tx_metrics)
}
