#[cfg(feature = "tokio")]
pub struct Tokio;

#[cfg(feature = "smol")]
pub struct Smol;

#[cfg(feature = "futures")]
pub struct Futures;
