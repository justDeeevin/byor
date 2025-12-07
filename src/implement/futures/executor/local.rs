use crate::{executor::local::*, runtime::Futures};
use futures::task::LocalSpawnExt;

impl Executor for futures::executor::LocalSpawner {
    type Handle<T: 'static> = futures::future::RemoteHandle<T>;

    fn spawn<T: 'static>(&self, future: impl Future<Output = T> + 'static) -> Self::Handle<T> {
        self.spawn_local_with_handle(future).unwrap()
    }

    fn block_on<T>(&self, future: impl Future<Output = T>) -> T {
        futures::executor::block_on(future)
    }

    fn new() -> std::io::Result<Self> {
        Ok(futures::executor::LocalPool::new().spawner())
    }
}

impl RuntimeExecutor for Futures {
    type Executor = futures::executor::LocalSpawner;
}
