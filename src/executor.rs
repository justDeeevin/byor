use crate::{
    channel::{
        mpsc::{self, BoundedSender, UnboundedSender},
        oneshot,
    },
    lock::{Barrier, Mutex, RwLock, Semaphore},
};

pub trait Executor {
    type TaskWrap<T>;
    type Task<T: 'static>: Future<Output = Self::TaskWrap<T>>;

    type Mutex<T: ?Sized>: Mutex<T> + ?Sized;

    type BoundedSender<T: 'static>: BoundedSender<T>;
    type BoundedReceiver<T>: mpsc::Receiver<T>;

    type UnboundedSender<T: 'static>: UnboundedSender<T>;
    type UnboundedReceiver<T>: mpsc::Receiver<T>;

    /// Spawn a future to be run by the executor.
    ///
    /// Returns a task that can be awaited or canceled.
    fn spawn<T: Send + 'static, F: Future<Output = T> + Send + 'static>(
        &self,
        future: F,
    ) -> Self::Task<T>;
}

pub trait ExecutorLockExt: Executor {
    type RwLock<T: ?Sized>: RwLock<T> + ?Sized;
    type Barrier: Barrier;
    type Semaphore: Semaphore;
}

pub trait ExecutorChannelExt: Executor {
    type OneshotSender<T>: oneshot::Sender<T>;
    type OneshotReceiver<T>: oneshot::Receiver<T>;
}
