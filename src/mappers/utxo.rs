use crate::pb::bitcoin::utxo::v1::{UTXO, UTXORecord};
use crate::utils;
use substreams::errors::Error;
use substreams_bitcoin::pb::sf::bitcoin::r#type::v1::{Block, Transaction};
use std::collections::HashMap;

/// Process a block to track UTXOs
pub fn process_utxos(block: &Block) -> Result<Vec<UTXORecord>, Error> {
    let mut utxo_records = Vec::new();
    let block_height = block.height as u64;
    let block_time = block.time as u64;
    
    // Process each transaction in the block
    for tx in &block.tx {
        let tx_id = utils::to_hex_string(&tx.hash);
        
        // Process inputs (mark UTXOs as spent)
        for (vin_idx, vin) in tx.vin.iter().enumerate() {
            // Skip coinbase inputs (first input of first transaction in a block)
            if vin_idx == 0 && tx.vin.len() > 0 && tx.vin[0].txid.is_empty() {
                continue;
            }
            
            // Check if the txid is not empty
            if !vin.txid.is_empty() {
                let prev_tx_id_hex = utils::to_hex_string(&vin.txid);
                let vout_index = vin.vout as u32;
                
                // Create a spent UTXO record
                // In a real implementation, we would look up the actual UTXO data
                // For this example, we'll create a placeholder with minimal data
                let spent_utxo = UTXO {
                    tx_id: prev_tx_id_hex.clone(),
                    vout_index,
                    value: 0, // In a real implementation, this would be the actual value
                    script_type: "UNKNOWN".to_string(),
                    address: "".to_string(), // In a real implementation, this would be the actual address
                    block_height: 0, // In a real implementation, this would be the block height when created
                    block_time: 0,   // In a real implementation, this would be the block time when created
                };
                
                // Create a record to mark this UTXO as spent
                let spent_record = UTXORecord {
                    table: "bitcoin_utxos".to_string(),
                    utxo: Some(spent_utxo),
                };
                
                utxo_records.push(spent_record);
            }
        }
        
        // Process outputs (create new UTXOs)
        for (vout_idx, vout) in tx.vout.iter().enumerate() {
            if let Some(script) = &vout.script_pub_key {
                let script_bytes = hex::decode(&script.hex).unwrap_or_default();
                let script_type = utils::bitcoin_utils::parse_output_script(&script_bytes);
                let address = utils::bitcoin_utils::extract_address_from_script(&script_bytes, false)
                    .unwrap_or_default();
                
                // Create a new UTXO
                let utxo = UTXO {
                    tx_id: tx_id.clone(),
                    vout_index: vout_idx as u32,
                    value: vout.value as u64,
                    script_type,
                    address,
                    block_height,
                    block_time,
                };
                
                // Create a record for this new UTXO
                let utxo_record = UTXORecord {
                    table: "bitcoin_utxos".to_string(),
                    utxo: Some(utxo),
                };
                
                utxo_records.push(utxo_record);
            }
        }
    }
    
    Ok(utxo_records)
}

/// Calculate token balances from UTXOs
pub fn calculate_token_balances(utxos: &[UTXO]) -> HashMap<String, (u64, u32)> {
    let mut balances = HashMap::new();
    
    for utxo in utxos {
        if !utxo.address.is_empty() {
            let entry = balances.entry(utxo.address.clone()).or_insert((0, 0));
            entry.0 += utxo.value;
            entry.1 += 1;
        }
    }
    
    balances
}

/// Extract UTXOs from a transaction
pub fn extract_transaction_utxos(
    tx: &Transaction,
    block_height: u64,
    block_time: u64,
) -> Vec<UTXO> {
    let mut utxos = Vec::new();
    let tx_id = utils::to_hex_string(&tx.hash);
    
    // Process outputs to create new UTXOs
    for (vout_idx, vout) in tx.vout.iter().enumerate() {
        if let Some(script) = &vout.script_pub_key {
            let script_bytes = hex::decode(&script.hex).unwrap_or_default();
            let script_type = utils::bitcoin_utils::parse_output_script(&script_bytes);
            let address = utils::bitcoin_utils::extract_address_from_script(&script_bytes, false)
                .unwrap_or_default();
            
            // Create a new UTXO
            let utxo = UTXO {
                tx_id: tx_id.clone(),
                vout_index: vout_idx as u32,
                value: vout.value as u64,
                script_type,
                address,
                block_height,
                block_time,
            };
            
            utxos.push(utxo);
        }
    }
    
    utxos
}

/// Check if a transaction input spends a specific UTXO
pub fn is_utxo_spent(
    tx: &Transaction,
    utxo_tx_id: &str,
    utxo_vout_index: u32,
) -> bool {
    for vin in &tx.vin {
        if !vin.txid.is_empty() {
            let prev_tx_id_hex = utils::to_hex_string(&vin.txid);
            if prev_tx_id_hex == utxo_tx_id && vin.vout as u32 == utxo_vout_index {
                return true;
            }
        }
    }
    
    false
}
