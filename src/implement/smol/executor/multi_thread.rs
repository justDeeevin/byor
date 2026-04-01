use crate::{executor::*, runtime::Smol};

impl Executor for smol::Executor<'_> {
    type Handle<T: 'static> = smol::Task<T>;

    fn spawn<T: Send + 'static>(
        &self,
        future: impl Future<Output = T> + Send + 'static,
    ) -> Self::Handle<T> {
        self.spawn(future)
    }

    fn block_on<T>(&self, future: impl Future<Output = T>) -> T {
        smol::block_on(future)
    }

    fn new() -> std::io::Result<Self> {
        Ok(smol::Executor::new())
    }
}

impl RuntimeExecutor for Smol {
    type Executor = smol::Executor<'static>;
}
