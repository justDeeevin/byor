use crate::runtime::*;

#[cfg(feature = "lock")]
impl RuntimeLock for Smol {
    type Mutex<T: ?Sized> = smol::lock::Mutex<T>;
}

#[cfg(feature = "lock")]
impl RuntimeLockExt for Smol {
    type RwLock<T: ?Sized> = smol::lock::RwLock<T>;
    type Barrier = smol::lock::Barrier;
    type Semaphore = smol::lock::Semaphore;
}

#[cfg(feature = "channel")]
impl RuntimeChannels for Smol {
    type BoundedSender<T: 'static> = smol::channel::Sender<T>;
    type BoundedReceiver<T> = smol::channel::Receiver<T>;

    type UnboundedSender<T: 'static> = smol::channel::Sender<T>;
    type UnboundedReceiver<T> = smol::channel::Receiver<T>;
}

#[cfg(feature = "exec")]
impl RuntimeExecutor for Smol {
    type Executor = smol::Executor<'static>;
}
