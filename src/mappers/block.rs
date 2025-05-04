use crate::pb::bitcoin::analytics::v1::BlockData;
use crate::utils;
use substreams::errors::Error;
use substreams_bitcoin::pb::sf::bitcoin::type::v1::Block;

pub fn extract_block_data(block: &Block) -> Result<BlockData, Error> {
    // Find coinbase transaction to extract miner info
    let miner = if !block.transactions.is_empty() {
        utils::extract_miner_name(&block.transactions[0])
    } else {
        "Unknown".to_string()
    };
    
    // Count SegWit and Taproot transactions for protocol metrics
    let mut segwit_count = 0;
    let mut taproot_count = 0;
    
    for tx in block.transactions() {
        if utils::is_segwit_transaction(tx) {
            segwit_count += 1;
        }
        if utils::is_taproot_transaction(tx) {
            taproot_count += 1;
        }
    }
    
    // Calculate protocol feature adoption percentages
    let tx_count = block.transactions.len() as u32;
    let mut protocol_features = Vec::new();
    
    if tx_count > 0 {
        if segwit_count > 0 {
            let segwit_feature = crate::pb::bitcoin::analytics::v1::ProtocolFeature {
                name: "SegWit".to_string(),
                count: segwit_count,
                percentage: (segwit_count as f64 / tx_count as f64) * 100.0,
            };
            protocol_features.push(segwit_feature);
        }
        
        if taproot_count > 0 {
            let taproot_feature = crate::pb::bitcoin::analytics::v1::ProtocolFeature {
                name: "Taproot".to_string(),
                count: taproot_count,
                percentage: (taproot_count as f64 / tx_count as f64) * 100.0,
            };
            protocol_features.push(taproot_feature);
        }
    }
    
    // Create BlockData object
    let block_data = BlockData {
        block_number: block.height as u64,
        block_hash: utils::to_hex_string(&block.hash),
        timestamp: block.timestamp as u64,
        size: block.size as u32,
        weight: block.weight as u32,
        transaction_count: tx_count,
        miner,
        version: block.version as u32,
        difficulty: block.bits.to_string(), // Convert bits to difficulty
        protocol_features,
    };
    
    Ok(block_data)
}
