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
}

impl RuntimeExecutor for Tokio {
    type Executor = tokio::runtime::Runtime;
}
