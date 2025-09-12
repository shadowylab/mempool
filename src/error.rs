//! Error

use std::fmt;

use url::ParseError;

/// Mempool error
#[derive(Debug)]
pub enum Error {
    /// URL parse error
    Url(ParseError),
    /// Reqwest error
    Reqwest(reqwest::Error),
    /// Tungstenite error
    #[cfg(feature = "ws")]
    Tungstenite(tokio_tungstenite::tungstenite::Error),
    /// JSON error
    #[cfg(feature = "ws")]
    Json(serde_json::Error),
    /// Can't forward websocket message
    #[cfg(feature = "ws")]
    CantForwardMessage,
    /// Unexpected URL scheme
    #[cfg(feature = "ws")]
    UnexpectedScheme,
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Url(e) => write!(f, "{e}"),
            Self::Reqwest(e) => write!(f, "{e}"),
            #[cfg(feature = "ws")]
            Self::Tungstenite(e) => write!(f, "{e}"),
            #[cfg(feature = "ws")]
            Self::Json(e) => write!(f, "{e}"),
            #[cfg(feature = "ws")]
            Self::CantForwardMessage => write!(f, "Can't forward websocket message"),
            #[cfg(feature = "ws")]
            Self::UnexpectedScheme => write!(f, "Unexpected URL scheme"),
        }
    }
}

impl From<ParseError> for Error {
    fn from(e: ParseError) -> Self {
        Self::Url(e)
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Self::Reqwest(e)
    }
}

#[cfg(feature = "ws")]
impl From<tokio_tungstenite::tungstenite::Error> for Error {
    fn from(e: tokio_tungstenite::tungstenite::Error) -> Self {
        Self::Tungstenite(e)
    }
}

#[cfg(feature = "ws")]
impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Self::Json(e)
    }
}
