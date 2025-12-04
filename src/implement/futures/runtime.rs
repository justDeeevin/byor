use crate::runtime::*;

#[cfg(feature = "lock")]
impl RuntimeLock for Futures {
    type Mutex<T: ?Sized> = futures::lock::Mutex<T>;
}

#[cfg(feature = "channel")]
impl RuntimeChannels for Futures {
    type BoundedSender<T: 'static> = futures::channel::mpsc::Sender<T>;
    type BoundedReceiver<T> = futures::channel::mpsc::Receiver<T>;

    type UnboundedSender<T: 'static> = futures::channel::mpsc::UnboundedSender<T>;
    type UnboundedReceiver<T> = futures::channel::mpsc::UnboundedReceiver<T>;
}

#[cfg(feature = "channel")]
impl RuntimeChannelsExt for Futures {
    type OneshotSender<T> = futures::channel::oneshot::Sender<T>;
    type OneshotReceiver<T> = futures::channel::oneshot::Receiver<T>;
}

#[cfg(feature = "exec")]
impl RuntimeExecutor for Futures {
    type Executor = futures::executor::ThreadPool;
}
