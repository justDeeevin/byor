use crate::fd::Fd;
use smol::Async;
use std::{io::Result, os::fd::AsFd};

impl<T: AsFd> Fd<T> for Async<T> {
    fn new(inner: T) -> Result<Self> {
        Async::new_nonblocking(inner)
    }

    unsafe fn get_mut(&mut self) -> &mut T {
        unsafe { self.get_mut() }
    }

    fn get_ref(&self) -> &T {
        self.get_ref()
    }

    fn into_inner(self) -> Result<T> {
        self.into_inner()
    }

    fn poll_readable(&self, cx: &mut std::task::Context) -> std::task::Poll<Result<()>> {
        self.poll_readable(cx)
    }

    fn poll_writable(&self, cx: &mut std::task::Context) -> std::task::Poll<Result<()>> {
        self.poll_writable(cx)
    }

    fn readable(&self) -> impl Future<Output = Result<()>> {
        self.readable()
    }

    fn writable(&self) -> impl Future<Output = Result<()>> {
        self.writable()
    }

    fn read_with<R>(&self, op: impl FnMut(&T) -> Result<R>) -> impl Future<Output = Result<R>> {
        self.read_with(op)
    }

    unsafe fn read_with_mut<R>(
        &mut self,
        op: impl FnMut(&mut T) -> Result<R>,
    ) -> impl Future<Output = Result<R>> {
        unsafe { self.read_with_mut(op) }
    }

    fn write_with<R>(&self, op: impl FnMut(&T) -> Result<R>) -> impl Future<Output = Result<R>> {
        self.write_with(op)
    }

    fn write_with_mut<R>(
        &mut self,
        op: impl FnMut(&mut T) -> Result<R>,
    ) -> impl Future<Output = Result<R>> {
        unsafe { self.write_with_mut(op) }
    }
}
