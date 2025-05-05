/// Parse an output script to determine its type
pub fn parse_output_script(script_bytes: &[u8]) -> String {
    if script_bytes.is_empty() {
        return "UNKNOWN".to_string();
    }
    
    // Simple script type detection based on common patterns
    // This is a simplified implementation and doesn't cover all script types
    
    // P2PKH: OP_DUP OP_HASH160 <pubKeyHash> OP_EQUALVERIFY OP_CHECKSIG
    if script_bytes.len() >= 25 && script_bytes[0] == 0x76 && script_bytes[1] == 0xa9 {
        return "P2PKH".to_string();
    }
    
    // P2SH: OP_HASH160 <scriptHash> OP_EQUAL
    if script_bytes.len() >= 23 && script_bytes[0] == 0xa9 {
        return "P2SH".to_string();
    }
    
    // P2WPKH: OP_0 <pubKeyHash>
    if script_bytes.len() >= 22 && script_bytes[0] == 0x00 && script_bytes[1] == 0x14 {
        return "P2WPKH".to_string();
    }
    
    // P2WSH: OP_0 <scriptHash>
    if script_bytes.len() >= 34 && script_bytes[0] == 0x00 && script_bytes[1] == 0x20 {
        return "P2WSH".to_string();
    }
    
    // P2TR: OP_1 <x-only pubkey>
    if script_bytes.len() >= 34 && script_bytes[0] == 0x51 && script_bytes[1] == 0x20 {
        return "P2TR".to_string();
    }
    
    // Multisig: OP_<M> <pubKey1> ... <pubKeyN> OP_<N> OP_CHECKMULTISIG
    if script_bytes.len() > 3 && script_bytes[script_bytes.len() - 1] == 0xae {
        return "MULTISIG".to_string();
    }
    
    // OP_RETURN: OP_RETURN <data>
    if script_bytes.len() >= 1 && script_bytes[0] == 0x6a {
        return "OP_RETURN".to_string();
    }
    
    "UNKNOWN".to_string()
}

/// Check if a transaction is a SegWit transaction
pub fn is_segwit_transaction(tx: &substreams_bitcoin::pb::sf::bitcoin::r#type::v1::Transaction) -> bool {
    // A transaction is SegWit if any of its inputs has witness data
    for vin in &tx.vin {
        if !vin.txinwitness.is_empty() {
            return true;
        }
    }
    false
}

/// Check if a transaction is a Taproot transaction
pub fn is_taproot_transaction(tx: &substreams_bitcoin::pb::sf::bitcoin::r#type::v1::Transaction) -> bool {
    // A transaction is Taproot if any of its outputs uses a P2TR script
    // P2TR scripts start with OP_1 (0x51) followed by a 32-byte x-only pubkey
    for vout in &tx.vout {
        // Check if the script_pub_key is present and is a P2TR script
        if let Some(script) = &vout.script_pub_key {
            // Check if the type field is set to "witness_v1_taproot"
            if script.r#type == "witness_v1_taproot" {
                return true;
            }
            
            // Alternatively, check the hex representation of the script
            // P2TR scripts in hex start with "51" (OP_1) followed by "20" (push 32 bytes) and then 32 bytes
            if script.hex.len() >= 68 && script.hex.starts_with("5120") {
                return true;
            }
        }
    }
    false
}

/// Extract the miner name from a coinbase transaction
pub fn extract_miner_name(_tx: &substreams_bitcoin::pb::sf::bitcoin::r#type::v1::Transaction) -> String {
    // In a real implementation, we would extract the miner name from the coinbase transaction
    // For now, return a placeholder
    "Unknown Miner".to_string()
}

/// Extract a Bitcoin address from an output script
pub fn extract_address_from_script(script_bytes: &[u8], _testnet: bool) -> Option<String> {
    // In a real implementation, this would use proper Bitcoin address encoding
    // This is a simplified placeholder that doesn't actually decode addresses
    if script_bytes.is_empty() {
        return None;
    }
    
    // Just returning a hex representation of the script for illustration
    // Real implementation would decode to actual bitcoin addresses
    Some(hex::encode(script_bytes))
}
