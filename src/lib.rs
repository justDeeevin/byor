//! Bring your own runtime!
//!
//! This crate provides a set of traits that abstract over common runtime-specific functionality,
//! including networking, fs, and synchronization primitives. It is intended to allow for libraries
//! to implement complex async behavior without locking its users into a specific runtime.
//!
//! Implementations are provided for [tokio](https://docs.rs/tokio), [smol](https://docs.rs/smol), and [futures](https://docs.rs/futures). The traits can be easily
//! implemented for other runtimes.

#[cfg(feature = "lock")]
pub mod lock;

#[cfg(feature = "channel")]
pub mod channel;

#[cfg(feature = "exec")]
pub mod executor;

#[cfg(feature = "fs")]
pub mod fs;

#[cfg(feature = "time")]
pub mod time;

#[cfg(feature = "net")]
pub mod net;

#[cfg(any(feature = "tokio", feature = "smol", feature = "futures"))]
pub mod runtime;

mod implement;
