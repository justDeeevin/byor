use crate::net::unix::*;
use futures_util::TryFutureExt;
use std::os::fd::AsFd;
use tokio_util::compat::{Compat, TokioAsyncReadCompatExt};

impl UnixStream for Compat<tokio::net::UnixStream> {
    fn connect(path: impl AsRef<std::path::Path>) -> impl Future<Output = std::io::Result<Self>> {
        tokio::net::UnixStream::connect(path).map_ok(TokioAsyncReadCompatExt::compat)
    }

    fn pair() -> std::io::Result<(Self, Self)> {
        tokio::net::UnixStream::pair().map(|(l, r)| (l.compat(), r.compat()))
    }

    fn local_addr(&self) -> std::io::Result<std::os::unix::net::SocketAddr> {
        self.get_ref().local_addr().map(Into::into)
    }

    fn peer_addr(&self) -> std::io::Result<std::os::unix::net::SocketAddr> {
        self.get_ref().peer_addr().map(Into::into)
    }

    // TODO: does this work?
    fn shutdown(&self, how: std::net::Shutdown) -> std::io::Result<()> {
        std::os::unix::net::UnixStream::from(self.get_ref().as_fd().try_clone_to_owned()?)
            .shutdown(how)
    }
}

impl UnixListener for tokio::net::UnixListener {
    type Stream = Compat<tokio::net::UnixStream>;

    fn bind(path: impl AsRef<std::path::Path>) -> std::io::Result<Self> {
        Self::bind(path)
    }

    fn accept(
        &self,
    ) -> impl Future<Output = std::io::Result<(Self::Stream, std::os::unix::net::SocketAddr)>> {
        self.accept().map_ok(|(s, a)| (s.compat(), a.into()))
    }

    fn local_addr(&self) -> std::io::Result<std::os::unix::net::SocketAddr> {
        self.local_addr().map(Into::into)
    }
}

impl UnixDatagram for tokio::net::UnixDatagram {
    fn bind(path: impl AsRef<std::path::Path>) -> std::io::Result<Self> {
        Self::bind(path)
    }

    fn pair() -> std::io::Result<(Self, Self)> {
        Self::pair()
    }

    fn unbound() -> std::io::Result<Self> {
        Self::unbound()
    }

    fn connect(&self, path: impl AsRef<std::path::Path>) -> std::io::Result<()> {
        self.connect(path)
    }

    fn local_addr(&self) -> std::io::Result<std::os::unix::net::SocketAddr> {
        self.local_addr().map(Into::into)
    }

    fn peer_addr(&self) -> std::io::Result<std::os::unix::net::SocketAddr> {
        self.peer_addr().map(Into::into)
    }

    fn recv(&self, buf: &mut [u8]) -> impl Future<Output = std::io::Result<usize>> {
        self.recv(buf)
    }

    fn recv_from(
        &self,
        buf: &mut [u8],
    ) -> impl Future<Output = std::io::Result<(usize, std::os::unix::net::SocketAddr)>> {
        self.recv_from(buf)
            .map_ok(|(read, addr)| (read, addr.into()))
    }

    fn send(&self, buf: &[u8]) -> impl Future<Output = std::io::Result<usize>> {
        self.send(buf)
    }

    fn send_to(
        &self,
        buf: &[u8],
        path: impl AsRef<std::path::Path>,
    ) -> impl Future<Output = std::io::Result<usize>> {
        self.send_to(buf, path)
    }

    fn shutdown(&self, how: std::net::Shutdown) -> std::io::Result<()> {
        self.shutdown(how)
    }
}
