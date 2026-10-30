# byor
[![crates.io](https://img.shields.io/crates/v/byor)](https://docs.rs/byor)

Bring your own runtime!

This crate provides a set of traits that abstract over common runtime-specific functionality,
including networking, fs, and synchronization primitives. It is intended to allow for libraries
to implement complex async behavior without locking their users into a specific runtime.

Implementations are provided for [tokio](https://docs.rs/tokio), [smol](https://docs.rs/smol), and [futures](https://docs.rs/futures). The traits can be easily
implemented for other runtimes.

## "concrete" functions

Many functions take generic inputs (e.g. `impl AsRef<Path>`). While this creates a very
convenient API, it means that the returned opaque futures can't be referenced with return-type
notation, which is restricted to non-generic and lifetime-generic functions. For this reason,
most of the generic functions in byor's traits have "concrete" counterparts that take in
concrete types. These forward to the generic functions by default; they only exist to provide a
reduced signature that is compatible with RTN.

This restriction on RTN may be relaxed in a future version. However, given how long it's taking
for RTN to even be stabilized, I anticipate it will be like this for a while.
