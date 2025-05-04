use crate::pb::bitcoin::analytics::v1::{AddressActivity, UTXOData, ActivityType};
use crate::utils;
use substreams::errors::Error;
use substreams_bitcoin::pb::sf::bitcoin::type::v1::Transaction;

pub fn extract_address_and_utxo_data(
    tx: &Transaction,
    block_number: u64,
    timestamp: u64,
) -> Result<(Vec<UTXOData>, Vec<AddressActivity>), Error> {
    let mut utxos = Vec::new();
    let mut activities = Vec::new();
    
    let tx_hash = utils::to_hex_string(&tx.hash);
    let is_coinbase = tx.inputs.len() == 1 && tx.inputs[0].outpoint.is_none();
    
    // Process inputs (spending UTXOs)
    if !is_coinbase {
        for input in &tx.inputs {
            if let Some(prev_out) = &input.outpoint {
                let prev_tx_hash = utils::to_hex_string(&prev_out.hash);
                
                // Extract address from input script
                if let Some(address) = utils::extract_address_from_script(&input.script_sig) {
                    // Create a spent UTXO record
                    let utxo = UTXOData {
                        transaction_hash: prev_tx_hash.clone(),
                        output_index: prev_out.index as u32,
                        block_number,  // We don't know the creating block number here
                        value: prev_out.value as u64,
                        is_spent: true,
                        spending_transaction_hash: tx_hash.clone(),
                        spending_block_number: block_number,
                        address: address.clone(),
                        age: 0,  // We don't know the age here
                    };
                    utxos.push(utxo);
                    
                    // Create an address activity record for sending
                    let activity = AddressActivity {
                        address: address.clone(),
                        block_number,
                        transaction_hash: tx_hash.clone(),
                        r#type: ActivityType::ActivityTypeSend.into(),
                        value: prev_out.value as u64,
                        balance: 0,  // We don't track balance here
                        utxo_count: 0,  // We don't track UTXO count here
                    };
                    activities.push(activity);
                }
            }
        }
    }
    
    // Process outputs (creating UTXOs)
    for (index, output) in tx.outputs.iter().enumerate() {
        if let Some(address) = utils::extract_address_from_script(&output.script_pubkey) {
            // Create a new UTXO record
            let utxo = UTXOData {
                transaction_hash: tx_hash.clone(),
                output_index: index as u32,
                block_number,
                value: output.value as u64,
                is_spent: false,
                spending_transaction_hash: String::new(),
                spending_block_number: 0,
                address: address.clone(),
                age: 0,  // New UTXO has age 0
            };
            utxos.push(utxo);
            
            // Create an address activity record for receiving
            let activity = AddressActivity {
                address: address.clone(),
                block_number,
                transaction_hash: tx_hash.clone(),
                r#type: ActivityType::ActivityTypeReceive.into(),
                value: output.value as u64,
                balance: 0,  // We don't track balance here
                utxo_count: 0,  // We don't track UTXO count here
            };
            activities.push(activity);
        }
    }
    
    Ok((utxos, activities))
}
