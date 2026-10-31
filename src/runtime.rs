//! Provided runtime implementations.

#[cfg(feature = "tokio")]
/// [`tokio`]
pub struct Tokio;
#[cfg(all(feature = "tokio", feature = "fd"))]
pub use crate::implement::tokio::fd::TokioFd;

#[cfg(feature = "smol")]
/// [`smol`](https://docs.rs/smol)
pub struct Smol;

#[cfg(feature = "futures")]
/// [`futures`]
pub struct Futures;
