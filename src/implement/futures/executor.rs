use crate::{executor::*, runtime::Futures};

impl Executor for futures::executor::ThreadPool {
    type TaskWrap<T> = T;
    type Task<T: 'static> = futures::future::RemoteHandle<T>;

    fn spawn<T: Send + 'static, F: Future<Output = T> + Send + 'static>(
        &self,
        future: F,
    ) -> Self::Task<T> {
        futures::task::SpawnExt::spawn_with_handle(self, future).unwrap()
    }
}

impl RuntimeExecutor for Futures {
    type Executor = futures::executor::ThreadPool;
}
