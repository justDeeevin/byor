use crate::{executor::local::*, runtime::Smol};

impl Executor for async_executor::LocalExecutor<'_> {
    type Handle<T: 'static> = async_executor::Task<T>;

    fn spawn<T: 'static>(&self, future: impl Future<Output = T> + 'static) -> Self::Handle<T> {
        self.spawn(future)
    }

    fn block_on<T>(&self, future: impl Future<Output = T>) -> T {
        async_io::block_on(future)
    }

    fn new() -> std::io::Result<Self> {
        Ok(async_executor::LocalExecutor::new())
    }
}

impl RuntimeExecutor for Smol {
    type Executor = async_executor::LocalExecutor<'static>;
}
