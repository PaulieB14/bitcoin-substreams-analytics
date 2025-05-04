use crate::pb::bitcoin::analytics::v1::TransactionData;
use crate::utils;
use substreams::errors::Error;
use substreams_bitcoin::pb::sf::bitcoin::type::v1::Transaction;

pub fn extract_transaction_data(tx: &Transaction, block_number: u64) -> Result<TransactionData, Error> {
    // Calculate virtual size
    let vsize = utils::calculate_transaction_vsize(tx);
    
    // Calculate input and output values
    let mut total_input_value = 0u64;
    let mut total_output_value = 0u64;
    
    for input in &tx.inputs {
        if let Some(prev_out) = &input.outpoint {
            total_input_value += prev_out.value as u64;
        }
    }
    
    for output in &tx.outputs {
        total_output_value += output.value as u64;
    }
    
    // Calculate fee
    // Coinbase transactions don't have a fee in the conventional sense
    let is_coinbase = tx.inputs.len() == 1 && tx.inputs[0].outpoint.is_none();
    let fee = if is_coinbase { 0 } else { total_input_value.saturating_sub(total_output_value) };
    
    // Calculate fee rate
    let fee_rate = utils::calculate_fee_rate(fee, vsize);
    
    // Determine transaction type
    let tx_type = utils::determine_transaction_type(tx);
    
    // Extract addresses
    let (input_addresses, output_addresses) = utils::extract_addresses_from_transaction(tx);
    
    // Create TransactionData object
    let tx_data = TransactionData {
        transaction_hash: utils::to_hex_string(&tx.hash),
        block_number,
        size: tx.size as u32,
        weight: tx.weight as u32,
        virtual_size: vsize,
        fee,
        fee_rate,
        input_count: tx.inputs.len() as u32,
        output_count: tx.outputs.len() as u32,
        total_input_value,
        total_output_value,
        is_coinbase,
        r#type: tx_type.into(),
        input_addresses,
        output_addresses,
    };
    
    Ok(tx_data)
}
