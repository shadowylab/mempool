//! Responses

use bitcoin::address::{Address, NetworkUnchecked};
use bitcoin::{Amount, FeeRate, Weight};
use serde::{Deserialize, Serialize};

use crate::deser;

/// Prices
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Prices {
    /// Timestamp
    pub time: u64,
    /// USD
    #[serde(rename = "USD")]
    pub usd: u32,
    /// EUR
    #[serde(rename = "EUR")]
    pub eur: u32,
    /// GBP
    #[serde(rename = "GBP")]
    pub gbp: u32,
    /// CAD
    #[serde(rename = "CAD")]
    pub cad: u32,
    /// CHF
    #[serde(rename = "CHF")]
    pub chf: u32,
    /// AUD
    #[serde(rename = "AUD")]
    pub aud: u32,
    /// JPY
    #[serde(rename = "JPY")]
    pub jpy: u32,
}

/// Bitcoin difficulty adjustment information
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct DifficultyAdjustment {
    /// Progress percentage towards next difficulty adjustment
    #[serde(rename = "progressPercent")]
    pub progress_percent: f64,
    /// Difficulty change percentage
    #[serde(rename = "difficultyChange")]
    pub difficulty_change: f64,
    /// Estimated timestamp for next retarget
    #[serde(rename = "estimatedRetargetDate")]
    pub estimated_retarget_date: u64,
    /// Number of blocks remaining until retarget
    #[serde(rename = "remainingBlocks")]
    pub remaining_blocks: u32,
    /// Estimated time remaining in seconds
    #[serde(rename = "remainingTime")]
    pub remaining_time: u64,
    /// Previous retarget percentage change
    #[serde(rename = "previousRetarget")]
    pub previous_retarget: f64,
    /// Block height of next retarget
    #[serde(rename = "nextRetargetHeight")]
    pub next_retarget_height: u32,
    /// Average time between blocks
    #[serde(rename = "timeAvg")]
    pub time_avg: u64,
    /// Adjusted average time between blocks
    #[serde(rename = "adjustedTimeAvg")]
    pub adjusted_time_avg: u64,
    /// Time offset
    #[serde(rename = "timeOffset")]
    pub time_offset: i64,
}

/// Bitcoin address statistics
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct AddressStats {
    /// Bitcoin address
    pub address: Address<NetworkUnchecked>,
    /// On-chain statistics
    pub chain_stats: TransactionStats,
    /// Mempool statistics
    pub mempool_stats: TransactionStats,
}

/// Transaction statistics for an address
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct TransactionStats {
    /// Number of funded transaction outputs
    pub funded_txo_count: u32,
    /// Total amount of funded transaction outputs
    pub funded_txo_sum: Amount,
    /// Number of spent transaction outputs
    pub spent_txo_count: u32,
    /// Total amount of spent transaction outputs
    pub spent_txo_sum: Amount,
    /// Total number of transactions
    pub tx_count: u32,
}

/// Bitcoin fee recommendations in sat/vB
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct FeeRecommendations {
    /// Fastest confirmation fee (sat/vB)
    #[serde(rename = "fastestFee")]
    pub fastest_fee: u32,
    /// Fee for confirmation within 30 minutes (sat/vB)
    #[serde(rename = "halfHourFee")]
    pub half_hour_fee: u32,
    /// Fee for confirmation within 1 hour (sat/vB)
    #[serde(rename = "hourFee")]
    pub hour_fee: u32,
    /// Economy fee for slower confirmation (sat/vB)
    #[serde(rename = "economyFee")]
    pub economy_fee: u32,
    /// Minimum fee (sat/vB)
    #[serde(rename = "minimumFee")]
    pub minimum_fee: u32,
}

/// Fee histogram entry representing transactions at a specific fee rate
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct FeeHistogramEntry {
    /// Fee rate in sat/vB (converted from API)
    #[serde(with = "deser::fee_rate_serde")]
    pub fee_rate: FeeRate,
    /// Virtual size of transactions at this fee rate (vB)
    #[serde(with = "deser::weight_serde")]
    pub vsize: Weight,
}

/// Bitcoin mempool statistics
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct MempoolStats {
    /// Number of transactions in mempool
    pub count: u32,
    /// Total virtual size of all transactions in mempool (vB)
    pub vsize: Weight,
    /// Total fees of all transactions in mempool
    pub total_fee: Amount,
    /// Fee histogram distribution
    #[serde(with = "deser::fee_histogram_serde")]
    pub fee_histogram: Vec<FeeHistogramEntry>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mempool_stats_deser() {
        let json_data = r#"{"count":4364,"vsize":4407712,"total_fee":7277557,"fee_histogram":[[8.687,50041],[5.135,50041],[4.07,50231]]}"#;

        let stats: MempoolStats = serde_json::from_str(json_data).unwrap();

        // Test basic fields
        assert_eq!(stats.count, 4364);
        assert_eq!(stats.vsize.to_wu(), 4407712);
        assert_eq!(stats.total_fee.to_sat(), 7277557);

        // Test fee histogram length
        assert_eq!(stats.fee_histogram.len(), 3);

        // Test first entry
        let first_entry = &stats.fee_histogram[0];
        assert_eq!(first_entry.fee_rate.to_sat_per_kwu(), 8687); // 8.6875 * 1000
        assert_eq!(first_entry.vsize.to_vbytes_ceil(), 50041);

        // Test second entry
        let last_entry = &stats.fee_histogram[1];
        assert_eq!(last_entry.fee_rate.to_sat_per_kwu(), 5135); // 5.1351 * 1000
        assert_eq!(last_entry.vsize.to_vbytes_ceil(), 50041);

        // Test third entry
        let middle_entry = &stats.fee_histogram[2];
        assert_eq!(middle_entry.fee_rate.to_sat_per_kwu(), 4070); // 4.071 * 1000
        assert_eq!(middle_entry.vsize.to_vbytes_ceil(), 50231);

        // Serialize
        let serialized = serde_json::to_string(&stats).unwrap();
        assert_eq!(serialized, json_data);
    }

    #[test]
    fn test_fee_rate_conversion() {
        // Test that fee rate conversion works correctly
        let fee_rate_sat_per_vb = 8.6875;
        let expected_sat_per_kwu = (fee_rate_sat_per_vb * 1000.0) as u64; // 8.6875 * 1000, truncated to u64

        let fee_rate = FeeRate::from_sat_per_kwu(expected_sat_per_kwu);
        let converted_back = fee_rate.to_sat_per_kwu() as f64 / 1000.0;

        // Should be close due to precision loss from float->int->float conversion
        assert!((converted_back - 8.687).abs() < 0.001);
    }
}
