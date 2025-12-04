pub trait Executor {
    type TaskWrap<T>;
    type Task<T: 'static>: Future<Output = Self::TaskWrap<T>>;

    /// Spawn a future to be run by the executor.
    ///
    /// Returns a task that can be awaited or canceled.
    fn spawn<T: Send + 'static, F: Future<Output = T> + Send + 'static>(
        &self,
        future: F,
    ) -> Self::Task<T>;
}

