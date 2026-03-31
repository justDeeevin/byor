//! Provided runtime implementations.

#[cfg(feature = "impl-tokio")]
/// [`tokio`]
pub struct Tokio;

#[cfg(feature = "impl-smol")]
/// [`smol`]
pub struct Smol;

#[cfg(feature = "impl-futures")]
/// [`futures`]
pub struct Futures;
