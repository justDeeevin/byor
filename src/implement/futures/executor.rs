use crate::{executor::*, runtime::Futures};

impl Executor for futures::executor::ThreadPool {
    type Handle<T: 'static> = futures::future::RemoteHandle<T>;

    fn spawn<T: Send + 'static, F: Future<Output = T> + Send + 'static>(
        &self,
        future: F,
    ) -> Self::Handle<T> {
        futures::task::SpawnExt::spawn_with_handle(self, future).unwrap()
    }

    fn block_on<T, F: Future<Output = T>>(&self, future: F) -> T {
        futures::executor::block_on(future)
    }

    fn new() -> std::io::Result<Self>
    where
        Self: Sized,
    {
        Ok(futures::executor::ThreadPool::new().unwrap())
    }
}

impl<T: 'static> Handle<T> for futures::future::RemoteHandle<T> {
    type Wrap<U> = U;

    fn detach(self) {
        self.forget();
    }
}

impl RuntimeExecutor for Futures {
    type Executor = futures::executor::ThreadPool;
}
