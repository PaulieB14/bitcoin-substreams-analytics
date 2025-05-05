use crate::mappers::block::extract_block_metrics;
use crate::pb::bitcoin::analytics::v1::BlockMetrics;
use substreams_bitcoin::pb::sf::bitcoin::r#type::v1::{Block, Transaction, Vin, Vout};

#[test]
fn test_extract_block_metrics() {
    // Create a mock Bitcoin block
    let mut block = Block {
        height: 123456,
        hash: vec![1, 2, 3, 4, 5],
        size: 1000,
        weight: 4000,
        version: 1,
        bits: "1d00ffff".to_string(),
        nonce: 12345,
        time: 1620000000,
        difficulty: 1.0,
        transaction_count: 2,
        transactions: vec![
            // Coinbase transaction
            Transaction {
                txid: "coinbase_tx".to_string(),
                vin: vec![
                    Vin {
                        coinbase: "01020304".to_string(),
                        ..Default::default()
                    }
                ],
                vout: vec![
                    Vout {
                        value: 50.0, // 50 BTC
                        ..Default::default()
                    }
                ],
                ..Default::default()
            },
            // Regular transaction
            Transaction {
                txid: "regular_tx".to_string(),
                vin: vec![
                    Vin {
                        txinwitness: vec!["010203".to_string()], // SegWit input
                        ..Default::default()
                    }
                ],
                vout: vec![
                    Vout {
                        value: 1.0, // 1 BTC
                        script_pub_key_type: "witness_v1_taproot".to_string(),
                        ..Default::default()
                    }
                ],
                ..Default::default()
            }
        ],
        ..Default::default()
    };

    // Extract metrics
    let result = extract_block_metrics(&block);
    assert!(result.is_ok());
    
    let metrics = result.unwrap();
    
    // Verify basic block properties
    assert_eq!(metrics.number, 123456);
    assert_eq!(metrics.hash, "0102030405");
    assert_eq!(metrics.timestamp, 1620000000);
    assert_eq!(metrics.size, 1000);
    assert_eq!(metrics.weight, 4000);
    assert_eq!(metrics.tx_count, 2);
    
    // Verify calculated metrics
    assert_eq!(metrics.block_time, 600); // Our placeholder value
    assert!(metrics.block_reward > 0); // Should have a reward
}
