//! Responses

use serde::{Deserialize, Serialize};

/// Prices
#[derive(Debug, Serialize, Deserialize)]
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
