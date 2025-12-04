use crate::{
    lock::{Mutex, RuntimeLock},
    runtime::Futures,
};

impl<T: ?Sized> Mutex<T> for futures::lock::Mutex<T> {
    type Guard<'a>
        = futures::lock::MutexGuard<'a, T>
    where
        Self: 'a;

    fn new(t: T) -> Self
    where
        T: Sized,
    {
        Self::new(t)
    }

    fn lock(&self) -> impl Future<Output = Self::Guard<'_>> {
        self.lock()
    }

    fn try_lock(&self) -> Option<Self::Guard<'_>> {
        self.try_lock()
    }

    fn get_mut(&mut self) -> &mut T {
        self.get_mut()
    }

    fn into_inner(self) -> T
    where
        T: Sized,
    {
        self.into_inner()
    }
}

impl RuntimeLock for Futures {
    type Mutex<T: ?Sized> = futures::lock::Mutex<T>;
}
