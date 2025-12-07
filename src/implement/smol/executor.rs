use crate::{executor::*, runtime::Smol};

impl Executor for smol::Executor<'_> {
    type Handle<T: 'static> = smol::Task<T>;

    fn spawn<T: Send + 'static, F: Future<Output = T> + Send + 'static>(
        &self,
        future: F,
    ) -> Self::Handle<T> {
        self.spawn(future)
    }

    fn block_on<T, F: Future<Output = T>>(&self, future: F) -> T {
        smol::block_on(future)
    }

    fn new() -> std::io::Result<Self>
    where
        Self: Sized,
    {
        Ok(smol::Executor::new())
    }
}

impl<T: 'static> Handle<T> for smol::Task<T> {
    type Wrap<U> = U;

    fn detach(self) {
        self.detach();
    }
}

impl RuntimeExecutor for Smol {
    type Executor = smol::Executor<'static>;
}
