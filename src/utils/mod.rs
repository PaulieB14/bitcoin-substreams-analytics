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
