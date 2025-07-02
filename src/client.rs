//! Client

use bitcoin::Address;
use reqwest::{Client, Response};
use url::Url;

use crate::error::Error;
use crate::response::{AddressStats, DifficultyAdjustment, Prices};

/// Mempool Space client
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
}
