use crate::{executor::*, runtime::Smol};

impl Executor for smol::Executor<'_> {
    type TaskWrap<T> = T;
    type Task<T: 'static> = smol::Task<T>;

    fn spawn<T: Send + 'static, F: Future<Output = T> + Send + 'static>(
        &self,
        future: F,
    ) -> Self::Task<T> {
        self.spawn(future)
    }

    fn block_on<T: Send + 'static, F: Future<Output = T> + Send + 'static>(&self, future: F) -> T {
        smol::block_on(future)
    }

    fn new() -> std::io::Result<Self>
    where
        Self: Sized,
    {
        Ok(smol::Executor::new())
    }
}

impl RuntimeExecutor for Smol {
    type Executor = smol::Executor<'static>;
}
