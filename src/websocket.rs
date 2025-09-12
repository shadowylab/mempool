//! WebSocket

use std::pin::Pin;
use std::time::Duration;

use futures_util::{SinkExt, StreamExt};
use serde::Serialize;
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;
use url::Url;

use crate::error::Error;
use crate::response::MempoolSubscriptionResponse;

const RECONNECT_DELAY: Duration = Duration::from_secs(10);

/// Live data action
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub enum LiveDataAction {
    /// Want
    #[serde(rename = "want")]
    Want,
}

/// Live data type
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub enum LiveDataType {
    /// Blocks
    #[serde(rename = "blocks")]
    Blocks,
    /// Mempool blocks
    #[serde(rename = "mempool-blocks")]
    MempoolBlocks,
    /// Live 2h chart
    #[serde(rename = "live-2h-chart")]
    Live2hChart,
    /// Stats
    #[serde(rename = "stats")]
    Stats,
}

/// Mempool subscription request
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[serde(untagged)]
pub enum MempoolSubscriptionRequest {
    /// Live data
    LiveData {
        /// Action
        action: LiveDataAction,
        /// Data
        data: Vec<LiveDataType>,
    },
}

/// Mempool subscription
pub struct MempoolSubscription {
    /// Worker future
    pub worker: Pin<Box<dyn Future<Output = ()> + Send + 'static>>,
    /// Receiver for messages
    pub receiver: UnboundedReceiver<MempoolSubscriptionResponse>,
}

fn upgrade_scheme_from_http_to_wss(url: &Url) -> Result<Url, Error> {
    match url.scheme() {
        "http" => {
            let mut url = url.clone();
            let _ = url.set_scheme("ws");
            Ok(url)
        }
        "https" => {
            let mut url = url.clone();
            let _ = url.set_scheme("wss");
            Ok(url)
        }
        _ => Err(Error::UnexpectedScheme),
    }
}

pub(crate) async fn subscribe(
    url: &Url,
    payload: MempoolSubscriptionRequest,
) -> Result<MempoolSubscription, Error> {
    let url: Url = upgrade_scheme_from_http_to_wss(url)?;
    let url: Url = url.join("/api/v1/ws")?;

    let (tx, rx) = mpsc::unbounded_channel();

    let worker = async move {
        loop {
            match connect_and_subscribe(&url, &tx, &payload).await {
                Ok(()) => tracing::warn!(
                    "Stream terminated. Reconnecting in {} seconds...",
                    RECONNECT_DELAY.as_secs()
                ),
                Err(e) => {
                    tracing::error!(error = %e, "Stream terminated with error. Reconnecting in {} seconds...", RECONNECT_DELAY.as_secs())
                }
            }

            tokio::time::sleep(RECONNECT_DELAY).await;
        }
    };

    Ok(MempoolSubscription {
        worker: Box::pin(worker),
        receiver: rx,
    })
}

async fn connect_and_subscribe(
    url: &Url,
    tx: &UnboundedSender<MempoolSubscriptionResponse>,
    payload: &MempoolSubscriptionRequest,
) -> Result<(), Error> {
    tracing::debug!("Connecting to {}", url);

    let (stream, _) = connect_async(url.as_str()).await?;

    tracing::info!("Connected to {}", url);

    // Split stream
    let (mut ws_tx, mut ws_rx) = stream.split();

    tracing::debug!("Subscribing to mempool");

    // Subscribe to mempool
    let payload: String = serde_json::to_string(&payload)?;
    ws_tx.send(Message::text(payload)).await?;

    tracing::info!("Subscribed to mempool");

    // Listen for messages
    while let Some(message) = ws_rx.next().await {
        if let Message::Text(text) = message? {
            // Parse message
            let msg: MempoolSubscriptionResponse = serde_json::from_str(&text)?;

            // Send message to receiver
            tx.send(msg).map_err(|_| Error::CantForwardMessage)?;
        }
    }

    Ok(())
}
