use crate::{runtime::Smol, time::Time};

impl Time for Smol {
    fn sleep(duration: std::time::Duration) -> impl Future<Output = std::time::Instant> {
        async_io::Timer::after(duration)
    }
    fn sleep_until(deadline: std::time::Instant) -> impl Future<Output = std::time::Instant> {
        async_io::Timer::at(deadline)
    }
    fn interval(duration: std::time::Duration) -> impl futures::Stream<Item = std::time::Instant> {
        async_io::Timer::interval(duration)
    }
    fn interval_at(
        start: std::time::Instant,
        duration: std::time::Duration,
    ) -> impl futures::Stream<Item = std::time::Instant> {
        async_io::Timer::interval_at(start, duration)
    }
}
