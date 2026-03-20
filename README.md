# byor
[![crates.io](https://img.shields.io/crates/v/byor)](https://docs.rs/byor)

Bring your own runtime!

This crate provides a set of traits that abstract over common runtime-specific functionality,
including networking, fs, and synchronization primitives. It is intended to allow for libraries
to implement complex async behavior without locking their users into a specific runtime.

Implementations are provided for [tokio](https://docs.rs/tokio), [smol](https://docs.rs/smol), and [futures](https://docs.rs/futures). The traits can be easily
implemented for other runtimes.
