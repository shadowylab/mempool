//! Responses

use bitcoin::Amount;
use bitcoin::address::{Address, NetworkUnchecked};
use serde::{Deserialize, Serialize};

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
#[derive(Debug, Serialize, Deserialize)]
pub struct AddressStats {
    /// Bitcoin address
    pub address: Address<NetworkUnchecked>,
    /// On-chain statistics
    pub chain_stats: TransactionStats,
    /// Mempool statistics
    pub mempool_stats: TransactionStats,
}

/// Transaction statistics for an address
#[derive(Debug, Serialize, Deserialize)]
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
