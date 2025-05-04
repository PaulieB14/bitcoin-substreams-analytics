mod pb;
mod utils;

use pb::bitcoin::analytics::v1::{
    AddressActivity, BlockMetrics, DatabaseChanges, NetworkDailyMetrics, TransactionMetrics,
    BlockMetricsRecord, TransactionMetricsRecord, AddressActivityRecord, NetworkDailyMetricsRecord,
};
use substreams::{log, store, Hex};
use substreams_bitcoin::pb::sf::bitcoin::type_::v1::{Block, Transaction, TransactionInput, TransactionOutput};
use substreams_bitcoin::pb::sf::bitcoin::type_::v1::block::Header;
use utils::{bitcoin_utils, metrics_utils};
use std::collections::HashMap;

const SATOSHI_PER_BITCOIN: u64 = 100_000_000;

#[substreams::handlers::map]
fn map_block_metrics(block: Block) -> Result<BlockMetrics, substreams::errors::Error> {
    let header = match block.header {
        Some(ref header) => header,
        None => {
            log::info!("Block without header, skipping");
            return Err(substreams::errors::Error::Unexpected("Missing block header".into()));
        }
    };

    let block_reward = calculate_block_reward(block.height);
    let total_fees = calculate_total_fees(&block);
    let miner_name = identify_mining_pool(&block);
    let prev_timestamp = if block.height > 0 {
        // For real implementation, we would need to get the previous block's timestamp
        // This is simplified for the example
        header.timestamp - 600 // Assume 10 minutes for simplicity
    } else {
        header.timestamp
    };
    let block_time = header.timestamp - prev_timestamp;

    Ok(BlockMetrics {
        number: block.height,
        hash: Hex(&block.hash).to_string(),
        timestamp: header.timestamp,
        size: get_block_size(&block),
        weight: get_block_weight(&block),
        tx_count: block.transaction_count as u32,
        difficulty: get_difficulty_from_bits(header.bits),
        miner: miner_name,
        block_time: block_time as u32,
        total_fees,
        block_reward,
        version: header.version as u32,
        nonce: header.nonce,
        bits: header.bits,
    })
}

#[substreams::handlers::map]
fn map_transaction_metrics(block: Block) -> Result<Vec<TransactionMetrics>, substreams::errors::Error> {
    let block_height = block.height;
    let block_timestamp = match block.header {
        Some(ref header) => header.timestamp,
        None => {
            log::info!("Block without header, skipping");
            return Err(substreams::errors::Error::Unexpected("Missing block header".into()));
        }
    };

    let mut transaction_metrics = Vec::new();

    for (tx_index, tx) in block.transaction_data.iter().enumerate() {
        let tx_hash = Hex(&tx.hash).to_string();
        let is_coinbase = tx_index == 0;
        
        let input_value = if is_coinbase {
            calculate_block_reward(block_height) + calculate_total_fees(&block)
        } else {
            calculate_tx_input_value(tx)
        };
        
        let output_value = calculate_tx_output_value(tx);
        let fee = if is_coinbase { 0 } else { input_value - output_value };
        let size = get_tx_size(tx);
        let weight = get_tx_weight(tx);
        let fee_rate = if size > 0 && !is_coinbase {
            (fee as f64) / (size as f64)
        } else {
            0.0
        };
        
        let has_witness = tx.witness.iter().any(|w| !w.is_empty());
        let tx_type = determine_tx_type(tx);

        transaction_metrics.push(TransactionMetrics {
            hash: tx_hash,
            block_number: block_height,
            block_timestamp,
            input_count: tx.input.len() as u32,
            output_count: tx.output.len() as u32,
            fee,
            fee_rate,
            size,
            weight,
            is_coinbase,
            version: tx.version as u32,
            has_witness,
            locktime: tx.lock_time,
            input_value,
            output_value,
            tx_type,
        });
    }

    Ok(transaction_metrics)
}

#[substreams::handlers::map]
fn map_address_activity(block: Block) -> Result<Vec<AddressActivity>, substreams::errors::Error> {
    let block_height = block.height;
    let block_timestamp = match block.header {
        Some(ref header) => header.timestamp,
        None => {
            log::info!("Block without header, skipping");
            return Err(substreams::errors::Error::Unexpected("Missing block header".into()));
        }
    };

    let mut address_activities = Vec::new();

    for tx in block.transaction_data.iter() {
        let tx_hash = Hex(&tx.hash).to_string();
        
        // Process inputs (excluding coinbase)
        if tx.input.len() > 0 && !is_coinbase_tx(tx) {
            for input in tx.input.iter() {
                if let Some(address) = extract_address_from_input(input) {
                    let value = get_input_value(input);
                    let script_type = determine_script_type_from_input(input);
                    let (address_tag, address_category) = get_address_tags(&address);
                    
                    address_activities.push(AddressActivity {
                        address,
                        tx_hash: tx_hash.clone(),
                        block_number: block_height,
                        block_timestamp,
                        is_input: true,
                        value,
                        script_type,
                        address_tag,
                        address_category,
                    });
                }
            }
        }
        
        // Process outputs
        for output in tx.output.iter() {
            if let Some(address) = extract_address_from_output(output) {
                let script_type = determine_script_type_from_output(output);
                let (address_tag, address_category) = get_address_tags(&address);
                
                address_activities.push(AddressActivity {
                    address,
                    tx_hash: tx_hash.clone(),
                    block_number: block_height,
                    block_timestamp,
                    is_input: false,
                    value: output.value,
                    script_type,
                    address_tag,
                    address_category,
                });
            }
        }
    }

    Ok(address_activities)
}

#[substreams::handlers::store]
fn store_daily_metrics(
    block_metrics: BlockMetrics,
    transaction_metrics: Vec<TransactionMetrics>,
    store: store::StoreAddInt64,
) {
    let date = metrics_utils::timestamp_to_date_key(block_metrics.timestamp);
    let key = format!("daily:{}", date);
    
    // Update block statistics
    store.add(format!("{}:block_count", key).as_str(), 1);
    store.add(format!("{}:tx_count", key).as_str(), block_metrics.tx_count as i64);
    store.add(format!("{}:block_time_sum", key).as_str(), block_metrics.block_time as i64);
    store.add(format!("{}:block_size_sum", key).as_str(), block_metrics.size as i64);
    store.add(format!("{}:total_fees", key).as_str(), block_metrics.total_fees as i64);
    
    // Calculate difficulty as sum of log(difficulty) for geometric mean
    store.add(format!("{}:difficulty_log_sum", key).as_str(), 
        (block_metrics.difficulty.ln() * 1_000_000.0) as i64);
    
    // Create sets of unique addresses for daily active address count
    let mut addresses = Vec::new();
    let mut segwit_tx_count = 0;
    let mut taproot_tx_count = 0;
    
    for tx in transaction_metrics {
        // Count SegWit and Taproot transactions
        if tx.has_witness {
            segwit_tx_count += 1;
        }
        
        if tx.tx_type == "P2TR" {
            taproot_tx_count += 1;
        }
        
        // Accumulate total volume
        if !tx.is_coinbase {
            store.add(format!("{}:volume", key).as_str(), tx.output_value as i64);
        }
    }
    
    // Store SegWit and Taproot counts
    store.add(format!("{}:segwit_tx_count", key).as_str(), segwit_tx_count);
    store.add(format!("{}:taproot_tx_count", key).as_str(), taproot_tx_count);
}

#[substreams::handlers::map]
fn map_aggregate_daily_metrics(deltas: store::Deltas<i64>) -> Result<Vec<NetworkDailyMetrics>, substreams::errors::Error> {
    let mut daily_stats: HashMap<String, NetworkDailyMetrics> = HashMap::new();

    for delta in deltas.deltas {
        let key = delta.key;
        let parts: Vec<&str> = key.split(':').collect();
        
        if parts.len() < 3 || !parts[0].eq("daily") {
            continue;
        }
        
        let date_str = parts[1];
        let date = date_str.parse::<u64>().unwrap_or(0);
        if date == 0 {
            continue;
        }
        
        let metric_type = parts[2];
        let value = delta.new_value;
        
        let entry = daily_stats.entry(date_str.to_string()).or_insert_with(|| NetworkDailyMetrics {
            date,
            avg_block_time: 0.0,
            total_tx_count: 0,
            total_tx_volume: 0,
            avg_block_size: 0,
            avg_tx_per_block: 0.0,
            avg_fee_rate: 0.0,
            mempool_tx_count: 0,  // Not tracked in this example
            segwit_tx_percent: 0.0,
            taproot_tx_percent: 0.0,
            avg_difficulty: 0.0,
            active_addresses: 0,
        });
        
        match metric_type {
            "block_count" => {
                // Get block count to use as denominator for averages
                let block_count = value;
                let block_time_sum = get_delta_value(&deltas, &format!("daily:{}:block_time_sum", date_str));
                let block_size_sum = get_delta_value(&deltas, &format!("daily:{}:block_size_sum", date_str));
                let tx_count = get_delta_value(&deltas, &format!("daily:{}:tx_count", date_str));
                
                if block_count > 0 {
                    entry.avg_block_time = block_time_sum as f64 / block_count as f64;
                    entry.avg_block_size = (block_size_sum / block_count) as u32;
                    entry.avg_tx_per_block = tx_count as f64 / block_count as f64;
                }
            },
            "tx_count" => {
                entry.total_tx_count = value as u32;
            },
            "volume" => {
                entry.total_tx_volume = value as u64;
            },
            "total_fees" => {
                let tx_count = get_delta_value(&deltas, &format!("daily:{}:tx_count", date_str));
                if tx_count > 0 {
                    entry.avg_fee_rate = value as f64 / tx_count as f64;
                }
            },
            "difficulty_log_sum" => {
                let block_count = get_delta_value(&deltas, &format!("daily:{}:block_count", date_str));
                if block_count > 0 {
                    // Convert back from log sum to geometric mean
                    entry.avg_difficulty = ((value as f64 / block_count as f64) / 1_000_000.0).exp();
                }
            },
            "segwit_tx_count" => {
                let tx_count = get_delta_value(&deltas, &format!("daily:{}:tx_count", date_str));
                if tx_count > 0 {
                    entry.segwit_tx_percent = (value as f64 / tx_count as f64) * 100.0;
                }
            },
            "taproot_tx_count" => {
                let tx_count = get_delta_value(&deltas, &format!("daily:{}:tx_count", date_str));
                if tx_count > 0 {
                    entry.taproot_tx_percent = (value as f64 / tx_count as f64) * 100.0;
                }
            },
            _ => {}
        }
    }

    Ok(daily_stats.into_values().collect())
}

#[substreams::handlers::map]
fn db_out(
    block_metrics: BlockMetrics,
    transaction_metrics: Vec<TransactionMetrics>,
    address_activities: Vec<AddressActivity>,
    daily_metrics: Vec<NetworkDailyMetrics>,
) -> Result<DatabaseChanges, substreams::errors::Error> {
    let mut changes = DatabaseChanges {
        block_metrics: Vec::new(),
        transaction_metrics: Vec::new(),
        address_activities: Vec::new(),
        daily_metrics: Vec::new(),
    };

    // Add block metrics
    changes.block_metrics.push(BlockMetricsRecord {
        table: "bitcoin_blocks".to_string(),
        block: Some(block_metrics),
    });

    // Add transaction metrics
    for tx in transaction_metrics {
        changes.transaction_metrics.push(TransactionMetricsRecord {
            table: "bitcoin_transactions".to_string(),
            transaction: Some(tx),
        });
    }

    // Add address activities
    for activity in address_activities {
        changes.address_activities.push(AddressActivityRecord {
            table: "bitcoin_addresses".to_string(),
            activity: Some(activity),
        });
    }

    // Add daily metrics
    for metrics in daily_metrics {
        changes.daily_metrics.push(NetworkDailyMetricsRecord {
            table: "bitcoin_network_daily".to_string(),
            metrics: Some(metrics),
        });
    }

    Ok(changes)
}

// Helper functions - These would be properly implemented in a real application

fn calculate_block_reward(height: u64) -> u64 {
    let halvings = height / 210_000;
    if halvings >= 64 {
        return 0;
    }
    
    // Initial reward of 50 BTC in satoshis
    let initial_reward = 50 * SATOSHI_PER_BITCOIN;
    initial_reward >> halvings
}

fn calculate_total_fees(block: &Block) -> u64 {
    if block.transaction_data.len() <= 1 {
        return 0;
    }
    
    let mut total_fees = 0;
    for (i, tx) in block.transaction_data.iter().enumerate() {
        // Skip coinbase transaction
        if i == 0 {
            continue;
        }
        
        let input_value = calculate_tx_input_value(tx);
        let output_value = calculate_tx_output_value(tx);
        
        // Fee is the difference between inputs and outputs
        if input_value > output_value {
            total_fees += input_value - output_value;
        }
    }
    
    total_fees
}

fn calculate_tx_input_value(tx: &Transaction) -> u64 {
    tx.input.iter().map(|input| input.value).sum()
}

fn calculate_tx_output_value(tx: &Transaction) -> u64 {
    tx.output.iter().map(|output| output.value).sum()
}

fn get_tx_size(tx: &Transaction) -> u32 {
    // This is a simplified size calculation
    // In a real implementation, you would need to calculate the serialized size
    let mut size = 8; // Version (4) + locktime (4)
    
    // Add input size (simplified)
    size += (tx.input.len() * 150) as u32;
    
    // Add output size (simplified)
    size += (tx.output.len() * 34) as u32;
    
    size
}

fn get_tx_weight(tx: &Transaction) -> u32 {
    // Simplified weight calculation
    // In a real implementation, this would be more complex
    // Weight = (base size) * 3 + total size
    let has_witness = tx.witness.iter().any(|w| !w.is_empty());
    
    if has_witness {
        let base_size = get_tx_size(tx);
        let witness_size = tx.witness.iter().map(|w| w.len()).sum::<usize>() as u32;
        
        base_size * 3 + base_size + witness_size
    } else {
        get_tx_size(tx) * 4
    }
}

fn get_block_size(block: &Block) -> u32 {
    // Simplified block size calculation
    let mut size = 80; // Header size
    
    for tx in &block.transaction_data {
        size += get_tx_size(tx);
    }
    
    size
}

fn get_block_weight(block: &Block) -> u32 {
    // Simplified block weight calculation
    let mut weight = 0;
    
    for tx in &block.transaction_data {
        weight += get_tx_weight(tx);
    }
    
    weight
}

fn identify_mining_pool(block: &Block) -> String {
    // In a real implementation, this would use heuristics to identify mining pools
    // based on coinbase data, addresses, etc.
    // For this example, we'll return a placeholder
    "Unknown".to_string()
}

fn get_difficulty_from_bits(bits: u32) -> f64 {
    // Convert bits to difficulty - simplified implementation
    let max_target = 0x00000000ffff0000000000000000000000000000000000000000000000000000u128;
    let target = bits_to_target(bits);
    max_target as f64 / target as f64
}

fn bits_to_target(bits: u32) -> u128 {
    let exponent = ((bits >> 24) & 0xff) as u8;
    let mantissa = bits & 0x00ffffff;
    
    if exponent <= 3 {
        (mantissa >> (8 * (3 - exponent as usize))) as u128
    } else {
        (mantissa as u128) << (8 * (exponent as usize - 3))
    }
}

fn is_coinbase_tx(tx: &Transaction) -> bool {
    if tx.input.is_empty() {
        return false;
    }
    
    // A coinbase transaction has a single input with a zeroed prev_hash
    let input = &tx.input[0];
    input.previous_output_index == 0xffffffff && input.previous_output_hash.iter().all(|&b| b == 0)
}

fn extract_address_from_input(input: &TransactionInput) -> Option<String> {
    // In a real implementation, this would extract addresses from input scripts
    // This is simplified for the example
    // You would use bitcoin-specific libraries to decode script and extract addresses
    Some(format!("input-{}", Hex(&input.previous_output_hash).to_string()))
}

fn extract_address_from_output(output: &TransactionOutput) -> Option<String> {
    // In a real implementation, this would extract addresses from output scripts
    // This is simplified for the example
    // You would use bitcoin-specific libraries to decode script and extract addresses
    Some(format!("output-{}", Hex(&output.hash).to_string()))
}

fn determine_script_type_from_input(input: &TransactionInput) -> String {
    // In a real implementation, this would determine the script type
    // Simplified for the example
    "P2PKH".to_string()
}

fn determine_script_type_from_output(output: &TransactionOutput) -> String {
    // In a real implementation, this would determine the script type
    // Simplified for the example
    "P2PKH".to_string()
}

fn determine_tx_type(tx: &Transaction) -> String {
    // In a real implementation, this would determine transaction type
    // based on inputs and outputs
    // Simplified for the example
    if tx.witness.iter().any(|w| !w.is_empty()) {
        "P2WPKH".to_string()
    } else {
        "P2PKH".to_string()
    }
}

fn get_address_tags(address: &str) -> (String, String) {
    // In a real implementation, this would query a database of known addresses
    // Simplified for the example
    ("".to_string(), "".to_string())
}

fn get_input_value(input: &TransactionInput) -> u64 {
    // In a real implementation, this would require looking up the referenced output
    // Simplified for the example
    input.value
}

fn get_delta_value(deltas: &store::Deltas<i64>, key: &str) -> i64 {
    for delta in &deltas.deltas {
        if delta.key == key {
            return delta.new_value;
        }
    }
    0
}
