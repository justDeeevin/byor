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
pub trait Handle<T: 'static>: Future<Output = Self::Wrap<T>> {
    /// A wrapper around the return type of a task.
    ///
    /// This is only used by tokio, whose [`JoinHandle`](tokio::task::JoinHandle) returns a
    /// [`Result`].
    type Wrap<U>;

    /// Drops the task _without_ canceling it.
    ///
    /// This is useful if you want a task to run in the background.
    fn detach(self);
}
