//! Executors for running tasks.

#[cfg(feature = "exec")]
mod multi_thread;
#[cfg(feature = "exec")]
pub use multi_thread::*;

#[cfg(feature = "local-exec")]
pub mod local;

/// A handle to a spawned task.
///
/// Dropping the handle will cancel the task. Awaiting it will wait for the task to complete.
pub trait Handle: Future {
    type Out: 'static;

    /// Drops the task _without_ canceling it.
    ///
    /// This is useful if you want a task to run in the background.
    fn detach(self);

    fn io_result(out: Self::Output) -> std::io::Result<Self::Out>;
}
