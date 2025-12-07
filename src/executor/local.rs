//! Thread-local executor.

/// A thread-local task executor that can spawn futures to be run concurrently.
pub trait Executor: Sized {
    type Handle<T: 'static>: super::Handle<T>;

    /// Spawn a future to be run by the executor.
    ///
    /// Returns a handle that can be awaited or canceled.
    fn spawn<T: 'static>(&self, future: impl Future<Output = T> + 'static) -> Self::Handle<T>;

    /// Run a future to completion.
    fn block_on<T>(&self, future: impl Future<Output = T>) -> T;

    /// Create a new executor.
    fn new() -> std::io::Result<Self>;
}

/// A runtime with a thread-local executor.
pub trait RuntimeExecutor {
    type Executor: Executor;
}
