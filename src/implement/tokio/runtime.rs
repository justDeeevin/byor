use crate::runtime::*;

#[cfg(feature = "lock")]
impl RuntimeLock for Tokio {
    type Mutex<T: ?Sized> = tokio::sync::Mutex<T>;
}

#[cfg(feature = "lock")]
impl RuntimeLockExt for Tokio {
    type RwLock<T: ?Sized> = tokio::sync::RwLock<T>;
    type Barrier = tokio::sync::Barrier;
    type Semaphore = tokio::sync::Semaphore;
}

#[cfg(feature = "channel")]
impl RuntimeChannels for Tokio {
    type BoundedSender<T: 'static> = tokio::sync::mpsc::Sender<T>;
    type BoundedReceiver<T> = tokio_stream::wrappers::ReceiverStream<T>;

    type UnboundedSender<T: 'static> = tokio::sync::mpsc::UnboundedSender<T>;
    type UnboundedReceiver<T> = tokio_stream::wrappers::UnboundedReceiverStream<T>;
}

#[cfg(feature = "channel")]
impl RuntimeChannelsExt for Tokio {
    type OneshotSender<T> = tokio::sync::oneshot::Sender<T>;
    type OneshotReceiver<T> = tokio::sync::oneshot::Receiver<T>;
}

#[cfg(feature = "exec")]
impl RuntimeExecutor for Tokio {
    type Executor = tokio::runtime::Runtime;
}
