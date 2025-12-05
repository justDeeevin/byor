use crate::{executor::*, runtime::Tokio};

impl Executor for tokio::runtime::Runtime {
    type TaskWrap<T> = Result<T, tokio::task::JoinError>;
    type Task<T: 'static> = tokio::task::JoinHandle<T>;

    fn spawn<T: Send + 'static, F: Future<Output = T> + Send + 'static>(
        &self,
        future: F,
    ) -> Self::Task<T> {
        self.spawn(future)
    }

    fn block_on<T: Send + 'static, F: Future<Output = T> + Send + 'static>(&self, future: F) -> T {
        self.block_on(future)
    }

    fn new() -> std::io::Result<Self>
    where
        Self: Sized,
    {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
    }
}

impl RuntimeExecutor for Tokio {
    type Executor = tokio::runtime::Runtime;
}
