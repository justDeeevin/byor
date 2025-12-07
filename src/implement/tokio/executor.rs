use crate::{executor::*, runtime::Tokio};
use tokio_util::task::AbortOnDropHandle;

impl Executor for tokio::runtime::Runtime {
    type Handle<T: 'static> = AbortOnDropHandle<T>;

    fn spawn<T: Send + 'static>(
        &self,
        future: impl Future<Output = T> + Send + 'static,
    ) -> Self::Handle<T> {
        AbortOnDropHandle::new(self.spawn(future))
    }

    fn block_on<T>(&self, future: impl Future<Output = T>) -> T {
        self.block_on(future)
    }

    fn new() -> std::io::Result<Self> {
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
