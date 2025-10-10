//! Client

use bitcoin::{Address, BlockHash};
use reqwest::{Client, Response};
use url::Url;

use crate::builder::MempoolClientBuilder;
use crate::error::Error;
use crate::response::{
    AddressStats, BlockInfo, BlockInfoV1, DifficultyAdjustment, FeeRecommendations, HashrateStats,
    MempoolBlockFees, MempoolStats, Prices,
};
#[cfg(feature = "ws")]
use crate::websocket::{self, MempoolSubscription, MempoolSubscriptionRequest};

/// Hashrate time period
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum HashratePeriod {
    /// 3 days
    #[default]
    ThreeDays,
    /// 1 month
    OneMonth,
    /// 3 months
    ThreeMonths,
    /// 6 months
    SixMonths,
    /// 1 year
    OneYear,
    /// 2 years
    TwoYears,
    /// 3 years
    ThreeYears,
    /// All time
    AllTime,
}

impl HashratePeriod {
    fn as_str(&self) -> &str {
        match self {
            Self::ThreeDays => "3d",
            Self::OneMonth => "1m",
            Self::ThreeMonths => "3m",
            Self::SixMonths => "6m",
            Self::OneYear => "1y",
            Self::TwoYears => "2y",
            Self::ThreeYears => "3y",
            Self::AllTime => "all_time",
        }
    }
}

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
    #[inline]
    pub fn new(url: Url) -> Self {
        Self::builder(url).build().expect("Failed to build client")
    }

    /// Construct a client builder
    #[inline]
    pub fn builder(url: Url) -> MempoolClientBuilder {
        MempoolClientBuilder::new(url)
    }

    /// Construct new with a custom reqwest [`Client`].
    #[inline]
    pub fn from_client(url: Url, client: Client) -> Self {
        Self { client, url }
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

    /// Get the block information
    pub async fn get_block(&self, hash: BlockHash) -> Result<BlockInfo, Error> {
        let url: Url = self
            .url
            .join("/api/block/")?
            .join(hash.to_string().as_str())?;
        let response: Response = self.client.get(url).send().await?;
        Ok(response.json().await?)
    }

    /// Get the block information (v1)
    pub async fn get_block_v1(&self, hash: BlockHash) -> Result<BlockInfoV1, Error> {
        let url: Url = self
            .url
            .join("/api/v1/block/")?
            .join(hash.to_string().as_str())?;
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
    pub async fn get_hashrate(&self, period: HashratePeriod) -> Result<HashrateStats, Error> {
        let url: Url = self
            .url
            .join("/api/v1/mining/hashrate/")?
            .join(period.as_str())?;
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

    /// Subscribe to mempool space websocket.
    ///
    /// This creates a new websocket connection!
    #[inline]
    #[cfg(feature = "ws")]
    pub async fn subscribe(
        &self,
        req: MempoolSubscriptionRequest,
    ) -> Result<MempoolSubscription, Error> {
        websocket::subscribe(&self.url, req).await
    }
}
