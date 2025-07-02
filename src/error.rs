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
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Url(e) => write!(f, "{e}"),
            Self::Reqwest(e) => write!(f, "{e}"),
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
