use crate::{executor::*, runtime::Tokio};
use tokio_util::task::AbortOnDropHandle;

impl Executor for tokio::runtime::Runtime {
    type Handle<T: 'static> = AbortOnDropHandle<T>;

    fn spawn<T: Send + 'static, F: Future<Output = T> + Send + 'static>(
        &self,
        future: F,
    ) -> Self::Handle<T> {
        AbortOnDropHandle::new(self.spawn(future))
    }

    fn block_on<T, F: Future<Output = T>>(&self, future: F) -> T {
        self.block_on(future)
    }

    fn new() -> std::io::Result<Self>
    where
        Self: Sized,
    {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
    }
}

impl<T: 'static> Handle<T> for AbortOnDropHandle<T> {
    type Wrap<U> = Result<U, tokio::task::JoinError>;

    fn detach(self) {
        self.detach();
    }
}

impl RuntimeExecutor for Tokio {
    type Executor = tokio::runtime::Runtime;
}
