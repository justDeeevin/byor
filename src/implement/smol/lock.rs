use crate::{lock::*, runtime::Smol};

impl Barrier for smol::lock::Barrier {
    fn new(n: usize) -> Self {
        Self::new(n)
    }

    fn wait(&self) -> impl Future<Output = impl BarrierWaitResult> {
        self.wait()
    }
}

impl BarrierWaitResult for smol::lock::BarrierWaitResult {
    fn is_leader(&self) -> bool {
        self.is_leader()
    }
}

impl<T: ?Sized> Mutex<T> for smol::lock::Mutex<T> {
    type Guard<'a>
        = smol::lock::MutexGuard<'a, T>
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

impl<'a, T: ?Sized + 'a> MutexExt<'a, T> for smol::lock::Mutex<T> {
    fn blocking_lock(&self) -> Self::Guard<'_> {
        self.lock_blocking()
    }
}

impl<'a, T: ?Sized> MutexGuard<'a, T> for smol::lock::MutexGuard<'a, T> {
    fn source(this: &Self) -> &'a (impl Mutex<T, Guard<'a> = Self> + ?Sized + 'a) {
        Self::source(this)
    }
}

impl<T: ?Sized> RwLock<T> for smol::lock::RwLock<T> {
    type ReadGuard<'a>
        = smol::lock::RwLockReadGuard<'a, T>
    where
        Self: 'a;
    type WriteGuard<'a>
        = smol::lock::RwLockWriteGuard<'a, T>
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
        self.read_blocking()
    }

    fn try_read(&self) -> Option<Self::ReadGuard<'_>> {
        self.try_read()
    }

    fn write(&self) -> impl Future<Output = Self::WriteGuard<'_>> {
        self.write()
    }

    fn blocking_write(&self) -> Self::WriteGuard<'_> {
        self.write_blocking()
    }

    fn try_write(&self) -> Option<Self::WriteGuard<'_>> {
        self.try_write()
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

impl<T: ?Sized> RwLockReadGuard<T> for smol::lock::RwLockReadGuard<'_, T> {}

impl<T: ?Sized> RwLockReadGuard<T> for smol::lock::RwLockWriteGuard<'_, T> {}
impl<T: ?Sized> RwLockWriteGuard<T> for smol::lock::RwLockWriteGuard<'_, T> {
    fn downgrade(self) -> impl RwLockReadGuard<T> {
        Self::downgrade(self)
    }
}

impl Semaphore for smol::lock::Semaphore {
    type Permit<'a>
        = smol::lock::SemaphoreGuard<'a>
    where
        Self: 'a;

    fn new(permits: usize) -> Self {
        Self::new(permits)
    }

    fn add_permits(&self, n: usize) {
        self.add_permits(n)
    }

    async fn acquire(&self) -> Option<Self::Permit<'_>> {
        Some(self.acquire().await)
    }

    fn try_acquire(&self) -> Option<Self::Permit<'_>> {
        self.try_acquire()
    }
}

impl SemaphorePermit for smol::lock::SemaphoreGuard<'_> {
    fn forget(self) {
        self.forget()
    }
}

impl RuntimeLock for Smol {
    type Mutex<T: ?Sized> = smol::lock::Mutex<T>;
}

impl RuntimeLockExt for Smol {
    type RwLock<T: ?Sized> = smol::lock::RwLock<T>;
    type Barrier = smol::lock::Barrier;
    type Semaphore = smol::lock::Semaphore;
}
