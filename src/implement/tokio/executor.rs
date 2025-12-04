use crate::executor::*;

impl Executor for tokio::runtime::Runtime {
    type TaskWrap<T> = Result<T, tokio::task::JoinError>;
    type Task<T: 'static> = tokio::task::JoinHandle<T>;

    type Mutex<T: ?Sized> = tokio::sync::Mutex<T>;

    type BoundedSender<T: 'static> = tokio::sync::mpsc::Sender<T>;
    type BoundedReceiver<T> = tokio_stream::wrappers::ReceiverStream<T>;

    type UnboundedSender<T: 'static> = tokio::sync::mpsc::UnboundedSender<T>;
    type UnboundedReceiver<T> = tokio_stream::wrappers::UnboundedReceiverStream<T>;

    fn spawn<T: Send + 'static, F: Future<Output = T> + Send + 'static>(
        &self,
        future: F,
    ) -> Self::Task<T> {
        self.spawn(future)
    }
}

impl ExecutorLockExt for tokio::runtime::Runtime {
    type RwLock<T: ?Sized> = tokio::sync::RwLock<T>;
    type Barrier = tokio::sync::Barrier;
    type Semaphore = tokio::sync::Semaphore;
}

impl ExecutorChannelExt for tokio::runtime::Runtime {
    type OneshotSender<T> = tokio::sync::oneshot::Sender<T>;
    type OneshotReceiver<T> = tokio::sync::oneshot::Receiver<T>;
}
