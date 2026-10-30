use crate::{runtime::Tokio, time::*};
use futures_util::{StreamExt, stream::Map};
use std::time::{Duration, Instant};
use tokio_stream::wrappers::IntervalStream;

impl Sleep for tokio::time::Sleep {
    fn after(duration: Duration) -> Self {
        tokio::time::sleep(duration)
    }

    fn at(instant: Instant) -> Self {
        tokio::time::sleep_until(instant.into())
    }
}

impl Interval for Map<IntervalStream, fn(tokio::time::Instant) -> Instant> {
    fn with_period(period: Duration) -> Self {
        IntervalStream::new(tokio::time::interval(period)).map(Into::into)
    }

    fn starting_at(start: Instant, period: Duration) -> Self {
        IntervalStream::new(tokio::time::interval_at(start.into(), period)).map(Into::into)
    }
}

impl RuntimeTime for Tokio {
    type Sleep = tokio::time::Sleep;
    type Interval = Map<IntervalStream, fn(tokio::time::Instant) -> Instant>;
}
