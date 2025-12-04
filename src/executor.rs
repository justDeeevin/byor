//! A task executor that can spawn futures to be run concurrently.

pub trait Executor {
    /// A wrapper around the return type of a task.
    ///
    /// This is only used by tokio, whose [`JoinHandle`](tokio::task::JoinHandle) returns a
    /// [`Result`].
    type TaskWrap<T>;

    /// A handle to a spawned task.
    ///
    /// Dropping the handle will cancel the task. Awaiting it will wait for the task to complete.
    type Task<T: 'static>: Future<Output = Self::TaskWrap<T>>;

    /// Spawn a future to be run by the executor.
    ///
    /// Returns a task that can be awaited or canceled.
    fn spawn<T: Send + 'static, F: Future<Output = T> + Send + 'static>(
        &self,
        future: F,
    ) -> Self::Task<T>;
}

/// A runtime with an executor.
pub trait RuntimeExecutor {
    type Executor: Executor;
}
