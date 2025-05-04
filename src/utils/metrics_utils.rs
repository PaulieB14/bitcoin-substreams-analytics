use chrono::{DateTime, NaiveDateTime, Utc, Datelike};
use std::collections::HashMap;

/// Convert a Unix timestamp to a date key in YYYYMMDD format
pub fn timestamp_to_date_key(timestamp: u64) -> u64 {
    let dt = DateTime::<Utc>::from_naive_utc_and_offset(
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
pub fn calculate_fee_rate_percentiles(fee_rates: &[f64], percentiles: &[f64]) -> HashMap<String, f64> {
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
        result.insert(p.to_string(), sorted_rates[idx]);
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
