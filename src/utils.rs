use substreams_bitcoin::pb::sf::bitcoin::type::v1::Transaction;

/// Convert a byte array to hex string
pub fn to_hex_string(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

/// Calculate the transaction virtual size (vSize)
pub fn calculate_transaction_vsize(tx: &Transaction) -> u32 {
    // This is a simplified calculation - a complete impl would parse the transaction
    // to get the actual witness data and calculate precise vSize
    let weight = tx.size as u32 * 4; // A simplified approximation
    (weight + 3) / 4 // Round up division
}

/// Calculate fee rate in satoshis per virtual byte
pub fn calculate_fee_rate(fee: u64, vsize: u32) -> f64 {
    if vsize == 0 {
        return 0.0;
    }
    fee as f64 / vsize as f64
}

/// Determine if transaction is a SegWit transaction
pub fn is_segwit_transaction(tx: &Transaction) -> bool {
    // In a real implementation, check if any input has witness data
    tx.inputs.iter().any(|input| !input.witness.is_empty())
}

/// Determine if transaction is a Taproot transaction
pub fn is_taproot_transaction(tx: &Transaction) -> bool {
    // In a real implementation, check for Taproot-specific output scripts
    // This is a placeholder logic
    false
}

/// Extract addresses from transaction inputs/outputs
pub fn extract_addresses_from_transaction(tx: &Transaction) -> (Vec<String>, Vec<String>) {
    let mut input_addresses = Vec::new();
    let mut output_addresses = Vec::new();
    
    // Extract addresses from inputs
    for input in &tx.inputs {
        if let Some(address) = extract_address_from_script(&input.script_sig) {
            input_addresses.push(address);
        }
    }
    
    // Extract addresses from outputs
    for output in &tx.outputs {
        if let Some(address) = extract_address_from_script(&output.script_pubkey) {
            output_addresses.push(address);
        }
    }
    
    (input_addresses, output_addresses)
}

/// Extract an address from a Bitcoin script
fn extract_address_from_script(script: &[u8]) -> Option<String> {
    // In a real implementation, this would decode various Bitcoin script types
    // and extract the appropriate address based on script pattern
    // This is a placeholder that returns a dummy address based on script hash
    if script.is_empty() {
        return None;
    }
    
    let script_hash = to_hex_string(&script);
    let truncated_hash = &script_hash[0..16]; // Just using first 16 chars as a sample
    Some(format!("btc1{}", truncated_hash))
}

/// Determine transaction type based on its characteristics
pub fn determine_transaction_type(tx: &Transaction) -> pb::bitcoin::analytics::v1::TransactionType {
    use pb::bitcoin::analytics::v1::TransactionType;
    
    if is_taproot_transaction(tx) {
        TransactionType::TransactionTypeTaproot
    } else if is_segwit_transaction(tx) {
        TransactionType::TransactionTypeSegwit
    } else if is_multisig_transaction(tx) {
        TransactionType::TransactionTypeMultisig
    } else if is_lightning_transaction(tx) {
        TransactionType::TransactionTypeLightning
    } else {
        TransactionType::TransactionTypeStandard
    }
}

/// Determine if transaction is a multisig transaction
fn is_multisig_transaction(tx: &Transaction) -> bool {
    // In a real implementation, check for multisig script patterns
    // This is a placeholder logic
    false
}

/// Determine if transaction might be related to Lightning Network
fn is_lightning_transaction(tx: &Transaction) -> bool {
    // In a real implementation, look for Lightning Network specific patterns
    // This is a placeholder logic
    false
}

/// Estimate miner name from coinbase transaction data
pub fn extract_miner_name(coinbase_tx: &Transaction) -> String {
    if coinbase_tx.inputs.is_empty() {
        return "Unknown".to_string();
    }
    
    // In Bitcoin, miners often include identifying information in the coinbase input
    let coinbase_input = &coinbase_tx.inputs[0];
    
    // Try to extract ASCII text from coinbase data
    let coinbase_data = &coinbase_input.script_sig;
    let miner_text = String::from_utf8_lossy(coinbase_data)
        .chars()
        .filter(|c| c.is_ascii_graphic())
        .collect::<String>();
    
    // Check for known mining pool signatures
    if miner_text.contains("Bitfury") {
        return "Bitfury".to_string();
    } else if miner_text.contains("AntPool") {
        return "AntPool".to_string();
    } else if miner_text.contains("F2Pool") {
        return "F2Pool".to_string();
    } else if miner_text.contains("Poolin") {
        return "Poolin".to_string();
    } else if miner_text.contains("SlushPool") {
        return "SlushPool".to_string();
    }
    
    "Unknown".to_string()
}
