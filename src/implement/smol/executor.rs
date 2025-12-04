use crate::executor::*;

impl Executor for smol::Executor<'_> {
    type TaskWrap<T> = T;
    type Task<T: 'static> = smol::Task<T>;

    type Mutex<T: ?Sized> = smol::lock::Mutex<T>;

    type BoundedSender<T: 'static> = smol::channel::Sender<T>;
    type BoundedReceiver<T> = smol::channel::Receiver<T>;

    type UnboundedSender<T: 'static> = smol::channel::Sender<T>;
    type UnboundedReceiver<T> = smol::channel::Receiver<T>;

    fn spawn<T: Send + 'static, F: Future<Output = T> + Send + 'static>(
        &self,
        future: F,
    ) -> Self::Task<T> {
        self.spawn(future)
    }
}

impl ExecutorLockExt for smol::Executor<'_> {
    type RwLock<T: ?Sized> = smol::lock::RwLock<T>;
    type Barrier = smol::lock::Barrier;
    type Semaphore = smol::lock::Semaphore;
}
