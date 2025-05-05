
pub mod bitcoin_utils;

use substreams_bitcoin::pb::sf::bitcoin::r#type::v1::Transaction;

pub fn to_hex_string<T: AsRef<[u8]>>(data: T) -> String {
    hex::encode(data.as_ref())
}

pub fn extract_miner_name(tx: &Transaction) -> String {
    // Use our implementation from bitcoin_utils
    bitcoin_utils::extract_miner_name(tx)
}

pub fn is_segwit_transaction(tx: &Transaction) -> bool {
    // Use our implementation from bitcoin_utils
    bitcoin_utils::is_segwit_transaction(tx)
}

pub fn is_taproot_transaction(tx: &Transaction) -> bool {
    // Use our implementation from bitcoin_utils
    bitcoin_utils::is_taproot_transaction(tx)
}
