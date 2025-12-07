use crate::{executor::local::*, runtime::Smol};

impl Executor for smol::LocalExecutor<'_> {
    type Handle<T: 'static> = smol::Task<T>;

    fn spawn<T: 'static>(&self, future: impl Future<Output = T> + 'static) -> Self::Handle<T> {
        self.spawn(future)
    }

    fn block_on<T>(&self, future: impl Future<Output = T>) -> T {
        smol::block_on(future)
    }

    fn new() -> std::io::Result<Self> {
        Ok(smol::LocalExecutor::new())
    }
}

impl RuntimeExecutor for Smol {
    type Executor = smol::LocalExecutor<'static>;
}
