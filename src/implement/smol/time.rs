use crate::{runtime::Smol, time::*};
use futures_util::{FutureExt, future::Map};
use smol::Timer;
use std::time::Instant;

impl Sleep for Map<Timer, fn(Instant)> {
    fn after(duration: std::time::Duration) -> Self {
        Timer::after(duration).map(drop)
    }

    fn at(instant: std::time::Instant) -> Self {
        Timer::at(instant).map(drop)
    }
}

impl Interval for Timer {
    fn with_period(period: std::time::Duration) -> Self {
        Self::interval(period)
    }

    fn starting_at(start: std::time::Instant, period: std::time::Duration) -> Self {
        Self::interval_at(start, period)
    }
}

impl RuntimeTime for Smol {
    type Sleep = Map<Timer, fn(Instant)>;
    type Interval = Timer;
}
