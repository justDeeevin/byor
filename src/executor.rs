//! A multi-threaded task executor that can spawn futures to be run concurrently.

pub trait Executor {
    type Handle<T: 'static>: Handle<T>;

    /// Spawn a future to be run by the executor.
    ///
    /// Returns a task that can be awaited or canceled.
    fn spawn<T: Send + 'static, F: Future<Output = T> + Send + 'static>(
        &self,
        future: F,
    ) -> Self::Handle<T>;

    /// Run a future to completion.
    fn block_on<T, F: Future<Output = T>>(&self, future: F) -> T;

    /// Create a new executor.
    fn new() -> std::io::Result<Self>
    where
        Self: Sized;
}

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

/// A runtime with an executor.
pub trait RuntimeExecutor {
    type Executor: Executor;
}
