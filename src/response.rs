//! Responses

use std::cmp::Ordering;
use std::collections::BTreeSet;

use bitcoin::address::{Address, NetworkUnchecked};
use bitcoin::{Amount, BlockHash, FeeRate, TxMerkleNode, Weight};
use serde::{Deserialize, Serialize};

use crate::deser;

/// Prices
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Prices {
    /// Timestamp
    #[serde(rename = "time")]
    pub timestamp: u64,
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

/// Block Info
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct BlockInfo {
    /// Block hash
    pub id: BlockHash,
    /// Block height
    pub height: u32,
    /// Block version
    pub version: u32,
    /// UNIX timestamp
    pub timestamp: u64,
    /// Number of transactions in the block
    pub tx_count: u32,
    /// Block size in bytes
    pub size: u32,
    /// Block weight
    #[serde(with = "deser::weight_serde")]
    pub weight: Weight,
    /// Merkle root hash
    pub merkle_root: TxMerkleNode,
    /// Previous block hash
    #[serde(rename = "previousblockhash")]
    pub previous_block_hash: BlockHash,
    /// Median time of the block
    #[serde(rename = "mediantime")]
    pub median_time: u64,
    /// Block nonce
    pub nonce: u32,
    /// Bits (difficulty target)
    pub bits: u32,
    /// Difficulty
    pub difficulty: f64,
}

/// Mining pool information
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct MiningPool {
    /// Pool ID
    pub id: u32,
    /// Pool name
    pub name: String,
    /// Pool slug
    pub slug: String,
    /// Miner names (if available)
    #[serde(rename = "minerNames")]
    pub miner_names: Option<Vec<String>>,
}

/// Block extras information containing detailed statistics
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct BlockExtras {
    /// Total fees in the block
    #[serde(rename = "totalFees")]
    pub total_fees: Amount,
    /// Median fee rate
    #[serde(rename = "medianFee")]
    #[serde(with = "deser::fee_rate_u64_serde")]
    pub median_fee: FeeRate,
    /// Fee rate range
    #[serde(rename = "feeRange")]
    #[serde(with = "deser::fee_rate_vec_serde")]
    pub fee_range: Vec<FeeRate>,
    /// Block reward (subsidy + fees)
    pub reward: Amount,
    /// Mining pool information
    pub pool: MiningPool,
    /// Average fee per transaction
    #[serde(rename = "avgFee")]
    pub avg_fee: Amount,
    /// Average fee rate
    #[serde(rename = "avgFeeRate")]
    #[serde(with = "deser::fee_rate_u64_serde")]
    pub avg_fee_rate: FeeRate,
    /// Raw coinbase transaction hex
    #[serde(rename = "coinbaseRaw")]
    pub coinbase_raw: String,
    /// Coinbase output address
    #[serde(rename = "coinbaseAddress")]
    pub coinbase_address: Option<String>,
    /// Coinbase output addresses
    #[serde(rename = "coinbaseAddresses")]
    pub coinbase_addresses: Option<Vec<String>>,
    /// Coinbase signature script
    #[serde(rename = "coinbaseSignature")]
    pub coinbase_signature: String,
    /// Coinbase signature ASCII representation
    #[serde(rename = "coinbaseSignatureAscii")]
    pub coinbase_signature_ascii: String,
    /// Average transaction size
    #[serde(rename = "avgTxSize")]
    pub avg_tx_size: f64,
    /// Total number of inputs
    #[serde(rename = "totalInputs")]
    pub total_inputs: u32,
    /// Total number of outputs
    #[serde(rename = "totalOutputs")]
    pub total_outputs: u32,
    /// Total output amount
    #[serde(rename = "totalOutputAmt")]
    pub total_output_amt: Amount,
    /// Median fee amount
    #[serde(rename = "medianFeeAmt")]
    pub median_fee_amt: Amount,
    /// Fee percentiles
    #[serde(rename = "feePercentiles")]
    pub fee_percentiles: Option<Vec<Amount>>,
    /// Total SegWit transactions
    #[serde(rename = "segwitTotalTxs")]
    pub segwit_total_txs: u32,
    /// Total SegWit size
    #[serde(rename = "segwitTotalSize")]
    pub segwit_total_size: u32,
    /// Total SegWit weight
    #[serde(rename = "segwitTotalWeight")]
    #[serde(with = "deser::weight_serde")]
    pub segwit_total_weight: Weight,
    /// Block header hex
    pub header: String,
    /// UTXO set change
    #[serde(rename = "utxoSetChange")]
    pub utxo_set_change: i32,
    /// UTXO set size
    #[serde(rename = "utxoSetSize")]
    pub utxo_set_size: u64,
    /// Total input amount
    #[serde(rename = "totalInputAmt")]
    pub total_input_amt: Amount,
    /// Virtual size
    #[serde(rename = "virtualSize")]
    pub virtual_size: u32,
    /// First seen timestamp
    #[serde(rename = "firstSeen")]
    pub first_seen: Option<u64>,
    /// Orphaned blocks
    pub orphans: Option<Vec<String>>,
    /// Match rate
    #[serde(rename = "matchRate")]
    pub match_rate: Option<f64>,
    /// Expected fees
    #[serde(rename = "expectedFees")]
    pub expected_fees: Option<Amount>,
    /// Expected weight
    #[serde(rename = "expectedWeight")]
    #[serde(with = "deser::optional_weight_serde")]
    pub expected_weight: Option<Weight>,
}

/// Block Info V1 with detailed extras
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct BlockInfoV1 {
    /// Block hash
    pub id: BlockHash,
    /// Block height
    pub height: u32,
    /// Block version
    pub version: u32,
    /// UNIX timestamp
    pub timestamp: u64,
    /// Bits (difficulty target)
    pub bits: u32,
    /// Block nonce
    pub nonce: u32,
    /// Difficulty
    pub difficulty: f64,
    /// Merkle root hash
    pub merkle_root: TxMerkleNode,
    /// Number of transactions in the block
    pub tx_count: u32,
    /// Block size in bytes
    pub size: u32,
    /// Block weight
    #[serde(with = "deser::weight_serde")]
    pub weight: Weight,
    /// Previous block hash
    #[serde(rename = "previousblockhash")]
    pub previous_block_hash: BlockHash,
    /// Median time of the block
    #[serde(rename = "mediantime")]
    pub median_time: u64,
    /// Block extras with detailed statistics
    pub extras: BlockExtras,
}

/// Hashrate stats entry
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct HashrateEntry {
    /// Unix timestamp
    #[serde(alias = "time")]
    pub timestamp: u64,
    /// Average hashrate
    #[serde(rename = "avgHashrate")]
    pub avg_hashrate: f64,
}

impl PartialEq for HashrateEntry {
    fn eq(&self, other: &Self) -> bool {
        self.timestamp == other.timestamp
    }
}

impl Eq for HashrateEntry {}

impl PartialOrd for HashrateEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HashrateEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.timestamp.cmp(&other.timestamp)
    }
}

/// Difficulty entry
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct DifficultyEntry {
    /// UNIX timestamp
    #[serde(alias = "time")]
    pub timestamp: u64,
    /// Block height
    pub height: u32,
    /// Difficulty
    pub difficulty: f64,
    /// Adjustment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjustment: Option<f32>,
}

impl PartialEq for DifficultyEntry {
    fn eq(&self, other: &Self) -> bool {
        self.timestamp == other.timestamp
    }
}

impl Eq for DifficultyEntry {}

impl PartialOrd for DifficultyEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DifficultyEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.timestamp.cmp(&other.timestamp)
    }
}

/// Hashrate stats
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct HashrateStats {
    /// Hashrates
    pub hashrates: BTreeSet<HashrateEntry>,
    /// Difficulty
    pub difficulty: BTreeSet<DifficultyEntry>,
    /// Current hashrate
    #[serde(rename = "currentHashrate")]
    pub current_hashrate: f64,
    /// Current difficulty
    #[serde(rename = "currentDifficulty")]
    pub current_difficulty: f64,
}

/// Bitcoin fee recommendations
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct FeeRecommendations {
    /// Fastest confirmation fee
    #[serde(rename = "fastestFee")]
    #[serde(with = "deser::fee_rate_u64_serde")]
    pub fastest_fee: FeeRate,
    /// Fee for confirmation within 30 minutes
    #[serde(rename = "halfHourFee")]
    #[serde(with = "deser::fee_rate_u64_serde")]
    pub half_hour_fee: FeeRate,
    /// Fee for confirmation within 1 hour
    #[serde(rename = "hourFee")]
    #[serde(with = "deser::fee_rate_u64_serde")]
    pub hour_fee: FeeRate,
    /// Economy fee for slower confirmation
    #[serde(rename = "economyFee")]
    #[serde(with = "deser::fee_rate_u64_serde")]
    pub economy_fee: FeeRate,
    /// Minimum fee
    #[serde(rename = "minimumFee")]
    #[serde(with = "deser::fee_rate_u64_serde")]
    pub minimum_fee: FeeRate,
}

/// Fee histogram entry representing transactions at a specific fee rate
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct FeeHistogramEntry {
    /// Fee rate (converted from API)
    #[serde(with = "deser::fee_rate_f64_serde")]
    pub fee_rate: FeeRate,
    /// Virtual size of transactions at this fee rate
    #[serde(with = "deser::weight_serde")]
    pub vsize: Weight,
}

/// Bitcoin mempool statistics
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct MempoolStats {
    /// Number of transactions in mempool
    pub count: u32,
    /// Total virtual size of all transactions in mempool
    pub vsize: Weight,
    /// Total fees of all transactions in mempool
    pub total_fee: Amount,
    /// Fee histogram distribution
    #[serde(with = "deser::fee_histogram_serde")]
    pub fee_histogram: Vec<FeeHistogramEntry>,
}

impl MempoolStats {
    /// Total size (MB) of all transactions in mempool
    pub fn size_mb(&self) -> f64 {
        let vbyte: u64 = self.vsize.to_vbytes_ceil();
        vbyte as f64 / 1_000_000.0
    }

    /// Calculate average fee rate
    pub fn avg_fee_rate(&self) -> FeeRate {
        if self.fee_histogram.is_empty() {
            return FeeRate::BROADCAST_MIN;
        }

        let fee_rate_sum: u64 = self
            .fee_histogram
            .iter()
            .map(|e| e.fee_rate.to_sat_per_kwu())
            .sum();
        let total: u64 = self.fee_histogram.len() as u64;
        let fee_rate_avg: u64 = fee_rate_sum / total;
        FeeRate::from_sat_per_kwu(fee_rate_avg)
    }
}

/// Mempool block fees
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct MempoolBlockFees {
    /// Block size
    #[serde(rename = "blockSize")]
    pub block_size: u64,
    /// Block vSize
    #[serde(rename = "blockVSize")]
    pub block_v_size: f64,
    /// Number of transactions
    #[serde(rename = "nTx")]
    pub n_tx: u64,
    /// Total fees
    #[serde(rename = "totalFees")]
    pub total_fees: Amount,
    /// Median fee rate
    #[serde(rename = "medianFee")]
    #[serde(with = "deser::fee_rate_f64_serde")]
    pub median_fee: FeeRate,
    /// Fee rate range
    #[serde(rename = "feeRange")]
    #[serde(with = "deser::fee_rate_vec_serde")]
    pub fee_range: Vec<FeeRate>,
}

/// Mempool info
#[cfg(feature = "ws")]
#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct MempoolInfo {
    /// Loaded
    pub loaded: bool,
    /// Mempool size
    pub size: usize,
    /// Bytes
    pub bytes: usize,
    /// Usage
    pub usage: usize,
    /// Total fee
    pub total_fee: f64,
    /// Max mempool
    #[serde(rename = "maxmempool")]
    pub max_mempool: usize,
    /// Mempool min fee
    #[serde(rename = "mempoolminfee")]
    pub mempool_min_fee: f64,
    /// Min relay tx fee
    #[serde(rename = "minrelaytxfee")]
    pub min_relay_tx_fee: f64,
    /// Incremental relay fee
    #[serde(rename = "incrementalrelayfee")]
    pub incremental_relay_fee: f64,
    /// Unbroadcast count
    #[serde(rename = "unbroadcastcount")]
    pub unbroadcast_count: usize,
    /// Full RBF
    #[serde(rename = "fullrbf")]
    pub full_rbf: bool,
}

/// Mempool subscription response
#[cfg(feature = "ws")]
#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct MempoolSubscriptionResponse {
    /// vByte/sec
    #[serde(rename = "vBytesPerSecond")]
    pub vbyte_per_second: Option<usize>,
    /// Mempool blocks
    #[serde(rename = "mempool-blocks")]
    pub mempool_blocks: Option<Vec<MempoolBlockFees>>,
    /// Mempool info
    #[serde(rename = "mempoolInfo")]
    pub mempool_info: Option<MempoolInfo>,
    /// Fees
    pub fees: Option<FeeRecommendations>,
    /// Difficulty adjustment
    #[serde(rename = "da")]
    pub difficulty_adjustment: Option<DifficultyAdjustment>,
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_block_info_deserialization() {
        let json_data = r#"{"id":"0000000000000000000384f28cb3b9cf4377a39cfd6c29ae9466951de38c0529","height":730000,"version":536870912,"timestamp":1648829449,"tx_count":1627,"size":1210916,"weight":3993515,"merkle_root":"efa344bcd6c0607f93b709515dd6dc5496178112d680338ebea459e3de7b4fbc","previousblockhash":"00000000000000000008b6f6fb83f8d74512ef1e0af29e642dd20daddd7d318f","mediantime":1648827418,"nonce":3580664066,"bits":386521239,"difficulty":28587155782195.14}"#;

        let block_info: BlockInfo = serde_json::from_str(json_data).unwrap();

        assert_eq!(
            block_info,
            BlockInfo {
                id: BlockHash::from_str(
                    "0000000000000000000384f28cb3b9cf4377a39cfd6c29ae9466951de38c0529"
                )
                .unwrap(),
                height: 730_000,
                version: 536870912,
                timestamp: 1648829449,
                tx_count: 1627,
                size: 1210916,
                weight: Weight::from_vb_unchecked(3993515),
                merkle_root: TxMerkleNode::from_str(
                    "efa344bcd6c0607f93b709515dd6dc5496178112d680338ebea459e3de7b4fbc"
                )
                .unwrap(),
                previous_block_hash: BlockHash::from_str(
                    "00000000000000000008b6f6fb83f8d74512ef1e0af29e642dd20daddd7d318f"
                )
                .unwrap(),
                median_time: 1648827418,
                nonce: 3580664066,
                bits: 386521239,
                difficulty: 28587155782195.14,
            }
        );
    }

    #[test]
    fn test_block_info_v1_deserialization() {
        let json = r#"{
  "id": "000000000000000015dc777b3ff2611091336355d3f0ee9766a2cf3be8e4b1ce",
  "height": 363366,
  "version": 2,
  "timestamp": 1435766771,
  "bits": 404111758,
  "nonce": 2892644888,
  "difficulty": 49402014931.22746,
  "merkle_root": "9d3cb87bf05ebae366b4262ed5f768ce8c62fc385c3886c9cb097647b04b686c",
  "tx_count": 494,
  "size": 286494,
  "weight": 1145976,
  "previousblockhash": "000000000000000010c545b6fa3ef1f7cf45a2a8760b1ee9f2e89673218207ce",
  "mediantime": 1435763435,
  "extras": {
    "totalFees": 5949764,
    "medianFee": 14,
    "feeRange": [
      0,
      0,
      1,
      14,
      38,
      48,
      261
    ],
    "reward": 2505949764,
    "pool": {
      "id": 0,
      "name": "Unknown",
      "slug": "unknown",
      "minerNames": null
    },
    "avgFee": 12068,
    "avgFeeRate": 20,
    "coinbaseRaw": "03668b050455940ee2ebbc03100000046d",
    "coinbaseAddress": "17JJ3oZyF4ShQMGukDjpMWhmooCjEvoVVB",
    "coinbaseAddresses": [
      "17JJ3oZyF4ShQMGukDjpMWhmooCjEvoVVB"
    ],
    "coinbaseSignature": "OP_DUP OP_HASH160 OP_PUSHBYTES_20 45160ea9d45f6edefef3977ac0b2cdcc29aa594a OP_EQUALVERIFY OP_CHECKSIG",
    "coinbaseSignatureAscii": "...",
    "avgTxSize": 579.57,
    "totalInputs": 1424,
    "totalOutputs": 1764,
    "totalOutputAmt": 531126071491,
    "medianFeeAmt": 10000,
    "feePercentiles": [
      0,
      735,
      10000,
      10000,
      10000,
      20000,
      300000
    ],
    "segwitTotalTxs": 0,
    "segwitTotalSize": 0,
    "segwitTotalWeight": 0,
    "header": "02000000ce0782217396e8f2e91e0b76a8a245cff7f13efab645c51000000000000000006c684bb0477609cbc986385c38fc628cce68f7d52e26b466e3ba5ef07bb83c9df30f94558e41161818426aac",
    "utxoSetChange": 340,
    "utxoSetSize": 21180314,
    "totalInputAmt": 531132021255,
    "virtualSize": 286494,
    "firstSeen": null,
    "orphans": [],
    "matchRate": null,
    "expectedFees": null,
    "expectedWeight": null
  }
}"#;
        let block_info: BlockInfoV1 = serde_json::from_str(json).unwrap();
        assert_eq!(block_info, BlockInfoV1 {
            id: BlockHash::from_str("000000000000000015dc777b3ff2611091336355d3f0ee9766a2cf3be8e4b1ce").unwrap(),
            height: 363366,
            version: 2,
            timestamp: 1435766771,
            bits: 404111758,
            nonce: 2892644888,
            difficulty: 49402014931.22746,
            merkle_root: TxMerkleNode::from_str("9d3cb87bf05ebae366b4262ed5f768ce8c62fc385c3886c9cb097647b04b686c").unwrap(),
            tx_count: 494,
            size: 286494,
            weight: Weight::from_vb_unchecked(1145976),
            previous_block_hash: BlockHash::from_str("000000000000000010c545b6fa3ef1f7cf45a2a8760b1ee9f2e89673218207ce").unwrap(),
            median_time: 1435763435,
            extras: BlockExtras {
                total_fees: Amount::from_sat(5949764),
                median_fee: FeeRate::from_sat_per_vb_unchecked(14),
                fee_range: vec![
                    FeeRate::from_sat_per_vb_unchecked(0),
                    FeeRate::from_sat_per_vb_unchecked(0),
                    FeeRate::from_sat_per_vb_unchecked(1),
                    FeeRate::from_sat_per_vb_unchecked(14),
                    FeeRate::from_sat_per_vb_unchecked(38),
                    FeeRate::from_sat_per_vb_unchecked(48),
                    FeeRate::from_sat_per_vb_unchecked(261),
                ],
                reward: Amount::from_sat(2505949764),
                pool: MiningPool {
                    id: 0,
                    name: "Unknown".to_string(),
                    slug: "unknown".to_string(),
                    miner_names: None,
                },
                avg_fee: Amount::from_sat(12068),
                avg_fee_rate: FeeRate::from_sat_per_vb_unchecked(20),
                coinbase_raw: "03668b050455940ee2ebbc03100000046d".to_string(),
                coinbase_address: Some("17JJ3oZyF4ShQMGukDjpMWhmooCjEvoVVB".to_string()),
                coinbase_addresses: Some(vec!["17JJ3oZyF4ShQMGukDjpMWhmooCjEvoVVB".to_string()]),
                coinbase_signature: "OP_DUP OP_HASH160 OP_PUSHBYTES_20 45160ea9d45f6edefef3977ac0b2cdcc29aa594a OP_EQUALVERIFY OP_CHECKSIG".to_string(),
                coinbase_signature_ascii: "...".to_string(),
                avg_tx_size: 579.57,
                total_inputs: 1424,
                total_outputs: 1764,
                total_output_amt: Amount::from_sat(531126071491),
                median_fee_amt: Amount::from_sat(10000),
                fee_percentiles: Some(vec![
                    Amount::from_sat(0),
                    Amount::from_sat(735),
                    Amount::from_sat(10000),
                    Amount::from_sat(10000),
                    Amount::from_sat(10000),
                    Amount::from_sat(20000),
                    Amount::from_sat(300000),
                ]),
                segwit_total_txs: 0,
                segwit_total_size: 0,
                segwit_total_weight: Weight::from_wu(0),
                header: "02000000ce0782217396e8f2e91e0b76a8a245cff7f13efab645c51000000000000000006c684bb0477609cbc986385c38fc628cce68f7d52e26b466e3ba5ef07bb83c9df30f94558e41161818426aac".to_string(),
                utxo_set_change: 340,
                utxo_set_size: 21180314,
                total_input_amt: Amount::from_sat(531132021255),
                virtual_size: 286494,
                first_seen: None,
                orphans: Some(vec![]),
                match_rate: None,
                expected_fees: None,
                expected_weight: None,
            },
        });
    }

    #[test]
    fn test_fee_recommendations_deserialization() {
        let json_data =
            r#"{"fastestFee":10,"halfHourFee":8,"hourFee":5,"economyFee":2,"minimumFee":1}"#;

        let fees: FeeRecommendations = serde_json::from_str(json_data).unwrap();

        assert_eq!(
            fees,
            FeeRecommendations {
                fastest_fee: FeeRate::from_sat_per_vb_unchecked(10),
                half_hour_fee: FeeRate::from_sat_per_vb_unchecked(8),
                hour_fee: FeeRate::from_sat_per_vb_unchecked(5),
                economy_fee: FeeRate::from_sat_per_vb_unchecked(2),
                minimum_fee: FeeRate::from_sat_per_vb_unchecked(1),
            }
        );
    }

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
    fn test_mempool_block_fees_deserialization() {
        let json_data = r#"{
    "blockSize": 873046,
    "blockVSize": 746096.5,
    "nTx": 863,
    "totalFees": 8875608,
    "medianFee": 10.79646017699115,
    "feeRange": [
      1,
      2.4242424242424243,
      8.107816711590296
    ]
  }"#;

        let block_fee: MempoolBlockFees = serde_json::from_str(json_data).unwrap();
        assert_eq!(block_fee.block_size, 873046);
        assert_eq!(block_fee.block_v_size, 746096.5);
        assert_eq!(block_fee.n_tx, 863);
        assert_eq!(block_fee.total_fees.to_sat(), 8875608);
        assert_eq!(block_fee.median_fee.to_sat_per_vb_floor(), 10);
        assert_eq!(block_fee.fee_range.len(), 3);
        assert_eq!(block_fee.fee_range[0].to_sat_per_vb_floor(), 1);
        assert_eq!(block_fee.fee_range[1].to_sat_per_vb_floor(), 2);
        assert_eq!(block_fee.fee_range[2].to_sat_per_vb_floor(), 8);
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

    #[test]
    fn test_mempool_size_conversion_to_mb() {
        let stats = MempoolStats {
            count: 1,
            vsize: Weight::from_vb_unchecked(2345565),
            total_fee: Amount::from_sat(1000),
            fee_histogram: vec![],
        };

        assert_eq!(stats.size_mb(), 2.345565);
    }

    #[test]
    fn test_mempool_avg_fee_rate() {
        let stats = MempoolStats {
            count: 1,
            vsize: Weight::from_vb_unchecked(2345565),
            total_fee: Amount::from_sat(1000),
            fee_histogram: vec![],
        };
        assert_eq!(stats.avg_fee_rate(), FeeRate::from_sat_per_vb_unchecked(1));

        let stats = MempoolStats {
            count: 1,
            vsize: Weight::from_vb_unchecked(2345565),
            total_fee: Amount::from_sat(1000),
            fee_histogram: vec![
                FeeHistogramEntry {
                    fee_rate: FeeRate::from_sat_per_vb_unchecked(18),
                    vsize: Weight::from_vb_unchecked(1000),
                },
                FeeHistogramEntry {
                    fee_rate: FeeRate::from_sat_per_vb_unchecked(8),
                    vsize: Weight::from_vb_unchecked(1000),
                },
                FeeHistogramEntry {
                    fee_rate: FeeRate::from_sat_per_vb_unchecked(4),
                    vsize: Weight::from_vb_unchecked(1000),
                },
            ],
        };
        assert_eq!(stats.avg_fee_rate(), FeeRate::from_sat_per_vb_unchecked(10));
    }
}
