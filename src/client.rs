//! Client

use bitcoin::Address;
use reqwest::{Client, Response};
use url::Url;

use crate::error::Error;
use crate::response::{
    AddressStats, BlockInfo, DifficultyAdjustment, FeeRecommendations, HashrateStats,
    MempoolBlockFees, MempoolStats, Prices,
};

/// Mempool Space client
#[derive(Debug, Clone)]
pub struct MempoolClient {
    url: Url,
    client: Client,
}

impl MempoolClient {
    /// Construct a new mempool client instance
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use mempoolspace::prelude::*;
    ///
    /// let url: Url = Url::parse("https://mempool.space").unwrap();
    /// let client = MempoolClient::new(url);
    /// # let _client = client;
    /// ```
    pub fn new(url: Url) -> Self {
        Self {
            url,
            client: Client::new(),
        }
    }

    /// Get details about difficulty adjustment.
    pub async fn get_difficulty_adjustment(&self) -> Result<DifficultyAdjustment, Error> {
        let url: Url = self.url.join("/api/v1/difficulty-adjustment")?;
        let response: Response = self.client.get(url).send().await?;
        Ok(response.json().await?)
    }

    /// Get bitcoin latest price denominated in main currencies.
    pub async fn get_prices(&self) -> Result<Prices, Error> {
        let url: Url = self.url.join("/api/v1/prices")?;
        let response: Response = self.client.get(url).send().await?;
        Ok(response.json().await?)
    }

    /// Get details about an address.
    pub async fn get_address(&self, address: &Address) -> Result<AddressStats, Error> {
        let url: Url = self
            .url
            .join("/api/address/")?
            .join(address.to_string().as_str())?;
        let response: Response = self.client.get(url).send().await?;
        Ok(response.json().await?)
    }

    /// Get the height of the last block.
    pub async fn get_block_tip_height(&self) -> Result<u32, Error> {
        let url: Url = self.url.join("/api/blocks/tip/height")?;
        let response: Response = self.client.get(url).send().await?;
        Ok(response.json().await?)
    }

    /// Get the details on the past 10 blocks.
    ///
    /// If `start_height` is specified, the 10 blocks before (and including) `start_height` are returned.
    pub async fn get_blocks(&self, start_height: Option<u32>) -> Result<Vec<BlockInfo>, Error> {
        let mut url: Url = self.url.join("/api/blocks")?;

        // Add start height, if any.
        if let Some(start_height) = start_height {
            url = url.join(start_height.to_string().as_str())?;
        }

        let response: Response = self.client.get(url).send().await?;
        Ok(response.json().await?)
    }

    /// Get network-wide hashrate and difficulty figures over the last 3 days.
    pub async fn get_hashrate(&self) -> Result<HashrateStats, Error> {
        let url: Url = self.url.join("/api/v1/mining/hashrate/3d")?;
        let response: Response = self.client.get(url).send().await?;
        Ok(response.json().await?)
    }

    /// Get currently suggested fees for new transactions.
    pub async fn get_recommended_fees(&self) -> Result<FeeRecommendations, Error> {
        let url: Url = self.url.join("/api/v1/fees/recommended")?;
        let response: Response = self.client.get(url).send().await?;
        Ok(response.json().await?)
    }

    /// Get current mempool backlog statistics.
    pub async fn get_mempool(&self) -> Result<MempoolStats, Error> {
        let url: Url = self.url.join("/api/mempool")?;
        let response: Response = self.client.get(url).send().await?;
        Ok(response.json().await?)
    }

    /// Get current mempool as projected blocks.
    pub async fn get_mempool_blocks_fees(&self) -> Result<Vec<MempoolBlockFees>, Error> {
        let url: Url = self.url.join("/api/v1/fees/mempool-blocks")?;
        let response: Response = self.client.get(url).send().await?;
        Ok(response.json().await?)
    }
}
