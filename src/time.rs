//! Asynchronous timing.

use futures_lite::Stream;
use std::time::{Duration, Instant};

pub trait Time {
    /// Sleep for the specified duration, returning the instant when the sleep is complete.
    fn sleep(duration: Duration) -> impl Future<Output = Instant>;

    /// Sleep until the specified instant, returning the instant when the sleep is complete.
    fn sleep_until(deadline: Instant) -> impl Future<Output = Instant>;

    /// Create a stream of instants that yields with the given interval.
    fn interval(duration: Duration) -> impl Stream<Item = Instant>;

    /// Create a stream of instants that yields with the given interval, starting at the given instant.
    fn interval_at(start: Instant, duration: Duration) -> impl Stream<Item = Instant>;
}
