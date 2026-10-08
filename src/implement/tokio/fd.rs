use crate::fd::{Fd, RuntimeFd};
use std::os::fd::{AsFd, AsRawFd};
use tokio::io::{Interest, unix::AsyncFd};

impl RuntimeFd for crate::runtime::Tokio {
    type Fd<T: AsFd + AsRawFd> = AsyncFd<T>;
}

impl<T: AsFd + AsRawFd> Fd<T> for AsyncFd<T> {
    fn new(inner: T) -> std::io::Result<Self> {
        AsyncFd::new(inner)
    }
    unsafe fn get_mut(&mut self) -> &mut T {
        self.get_mut()
    }
    fn get_ref(&self) -> &T {
        self.get_ref()
    }
    fn into_inner(self) -> std::io::Result<T> {
        Ok(self.into_inner())
    }
    fn poll_readable(&self, cx: &mut std::task::Context) -> std::task::Poll<std::io::Result<()>> {
        self.poll_read_ready(cx).map(|r| r.map(|_| ()))
    }
    fn poll_writable(&self, cx: &mut std::task::Context) -> std::task::Poll<std::io::Result<()>> {
        self.poll_write_ready(cx).map(|r| r.map(|_| ()))
    }
    async fn readable(&self) -> std::io::Result<()> {
        self.readable().await.map(|_| ())
    }
    async fn writable(&self) -> std::io::Result<()> {
        self.writable().await.map(|_| ())
    }
    fn read_with<R>(
        &self,
        op: impl FnMut(&T) -> std::io::Result<R>,
    ) -> impl Future<Output = std::io::Result<R>> {
        self.async_io(Interest::READABLE, op)
    }
    unsafe fn read_with_mut<R>(
        &mut self,
        op: impl FnMut(&mut T) -> std::io::Result<R>,
    ) -> impl Future<Output = std::io::Result<R>> {
        self.async_io_mut(Interest::READABLE, op)
    }
    fn write_with<R>(
        &self,
        op: impl FnMut(&T) -> std::io::Result<R>,
    ) -> impl Future<Output = std::io::Result<R>> {
        self.async_io(Interest::WRITABLE, op)
    }
    fn write_with_mut<R>(
        &mut self,
        op: impl FnMut(&mut T) -> std::io::Result<R>,
    ) -> impl Future<Output = std::io::Result<R>> {
        self.async_io_mut(Interest::WRITABLE, op)
    }
}
