//! Provided runtime implementations.

#[cfg(feature = "tokio")]
/// [`tokio`]
pub struct Tokio;

#[cfg(feature = "smol")]
/// [`smol`]
pub struct Smol;

#[cfg(feature = "futures")]
/// [`futures`]
pub struct Futures;
