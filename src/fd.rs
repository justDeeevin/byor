//! File descriptor.
//!
//! This is a low-level interface for I/O interactions.

use std::{
    io::Result,
    os::fd::AsFd,
    task::{Context, Poll},
};

pub trait Fd<T: AsFd>: AsFd + Sized {
    /// Create a new file descriptor from the given I/O handle.
    ///
    /// <div class="warning">The given resource must have already been put into non-blocking
    /// mode.</div>
    fn new(inner: T) -> Result<Self>;
    /// Get a mutable reference to the inner I/O handle.
    ///
    /// # Safety
    ///
    /// The underlying I/O source must not be dropped using the returned handle.
    ///
    /// # Implementation Note
    /// This method is marked unsafe in smol but is safe in tokio. Violating the above
    /// condition is still dangerous with tokio, but it is guaranteed to not be undefined. Whether
    /// this is the case with smol is unclear. Since it is extremely ill-advised to drop the handle
    /// with tokio as well, this abstract method is marked unsafe.
    unsafe fn get_mut(&mut self) -> &mut T;
    /// Get a reference to the inner I/O handle.
    fn get_ref(&self) -> &T;
    /// Unwrap the inner I/O handle.
    fn into_inner(self) -> Result<T>;

    /// Poll for read readiness.
    ///
    /// This should not be used by multiple tasks at once. It is intended for cases where creating
    /// and pinning the future returned by [`readable`](Fd::readable) is not feasible. Using
    /// [`readable`](Fd::readable)  is preferred.
    fn poll_readable(&self, cx: &mut Context) -> Poll<Result<()>>;
    /// Poll for write readiness.
    ///
    /// This should not be used by multiple tasks at once. It is intended for cases where creating
    /// and pinning the future returned by [`writable`](Fd::writable) is not feasible. Using
    /// [`writable`](Fd::writable)  is preferred.
    fn poll_writable(&self, cx: &mut Context) -> Poll<Result<()>>;

    /// Wait for the resource to become readable.
    fn readable(&self) -> impl Future<Output = Result<()>>;
    /// Wait for the resource to become writable.
    fn writable(&self) -> impl Future<Output = Result<()>>;

    /// Runs the user-provided I/O operation after waiting for read readiness.
    ///
    /// Since file descriptors may be spuriously marked read-ready, the closure will be called in a
    /// loop until it returns something other than a [`WouldBlock`](std::io::ErrorKind::WouldBlock)
    /// error.
    fn read_with<R>(&self, op: impl FnMut(&T) -> Result<R>) -> impl Future<Output = Result<R>>;
    /// Same as [`read_with`](Fd::read_with), but with mutable access to the inner I/O handle.
    ///
    /// # Safety
    ///
    /// The underlying I/O source must not be dropped using the returned handle.
    ///
    /// See [`get_mut`](Fd::get_mut) for more information.
    unsafe fn read_with_mut<R>(
        &mut self,
        op: impl FnMut(&mut T) -> Result<R>,
    ) -> impl Future<Output = Result<R>>;

    /// Runs the user-provided I/O operation after waiting for write readiness.
    ///
    /// Since file descriptors may be spuriously marked write-ready, the closure will be called in a
    /// loop until it returns something other than a [`WouldBlock`](std::io::ErrorKind::WouldBlock)
    /// error.
    fn write_with<R>(&self, op: impl FnMut(&T) -> Result<R>) -> impl Future<Output = Result<R>>;
    /// Same as [`write_with`](Fd::write_with), but with mutable access to the inner I/O handle.
    ///
    /// # Safety
    ///
    /// The underlying I/O source must not be dropped using the returned handle.
    ///
    /// See [`get_mut`](Fd::get_mut) for more information.
    fn write_with_mut<R>(
        &mut self,
        op: impl FnMut(&mut T) -> Result<R>,
    ) -> impl Future<Output = Result<R>>;
}
