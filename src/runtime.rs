#[cfg(feature = "channel")]
use crate::channel::{
    mpsc::{self, BoundedSender, UnboundedSender},
    oneshot,
};
#[cfg(feature = "exec")]
use crate::executor::Executor;
#[cfg(feature = "lock")]
use crate::lock::{Barrier, Mutex, RwLock, Semaphore};

#[cfg(feature = "lock")]
pub trait RuntimeLock {
    type Mutex<T: ?Sized>: Mutex<T> + ?Sized;
}

#[cfg(feature = "lock")]
pub trait RuntimeLockExt: RuntimeLock {
    type RwLock<T: ?Sized>: RwLock<T> + ?Sized;
    type Barrier: Barrier;
    type Semaphore: Semaphore;
}

#[cfg(feature = "channel")]
pub trait RuntimeChannels {
    type BoundedSender<T: 'static>: BoundedSender<T>;
    type BoundedReceiver<T>: mpsc::Receiver<T>;

    type UnboundedSender<T: 'static>: UnboundedSender<T>;
    type UnboundedReceiver<T>: mpsc::Receiver<T>;
}

#[cfg(feature = "channel")]
pub trait RuntimeChannelsExt: RuntimeChannels {
    type OneshotSender<T>: oneshot::Sender<T>;
    type OneshotReceiver<T>: oneshot::Receiver<T>;
}

#[cfg(feature = "exec")]
pub trait RuntimeExecutor {
    type Executor: Executor;
}

#[cfg(feature = "tokio")]
pub struct Tokio;

#[cfg(feature = "smol")]
pub struct Smol;

#[cfg(feature = "futures")]
pub struct Futures;
