mod mpsc;
#[cfg(all(feature = "channel", feature = "smol-oneshot"))]
mod oneshot;
