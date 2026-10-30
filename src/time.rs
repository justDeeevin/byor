//! Timing.

use futures_core::Stream;
use std::time::{Duration, Instant};

/// A future that completes after a certain amount of time has passed.
pub trait Sleep: Future<Output = ()> {
    /// Creates a future that completes after the given duration of time.
    fn after(duration: Duration) -> Self;

    /// Creates a future that completes once the given instant is reached.
    fn at(instant: Instant) -> Self;
}

/// A stream of instants that yields continuously with a consistent period.
pub trait Interval: Stream<Item = Instant> {
    /// Creates a stream that yields with the given period starting immediately.
    fn with_period(period: Duration) -> Self;

    /// Creates a stream that yields with the given period starting at the given instant.
    fn starting_at(start: Instant, period: Duration) -> Self;
}

/// A runtime with timing support.
pub trait RuntimeTime {
    type Sleep: Sleep;
    type Interval: Interval;
}
