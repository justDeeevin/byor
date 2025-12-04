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
}

impl RuntimeExecutor for Smol {
    type Executor = smol::Executor<'static>;
}
