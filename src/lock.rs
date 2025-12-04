//! Async synchronization primitives.
//!
//! In general, using [`std::sync`] is preferred over their async counterparts. You should only
//! reach for these when a lock needs to be held across an await point (clippy will warn you in
//! this case:
//! [`Clippy::await_holding_lock`](https://rust-lang.github.io/rust-clippy/stable/index.html#await_holding_lock)).

use std::ops::{Deref, DerefMut};

/// An asynchronous mutex similar to [`std::sync::Mutex`].
pub trait Mutex<T: ?Sized> {
    /// An RAII guard that unlocks the mutex when dropped.
    type Guard<'a>
    where
        Self: 'a;

    /// Creates a new lock in an unlocked state ready for use.
    fn new(t: T) -> Self
    where
        T: Sized;

    /// Locks this mutex, causing the current task to yield until the lock has been acquired.
    fn lock(&self) -> impl Future<Output = Self::Guard<'_>>;

    /// Attempts to lock the mutex, returning [`None`] if it is already locked.
    fn try_lock(&self) -> Option<Self::Guard<'_>>;

    /// Returns a mutable reference to the underlying data.
    ///
    /// Since this call borrows the `Mutex` mutably, no actual locking needs to take place—the mutable borrow statically guarantees no locks exist.
    fn get_mut(&mut self) -> &mut T;

    /// Consumes the `Mutex`, returning the underlying data.
    fn into_inner(self) -> T
    where
        T: Sized;
}

/// A guard that unlocks its associated [`Mutex`] when dropped.
pub trait MutexGuard<'a, T: ?Sized>: Deref<Target = T> + DerefMut {
    /// Returns a reference to the `Mutex` from which this guard was acquired.
    fn source(this: &Self) -> &'a (impl Mutex<T, Guard<'a> = Self> + ?Sized + 'a);
}

/// More extensive behavior for `Mutex` implemented by tokio and smol (that is, only _not_
/// implemented by futures).
pub trait MutexExt<'a, T: ?Sized>: Mutex<T>
where
    <Self as Mutex<T>>::Guard<'a>: MutexGuard<'a, T>,
    Self: 'a,
{
    /// Locks this mutex, **blocking the current thread** until it can be acquired.
    ///
    /// This method should not be used in an asynchronous context. It is intended to facilitate
    /// the use of the same `Mutex` in synchronous and asynchronous code.
    ///
    /// # Panics
    ///
    /// [Tokio's implementation](tokio::sync::Mutex::blocking_lock) will panic if used in an
    /// asynchronous context.
    fn blocking_lock(&self) -> Self::Guard<'_>;
}

/// An asynchronous reader-writer lock similar to [`std::sync::RwLock`].
pub trait RwLock<T: ?Sized> {
    /// A read-only RAII guard that releases its read lock when dropped.
    type ReadGuard<'a>: RwLockReadGuard<T>
    where
        Self: 'a;
    /// A read-write RAII guard that releases its write lock when dropped.
    type WriteGuard<'a>: RwLockWriteGuard<T>
    where
        Self: 'a;

    /// Creates a new lock in an unlocked state ready for use.
    fn new(t: T) -> Self
    where
        T: Sized;

    /// Locks this `RwLock` with shared read access, causing the current task to yield until
    /// the lock as been acquired.
    fn read(&self) -> impl Future<Output = Self::ReadGuard<'_>>;

    /// Locks this `RwLock` with shared read access, **blocking the current thrread** until it
    /// can be acquired.
    ///
    /// This method should not be used in an asynchronous context. It is intended to facilitate
    /// the use of the same `RwLock` in synchronous and asynchronous code.
    ///
    /// # Panics
    ///
    /// [Tokio's implementation](tokio::sync::RwLock::blocking_read) will panic if used in an
    /// asynchronous context.
    fn blocking_read(&self) -> Self::ReadGuard<'_>;

    /// Attempts to lock this `RwLock` with shared read access, returning [`None`] if it is
    /// already locked.
    fn try_read(&self) -> Option<Self::ReadGuard<'_>>;

    /// Locks this `RwLock` with exclusive write access, causing the current task to yield until
    /// the lock has been acquired.
    fn write(&self) -> impl Future<Output = Self::WriteGuard<'_>>;

    /// Locks this `RwLock` with exclusive write access, **blocking the current thread** until it
    /// can be acquired.
    /// This method should not be used in an asynchronous context. It is intended to facilitate
    /// the use of the same `RwLock` in synchronous and asynchronous code.
    ///
    /// # Panics
    ///
    /// [Tokio's implementation](tokio::sync::RwLock::blocking_write) will panic if used in an
    /// asynchronous context.
    fn blocking_write(&self) -> Self::WriteGuard<'_>;

    /// Attempts to lock this `RwLock` with exclusive write access, returning [`None`] if it is
    /// already locked.
    fn try_write(&self) -> Option<Self::WriteGuard<'_>>;

    /// Returns a mutable reference to the underlying data.
    ///
    /// Since this call borrows the `RwLock` mutably, no actual locking needs to take place—the
    /// mutable borrow statically guarantees no locks exist.
    fn get_mut(&mut self) -> &mut T;

    /// Consumes the `RwLock`, returning the underlying data.
    fn into_inner(self) -> T
    where
        T: Sized;
}

/// A shared read guard that unlocks its associated [`RwLock`] when dropped.
pub trait RwLockReadGuard<T: ?Sized>: Deref<Target = T> {}

/// An exclusive write guard that unlocks its associated [`RwLock`] when dropped.
pub trait RwLockWriteGuard<T: ?Sized>: RwLockReadGuard<T> + DerefMut {
    /// Downgrades into a read lock.
    fn downgrade(self) -> impl RwLockReadGuard<T>;
}

/// A counter to synchrononize multiple tasks at the same time.
pub trait Barrier {
    /// Creates a new barrier that will block a given number of tasks.
    ///
    /// A barrier will block `n`-1 tasks which will call [`Barrier::wait`] and then wake up all
    /// tasks at once when the `n`th task calls `wait`.
    fn new(n: usize) -> Self;

    /// Does not resolve until all tasks have rendezvoused here.
    ///
    /// Barriers are re-usable after all tasks have rendezvoused.
    ///
    /// A single (arbitrary) future will receive a [`BarrierWaitResult`] that returns true from [`BarrierWaitResult::is_leader`] when returning from this function, and all other tasks will receive a result that will return false from `is_leader`.
    fn wait(&self) -> impl Future<Output = impl BarrierWaitResult>;
}

/// Returned by [`wait`](Barrier::wait) when all tasks in the `Barrier` have rendezvoused.
pub trait BarrierWaitResult {
    /// Returns `true` if this task from `wait` is the "leader" task.
    ///
    /// Only one task will have `true` returned from their result, all other tasks will have `false` returned.
    fn is_leader(&self) -> bool;
}

/// A counter for limiting the number of concurrent operations.
pub trait Semaphore {
    // An RAII guard that releases the permit when dropped.
    type Permit<'a>: SemaphorePermit
    where
        Self: 'a;

    /// Creates a new semaphore with the given number of permits.
    ///
    /// # Panics
    ///
    /// Tokio's semaphore will panic if `permits` exceeds [`tokio::sync::Semaphore::MAX_PERMITS`].
    fn new(permits: usize) -> Self;

    /// Adds `n` new permits to the semaphore.
    ///
    /// # Panics
    ///
    /// Tokio's semaphore will panic if this makes the semaphore's permit count exceed [`tokio::sync::Semaphore::MAX_PERMITS`].
    fn add_permits(&self, n: usize);

    /// Waits for a permit for a concurrent operation.
    ///
    /// Tokio's semaphore can be closed. In this case, this function will return [`None`]
    fn acquire(&self) -> impl Future<Output = Option<Self::Permit<'_>>>;

    /// Attempts to acquire a permit for a concurrent operation.
    fn try_acquire(&self) -> Option<Self::Permit<'_>>;
}

/// A permit for a concurrent operation.
///
/// The number of used permits in the semaphore is decremented when this is dropped.
pub trait SemaphorePermit {
    /// Drops the guard _without_ releasing the acquired permit.
    fn forget(self);
}

/// A runtime with a Mutex implementation.
pub trait RuntimeLock {
    type Mutex<T: ?Sized>: Mutex<T> + ?Sized;
}

/// A runtime with more locks.
pub trait RuntimeLockExt: RuntimeLock {
    type RwLock<T: ?Sized>: RwLock<T> + ?Sized;
    type Barrier: Barrier;
    type Semaphore: Semaphore;
}
