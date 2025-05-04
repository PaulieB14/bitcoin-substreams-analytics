pub mod metrics_utils;
pub mod bitcoin_utils;

use substreams_bitcoin::pb::sf::bitcoin::r#type::v1::Transaction;

pub fn to_hex_string<T: AsRef<[u8]>>(data: T) -> String {
    hex::encode(data.as_ref())
}

pub fn extract_miner_name(tx: &Transaction) -> String {
    // Use our implementation from bitcoin_utils
    bitcoin_utils::extract_miner_name(tx)
}

pub fn is_segwit_transaction(_tx: &Transaction) -> bool {
    // In a real implementation, we would check if the transaction has witness data
    // For now, return a placeholder
    false
}

pub fn is_taproot_transaction(_tx: &Transaction) -> bool {
    // This is a simplified check - in a real implementation, we would need to analyze the outputs
    // to determine if they use Taproot (P2TR) scripts
    false
}

pub fn calculate_transaction_vsize(tx: &Transaction) -> u32 {
    // Simple placeholder implementation
    tx.size as u32
}

pub fn calculate_fee_rate(fee: u64, vsize: u32) -> f64 {
    fee as f64 / vsize as f64
}

pub fn extract_address_from_script(script: &[u8]) -> Option<String> {
    // Use the implementation from bitcoin_utils
    bitcoin_utils::extract_address_from_script(script, false)
}
