//! mempool space rust client

#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![warn(clippy::large_futures)]
#![warn(rustdoc::bare_urls)]

pub mod builder;
pub mod client;
mod deser;
pub mod error;
pub mod prelude;
pub mod response;
