use crate::fd::{Fd, RuntimeFd};
use std::os::fd::AsFd;
use tokio::io::{Interest, unix::AsyncFd};

impl RuntimeFd for crate::runtime::Tokio {
    type Fd<T: AsFd> = TokioFd<T>;
}

struct AsRaw<T>(T);

impl<T: AsFd> std::os::fd::AsRawFd for AsRaw<T> {
    fn as_raw_fd(&self) -> std::os::unix::prelude::RawFd {
        self.0.as_fd().as_raw_fd()
    }
}

pub struct TokioFd<T: AsFd>(AsyncFd<AsRaw<T>>);

impl<T: AsFd> AsFd for TokioFd<T> {
    fn as_fd(&self) -> std::os::unix::prelude::BorrowedFd<'_> {
        self.0.as_fd()
    }
}

impl<T: AsFd> Fd for TokioFd<T> {
    type Inner = T;

    fn new(inner: T) -> std::io::Result<Self> {
        // SAFETY: T implements AsFd, which means that its AsRawFd implementation
        // always satisfies AsyncFd's requirements.
        Ok(Self(
            unsafe { AsyncFd::register(AsRaw(inner)) }.map_err(|e| e.into_parts().1)?,
        ))
    }
    unsafe fn get_mut(&mut self) -> &mut T {
        &mut self.0.get_mut().0
    }
    fn get_ref(&self) -> &T {
        &self.0.get_ref().0
    }
    fn into_inner(self) -> std::io::Result<T> {
        Ok(self.0.into_inner().0)
    }
    fn poll_readable(&self, cx: &mut std::task::Context) -> std::task::Poll<std::io::Result<()>> {
        self.0
            .poll_read_ready(cx)
            .map(|r| r.map(|mut guard| guard.clear_ready()))
    }
    fn poll_writable(&self, cx: &mut std::task::Context) -> std::task::Poll<std::io::Result<()>> {
        self.0
            .poll_write_ready(cx)
            .map(|r| r.map(|mut guard| guard.clear_ready()))
    }
    async fn readable(&self) -> std::io::Result<()> {
        self.0.readable().await.map(|mut guard| guard.clear_ready())
    }
    async fn writable(&self) -> std::io::Result<()> {
        self.0.writable().await.map(|mut guard| guard.clear_ready())
    }
    fn read_with<R>(
        &self,
        mut op: impl FnMut(&T) -> std::io::Result<R>,
    ) -> impl Future<Output = std::io::Result<R>> {
        self.0.async_io(Interest::READABLE, move |wrap| op(&wrap.0))
    }
    unsafe fn read_with_mut<R>(
        &mut self,
        mut op: impl FnMut(&mut T) -> std::io::Result<R>,
    ) -> impl Future<Output = std::io::Result<R>> {
        self.0
            .async_io_mut(Interest::READABLE, move |wrap| op(&mut wrap.0))
    }
    fn write_with<R>(
        &self,
        mut op: impl FnMut(&T) -> std::io::Result<R>,
    ) -> impl Future<Output = std::io::Result<R>> {
        self.0.async_io(Interest::WRITABLE, move |wrap| op(&wrap.0))
    }
    unsafe fn write_with_mut<R>(
        &mut self,
        mut op: impl FnMut(&mut T) -> std::io::Result<R>,
    ) -> impl Future<Output = std::io::Result<R>> {
        self.0
            .async_io_mut(Interest::WRITABLE, move |wrap| op(&mut wrap.0))
    }
}
