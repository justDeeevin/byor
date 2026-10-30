//! Unix domain sockets.

use futures_io::{AsyncRead, AsyncWrite};
use std::{
    io::Result,
    net::Shutdown,
    os::{
        fd::{AsFd, AsRawFd},
        unix::net::SocketAddr,
    },
    path::Path,
};

/// A runtime with Unix domain sockets.
pub trait RuntimeUnixSocket {
    type UnixStream: UnixStream;
    type UnixListener: UnixListener<Stream = Self::UnixStream>;
    type UnixDatagram: UnixDatagram;
}

/// A Unix connection.
pub trait UnixStream: AsRawFd + AsyncRead + AsyncWrite + Sized
// This can't require TryFrom<std::os::unix::net::UnixStream> because Compat
// doesn't implement it
{
    /// Creates a Unix connection to the given path.
    fn connect(path: impl AsRef<Path>) -> impl Future<Output = Result<Self>>;

    fn connect_concrete(path: &Path) -> impl Future<Output = Result<Self>> {
        Self::connect(path)
    }

    /// Creates a pair of connected Unix sockets.
    ///
    /// Data written to one socket will be readable by the other, and vice versa.
    fn pair() -> Result<(Self, Self)>;

    /// The socket address of the local half of this connection.
    fn local_addr(&self) -> Result<SocketAddr>;

    /// The socket address of the remote half of this connection.
    fn peer_addr(&self) -> Result<SocketAddr>;

    /// Shuts down the read half, write half, or both halves of this connection.
    fn shutdown(&self, how: Shutdown) -> Result<()>;
}

/// A Unix socket which can accept connections from other Unix sockets.
pub trait UnixListener:
    AsFd + AsRawFd + TryFrom<std::os::unix::net::UnixListener, Error = std::io::Error>
{
    type Stream: UnixStream;

    /// Creates a new listener bound to the given path.
    ///
    /// <div class="warning">The file at the bound path persists after the program exits, and
    /// thus attempting to bind to the same path on a second execution will fail. Make sure to
    /// clean the path either before calling <code>bind</code> or before the program ends.</div>
    fn bind(path: impl AsRef<Path>) -> Result<Self>;

    fn bind_concrete(path: &Path) -> Result<Self> {
        Self::bind(path)
    }

    /// Accepts a new incoming connection.
    fn accept(&self) -> impl Future<Output = Result<(Self::Stream, SocketAddr)>>;

    /// The local socket address of this listener.
    fn local_addr(&self) -> Result<SocketAddr>;
}

/// A Unix datagram socket.
///
/// A socket can either be named (associated with a filesystem path; see [`bind`](UnixDatagram::bind)) or unnamed (see [`unbound`](UnixDatagram::unbound))
pub trait UnixDatagram:
    AsRawFd + TryFrom<std::os::unix::net::UnixDatagram, Error = std::io::Error>
{
    /// Creates a new datagram bound to the given path.
    ///
    /// <div class="warning">The file at the bound path persists after the program exits, and
    /// thus attempting to bind to the same path on a second execution will fail. Make sure to
    /// clean the path either before calling <code>bind</code> or before the program ends.</div>
    fn bind(path: impl AsRef<Path>) -> Result<Self>;

    fn bind_concrete(path: &Path) -> Result<Self> {
        Self::bind(path)
    }

    /// Creates an unnamed pair of connected sockets.
    ///
    /// Data written to one socket will be readable by the other, and vice versa.
    fn pair() -> Result<(Self, Self)>;

    /// Creates a new datagram which is not bound to any address.
    fn unbound() -> Result<Self>;

    /// Connects the socket to the specified address.
    ///
    /// [`send`](UnixDatagram::send) and [`recv`](UnixDatagram::recv) will use the given
    /// address. Additionally, [`recv_from`](UnixDatagram::recv_from) will filter out messages
    /// from any other address.
    fn connect(&self, path: impl AsRef<Path>) -> Result<()>;

    fn connect_concrete(&self, path: &Path) -> Result<()> {
        self.connect(path)
    }

    /// The local address to which this socket is bound.
    fn local_addr(&self) -> Result<SocketAddr>;

    /// The remote address to which this socket is connected.
    fn peer_addr(&self) -> Result<SocketAddr>;

    /// Receives data from the connected address, returning the number of bytes received.
    fn recv(&self, buf: &mut [u8]) -> impl Future<Output = Result<usize>>;

    /// Receives data from an address, returning the number of bytes received and the address of
    /// their source.
    fn recv_from(&self, buf: &mut [u8]) -> impl Future<Output = Result<(usize, SocketAddr)>>;

    /// Sends data to the connected address, returning the number of bytes sent.
    fn send(&self, buf: &[u8]) -> impl Future<Output = Result<usize>>;

    /// Sends data to the given address, returning the number of bytes sent.
    fn send_to(&self, buf: &[u8], path: impl AsRef<Path>) -> impl Future<Output = Result<usize>>;

    fn send_to_concrete(&self, buf: &[u8], path: &Path) -> impl Future<Output = Result<usize>> {
        self.send_to(buf, path)
    }

    /// Shuts down the read half, write half, or both halves of this socket.
    fn shutdown(&self, how: Shutdown) -> Result<()>;
}
