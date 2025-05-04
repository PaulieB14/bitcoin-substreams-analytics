pub mod bitcoin_utils {
    use substreams::Hex;
    
    /// Parse Bitcoin script to determine the output type
    pub fn parse_output_script(script_bytes: &[u8]) -> String {
        if script_bytes.is_empty() {
            return "UNKNOWN".to_string();
        }
        
        // Very simplified script analysis - real implementation would be more comprehensive
        match script_bytes[0] {
            0x76 => {
                // Likely P2PKH (OP_DUP OP_HASH160...)
                if script_bytes.len() >= 25 && script_bytes[1] == 0xa9 {
                    "P2PKH".to_string()
                } else {
                    "UNKNOWN".to_string()
                }
            },
            0xa9 => {
                // Likely P2SH (OP_HASH160...)
                if script_bytes.len() >= 23 && script_bytes.last() == Some(&0x87) {
                    "P2SH".to_string()
                } else {
                    "UNKNOWN".to_string()
                }
            },
            0x00 => {
                // Likely P2WPKH or P2WSH (SegWit v0)
                if script_bytes.len() >= 2 {
                    match script_bytes[1] {
                        0x14 => "P2WPKH".to_string(), // 20 bytes (HASH160)
                        0x20 => "P2WSH".to_string(),  // 32 bytes (SHA256)
                        _ => "UNKNOWN".to_string(),
                    }
                } else {
                    "UNKNOWN".to_string()
                }
            },
            0x51 => {
                // Likely P2TR (Taproot, SegWit v1)
                if script_bytes.len() >= 2 && script_bytes[1] == 0x20 {
                    "P2TR".to_string()
                } else {
                    "UNKNOWN".to_string()
                }
            },
            _ => "UNKNOWN".to_string(),
        }
    }
    
    /// Extract a Bitcoin address from an output script
    pub fn extract_address_from_script(script_bytes: &[u8], testnet: bool) -> Option<String> {
        // In a real implementation, this would use proper Bitcoin address encoding
        // This is a simplified placeholder that doesn't actually decode addresses
        if script_bytes.is_empty() {
            return None;
        }
        
        // Just returning a hex representation of the script for illustration
        // Real implementation would decode to actual bitcoin addresses
        Some(Hex(script_bytes).to_string())
    }
    
    /// Calculate transaction virtual size (vsize) for fee rate calculation
    pub fn calculate_tx_vsize(tx_size: u32, tx_weight: u32) -> u32 {
        (tx_weight + 3) / 4
    }
    
    /// Check if a transaction is using SegWit
    pub fn is_segwit_tx(witness_data: &[Vec<u8>]) -> bool {
        !witness_data.is_empty() && witness_data.iter().any(|w| !w.is_empty())
    }
    
    /// Identify common Bitcoin address patterns
    pub fn identify_address_type(address: &str) -> &'static str {
        // Simplified identification based on address prefix
        // Real implementation would be more comprehensive
        if address.starts_with("1") {
            "P2PKH"
        } else if address.starts_with("3") {
            "P2SH"
        } else if address.starts_with("bc1q") {
            "P2WPKH/P2WSH"
        } else if address.starts_with("bc1p") {
            "P2TR"
        } else {
            "Unknown"
        }
    }
}

pub mod metrics_utils {
    use chrono::{DateTime, NaiveDateTime, Utc};
    use std::collections::HashMap;
    
    /// Convert a Unix timestamp to a date key in YYYYMMDD format
    pub fn timestamp_to_date_key(timestamp: u64) -> u64 {
        let dt = DateTime::<Utc>::from_utc(
            NaiveDateTime::from_timestamp_opt(timestamp as i64, 0).unwrap_or_default(),
            Utc,
        );
        
        let year = dt.year() as u64;
        let month = dt.month() as u64;
        let day = dt.day() as u64;
        
        year * 10000 + month * 100 + day
    }
    
    /// Calculate the geometric mean of a set of values
    pub fn geometric_mean(values: &[f64]) -> f64 {
        if values.is_empty() {
            return 0.0;
        }
        
        let sum_of_logs: f64 = values.iter().map(|&x| x.ln()).sum();
        (sum_of_logs / values.len() as f64).exp()
    }
    
    /// Calculate percentiles for fee rate distribution
    pub fn calculate_fee_rate_percentiles(fee_rates: &[f64], percentiles: &[f64]) -> HashMap<f64, f64> {
        if fee_rates.is_empty() {
            return HashMap::new();
        }
        
        let mut sorted_rates = fee_rates.to_vec();
        sorted_rates.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        
        let mut result = HashMap::new();
        let len = sorted_rates.len();
        
        for &p in percentiles {
            let idx = (p * len as f64 / 100.0).round() as usize;
            let idx = std::cmp::min(idx, len - 1);
            result.insert(p, sorted_rates[idx]);
        }
        
        result
    }
    
    /// Format a satoshi value as a BTC string
    pub fn format_btc(satoshis: u64) -> String {
        let btc = satoshis as f64 / 100_000_000.0;
        format!("{:.8} BTC", btc)
    }
    
    /// Calculate transaction throughput (tx/second) for a block
    pub fn calculate_throughput(tx_count: u32, block_time_seconds: u32) -> f64 {
        if block_time_seconds == 0 {
            return 0.0;
        }
        
        tx_count as f64 / block_time_seconds as f64
    }
}
