use crate::{executor::*, runtime::Futures};

impl Executor for futures::executor::ThreadPool {
    type Handle<T: 'static> = futures::future::RemoteHandle<T>;

    fn spawn<T: Send + 'static>(
        &self,
        future: impl Future<Output = T> + Send + 'static,
    ) -> Self::Handle<T> {
        futures::task::SpawnExt::spawn_with_handle(self, future).unwrap()
    }

    fn block_on<T>(&self, future: impl Future<Output = T>) -> T {
        futures::executor::block_on(future)
    }

    fn new() -> std::io::Result<Self> {
        Ok(futures::executor::ThreadPool::new().unwrap())
    }
}

impl RuntimeExecutor for Futures {
    type Executor = futures::executor::ThreadPool;
}
