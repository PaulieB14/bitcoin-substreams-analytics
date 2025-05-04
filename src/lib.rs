mod pb;
mod utils;
mod mappers;

use pb::bitcoin::analytics::v1::{DatabaseChanges, BlockMetrics, TransactionMetrics, AddressActivity};
use substreams::errors::Error;
use substreams_bitcoin::pb::sf::bitcoin::r#type::v1::{Block, Transaction};

/// Maps Bitcoin blocks to analytics events
#[substreams::handlers::map]
pub fn map_events(block: Block) -> Result<DatabaseChanges, Error> {
    let mut events = DatabaseChanges::default();
    
    // Process block-level data
    let block_data = mappers::block::extract_block_metrics(&block)?;
    events.block_metrics.push(pb::bitcoin::analytics::v1::BlockMetricsRecord {
        table: "blocks".to_string(),
        block: Some(block_data),
    });
    
    // Process transaction-level data
    let transactions = block.transactions().collect::<Vec<_>>();
    for transaction in &transactions {
        let tx_data = mappers::transaction::extract_transaction_metrics(transaction, block.height as u64)?;
        events.transaction_metrics.push(pb::bitcoin::analytics::v1::TransactionMetricsRecord {
            table: "transactions".to_string(),
            transaction: Some(tx_data),
        });
        
        // Process address data from this transaction
        let addresses = mappers::address::extract_address_activities(
            transaction, 
            block.height as u64, 
            block.height as u64 // Using block height as timestamp for now
        )?;
        
        for activity in addresses {
            events.address_activities.push(pb::bitcoin::analytics::v1::AddressActivityRecord {
                table: "address_activities".to_string(),
                activity: Some(activity),
            });
        }
    }
    
    Ok(events)
}

/// Maps blocks to block metrics
#[substreams::handlers::map]
pub fn map_block_metrics(block: Block) -> Result<pb::bitcoin::analytics::v1::BlockMetricsRecord, Error> {
    let block_data = mappers::block::extract_block_metrics(&block)?;
    Ok(pb::bitcoin::analytics::v1::BlockMetricsRecord {
        table: "blocks".to_string(),
        block: Some(block_data),
    })
}

/// Maps transactions to transaction metrics - this is a helper function, not exposed as a module
fn map_transaction_metrics_helper(block: &Block) -> Result<Vec<pb::bitcoin::analytics::v1::TransactionMetricsRecord>, Error> {
    let mut records = Vec::new();
    
    let transactions = block.transactions().collect::<Vec<_>>();
    for transaction in &transactions {
        let tx_data = mappers::transaction::extract_transaction_metrics(transaction, block.height as u64)?;
        records.push(pb::bitcoin::analytics::v1::TransactionMetricsRecord {
            table: "transactions".to_string(),
            transaction: Some(tx_data),
        });
    }
    
    Ok(records)
}

/// Maps to address activity records - this is a helper function, not exposed as a module
fn map_address_activities_helper(block: &Block) -> Result<Vec<pb::bitcoin::analytics::v1::AddressActivityRecord>, Error> {
    let mut records = Vec::new();
    
    let transactions = block.transactions().collect::<Vec<_>>();
    for transaction in &transactions {
        let addresses = mappers::address::extract_address_activities(
            transaction, 
            block.height as u64, 
            block.height as u64 // Using block height as timestamp for now
        )?;
        
        for activity in addresses {
            records.push(pb::bitcoin::analytics::v1::AddressActivityRecord {
                table: "address_activities".to_string(),
                activity: Some(activity),
            });
        }
    }
    
    Ok(records)
}

/// Maps to daily network metrics
#[substreams::handlers::map]
pub fn map_daily_metrics(block: Block) -> Result<pb::bitcoin::analytics::v1::NetworkDailyMetricsRecord, Error> {
    // This is a placeholder implementation
    let date_key = utils::metrics_utils::timestamp_to_date_key(block.height as u64);
    
    let metrics = pb::bitcoin::analytics::v1::NetworkDailyMetrics {
        date: date_key,
        avg_block_time: 600.0, // 10 minutes in seconds
        total_tx_count: block.transactions().count() as u32,
        total_tx_volume: 0,
        avg_block_size: block.size as u32,
        avg_tx_per_block: block.transactions().count() as f64,
        avg_fee_rate: 0.0,
        mempool_tx_count: 0,
        segwit_tx_percent: 0.0,
        taproot_tx_percent: 0.0,
        avg_difficulty: block.bits.parse::<f64>().unwrap_or(0.0),
        active_addresses: 0,
    };
    
    Ok(pb::bitcoin::analytics::v1::NetworkDailyMetricsRecord {
        table: "daily_metrics".to_string(),
        metrics: Some(metrics),
    })
}
