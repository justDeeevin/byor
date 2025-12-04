use crate::{lock::*, runtime::Tokio};

impl Barrier for tokio::sync::Barrier {
    fn new(n: usize) -> Self {
        Self::new(n)
    }

    fn wait(&self) -> impl Future<Output = impl BarrierWaitResult> {
        self.wait()
    }
}

impl BarrierWaitResult for tokio::sync::BarrierWaitResult {
    fn is_leader(&self) -> bool {
        self.is_leader()
    }
}

impl<T: ?Sized> Mutex<T> for tokio::sync::Mutex<T> {
    type Guard<'a>
        = tokio::sync::MutexGuard<'a, T>
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
        self.try_lock().ok()
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

impl<'a, T: ?Sized + 'a> MutexExt<'a, T> for tokio::sync::Mutex<T> {
    fn blocking_lock(&self) -> Self::Guard<'_> {
        self.blocking_lock()
    }
}

impl<'a, T: ?Sized> MutexGuard<'a, T> for tokio::sync::MutexGuard<'a, T> {
    fn source(this: &Self) -> &'a (impl Mutex<T, Guard<'a> = Self> + ?Sized + 'a) {
        Self::mutex(this)
    }
}

impl<T: ?Sized> RwLock<T> for tokio::sync::RwLock<T> {
    type ReadGuard<'a>
        = tokio::sync::RwLockReadGuard<'a, T>
    where
        Self: 'a;
    type WriteGuard<'a>
        = tokio::sync::RwLockWriteGuard<'a, T>
    where
        Self: 'a;

    fn new(t: T) -> Self
    where
        T: Sized,
    {
        Self::new(t)
    }

    fn read(&self) -> impl Future<Output = Self::ReadGuard<'_>> {
        self.read()
    }

    fn blocking_read(&self) -> Self::ReadGuard<'_> {
        self.blocking_read()
    }

    fn try_read(&self) -> Option<Self::ReadGuard<'_>> {
        self.try_read().ok()
    }

    fn write(&self) -> impl Future<Output = Self::WriteGuard<'_>> {
        self.write()
    }

    fn blocking_write(&self) -> Self::WriteGuard<'_> {
        self.blocking_write()
    }

    fn try_write(&self) -> Option<Self::WriteGuard<'_>> {
        self.try_write().ok()
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

impl<T: ?Sized> RwLockReadGuard<T> for tokio::sync::RwLockReadGuard<'_, T> {}

impl<T: ?Sized> RwLockReadGuard<T> for tokio::sync::RwLockWriteGuard<'_, T> {}
impl<T: ?Sized> RwLockWriteGuard<T> for tokio::sync::RwLockWriteGuard<'_, T> {
    fn downgrade(self) -> impl RwLockReadGuard<T> {
        self.downgrade()
    }
}

impl Semaphore for tokio::sync::Semaphore {
    type Permit<'a>
        = tokio::sync::SemaphorePermit<'a>
    where
        Self: 'a;

    fn new(permits: usize) -> Self {
        Self::new(permits)
    }

    fn add_permits(&self, n: usize) {
        self.add_permits(n)
    }

    async fn acquire(&self) -> Option<Self::Permit<'_>> {
        self.acquire().await.ok()
    }

    fn try_acquire(&self) -> Option<Self::Permit<'_>> {
        self.try_acquire().ok()
    }
}

impl SemaphorePermit for tokio::sync::SemaphorePermit<'_> {
    fn forget(self) {
        self.forget()
    }
}

impl RuntimeLock for Tokio {
    type Mutex<T: ?Sized> = tokio::sync::Mutex<T>;
}

impl RuntimeLockExt for Tokio {
    type RwLock<T: ?Sized> = tokio::sync::RwLock<T>;
    type Barrier = tokio::sync::Barrier;
    type Semaphore = tokio::sync::Semaphore;
}
