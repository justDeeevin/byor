//! TCP/UDP networking.

use futures_lite::{AsyncRead, AsyncWrite};
use std::{
    io::Result,
    net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6},
    os::fd::{AsFd, AsRawFd},
};

/// A type that can be converted to [`SocketAddr`]s.
pub enum SocketAddrsKind<'a> {
    Slice(&'a [SocketAddr]),
    StrPort(&'a str, u16),
    IpAddr(IpAddr, u16),
    Ipv4Addr(Ipv4Addr, u16),
    Ipv6Addr(Ipv6Addr, u16),
    Str(&'a str),
    SocketAddr(&'a SocketAddr),
    SocketAddrV4(&'a SocketAddrV4),
    SocketAddrV6(&'a SocketAddrV6),
}

pub trait ToSocketAddrs {
    fn kind(&self) -> SocketAddrsKind<'_>;
}

impl ToSocketAddrs for &[SocketAddr] {
    fn kind(&self) -> SocketAddrsKind<'_> {
        SocketAddrsKind::Slice(self)
    }
}
impl ToSocketAddrs for (&str, u16) {
    fn kind(&self) -> SocketAddrsKind<'_> {
        SocketAddrsKind::StrPort(self.0, self.1)
    }
}
impl ToSocketAddrs for (IpAddr, u16) {
    fn kind(&self) -> SocketAddrsKind<'_> {
        SocketAddrsKind::IpAddr(self.0, self.1)
    }
}
impl ToSocketAddrs for (String, u16) {
    fn kind(&self) -> SocketAddrsKind<'_> {
        SocketAddrsKind::StrPort(&self.0, self.1)
    }
}
impl ToSocketAddrs for (Ipv4Addr, u16) {
    fn kind(&self) -> SocketAddrsKind<'_> {
        SocketAddrsKind::Ipv4Addr(self.0, self.1)
    }
}
impl ToSocketAddrs for (Ipv6Addr, u16) {
    fn kind(&self) -> SocketAddrsKind<'_> {
        SocketAddrsKind::Ipv6Addr(self.0, self.1)
    }
}
impl ToSocketAddrs for str {
    fn kind(&self) -> SocketAddrsKind<'_> {
        SocketAddrsKind::Str(self)
    }
}
impl ToSocketAddrs for String {
    fn kind(&self) -> SocketAddrsKind<'_> {
        SocketAddrsKind::Str(self)
    }
}
impl ToSocketAddrs for SocketAddr {
    fn kind(&self) -> SocketAddrsKind<'_> {
        SocketAddrsKind::SocketAddr(self)
    }
}
impl ToSocketAddrs for SocketAddrV4 {
    fn kind(&self) -> SocketAddrsKind<'_> {
        SocketAddrsKind::SocketAddrV4(self)
    }
}
impl ToSocketAddrs for SocketAddrV6 {
    fn kind(&self) -> SocketAddrsKind<'_> {
        SocketAddrsKind::SocketAddrV6(self)
    }
}
impl<T: ToSocketAddrs> ToSocketAddrs for &T {
    fn kind(&self) -> SocketAddrsKind<'_> {
        (*self).kind()
    }
}

/// A TCP stream between a local and a remote socket.
///
/// A TCP stream can either be created by connecting to an endpoint with
/// [`connect`](TcpStream::connect) or by accepting a connection from a [listener](TcpListener).
pub trait TcpStream: AsyncRead + AsyncWrite + AsRawFd + Sized {
    /// Opens a TCP connection to a remote host.
    fn connect(addr: impl ToSocketAddrs) -> impl Future<Output = Result<Self>>;
    ///
    /// Returns the local address to which this stream is bound.
    fn local_addr(&self) -> Result<SocketAddr>;

    /// Returns the value of the `TCP_NODELAY` option.
    ///
    /// For more information about this option, see [`set_nodelay`](TcpStream::set_nodelay).
    fn nodelay(&self) -> Result<bool>;

    /// Receives data on the socket from the remote address to which it is connected without
    /// removing that data from the queue.
    ///
    /// Successive calls return the same data.
    fn peek(&self, buf: &mut [u8]) -> impl Future<Output = Result<usize>>;

    /// Returns the remote address to which this stream is connected.
    fn peer_addr(&self) -> Result<SocketAddr>;

    /// Sets the value of the `TCP_NODELAY` option.
    ///
    /// If set, this option disables the Nagle algorithm. This means that segments are always sent as soon as possible, even if there is only a small amount of data. When not set, data is buffered until there is a sufficient amount to send out, thereby avoiding the frequent sending of small packets.
    fn set_nodelay(&self, nodelay: bool) -> Result<()>;

    /// Sets the value of the `IP_TTL` option.
    ///
    /// This value sets the time-to-live field that is used in every sent packet.
    fn set_ttl(&self, ttl: u32) -> Result<()>;

    /// Returns the value of the `IP_TTL` option.
    ///
    /// For more information about this option, see [`set_ttl`](TcpStream::set_ttl).
    fn ttl(&self) -> Result<u32>;
}

/// A TCP socket server, listening for connections.
///
/// You can accept a new connection with [`accept`](TcpListener::accept).
pub trait TcpListener: AsFd + AsRawFd + Sized {
    /// Accepts a new incoming connection.
    ///
    /// This function will yield once a new TCP connection is established. When established,
    /// the corresponding [`TcpStream`] and the remote peer's address will be returned.
    fn accept(&self) -> impl Future<Output = Result<(impl TcpStream, SocketAddr)>>;

    /// Creates a new `TcpListener`, which will be bound to the specified address.
    ///
    /// Binding to port 0 will request that the OS assigns a port. The result can be queried
    /// with [`local_addr`](TcpListener::local_addr).
    fn bind(addr: impl ToSocketAddrs) -> impl Future<Output = Result<Self>>;

    /// Returns the local address to which this listener is bound.
    fn local_addr(&self) -> Result<SocketAddr>;

    /// Sets the value of the `IP_TTL` option.
    ///
    /// This value sets the time-to-live field that is used in every sent packet.
    fn set_ttl(&self, ttl: u32) -> Result<()>;

    /// Returns the value of the `IP_TTL` option.
    ///
    /// For more information about this option, see [`set_ttl`](TcpListener::set_ttl).
    fn ttl(&self) -> Result<u32>;
}

/// A UDP socket.
///
/// UDP is “connectionless”, unlike TCP. Meaning, regardless of what address you’ve bound to, a `UdpSocket` is free to communicate with many different remotes. There are basically two main ways to use `UdpSocket`:
/// - one to many: [`bind`](UdpSocket::bind) and use [`send_to`](UdpSocket::send_to) and
///   [`recv_from`](UdpSocket::recv_from) to communicate with many different addresses
/// - one to one: [`connect`](UdpSocket::connect) and associate with a single address, using
///   [`send`](UdpSocket::send) and [`recv`](UdpSocket::recv) to communicate only with that remote
///   address
pub trait UdpSocket: AsFd + AsRawFd + Sized {
    /// Creates a new UDP socket bound to `addr`.
    ///
    /// Binding to port 0 will request that the OS assigns a port. The result can be queried
    /// with [`local_addr`](UdpSocket::local_addr).
    fn bind(addr: impl ToSocketAddrs) -> impl Future<Output = Result<Self>>;

    /// Returns the value of the `SO_BROADCAST` option.
    ///
    /// For more information about this option, see [`set_broadcast`](UdpSocket::set_broadcast).
    fn broadcast(&self) -> Result<bool>;

    /// Connects the UDP socket, setting the default destination for [`send`](UdpSocket::send)
    /// to `addr` and limiting packets that are read with [`recv`](UdpSocket::recv) to those received
    /// from `addr`.
    fn connect(&self, addr: impl ToSocketAddrs) -> impl Future<Output = Result<()>>;

    /// Executes an operation of the `IP_ADD_MEMBERSHIP` type.
    ///
    /// This function specifies a new multicast group for this socket to join. The address must
    /// be a valid multicast address, and `interface` is the address of the local interface with
    /// which the system should join the multicast group. If it's equal to
    /// [`INADDR_ANY`](std::net::Ipv4Addr::UNSPECIFIED), then
    /// an appropriate interface is chosen by the system.
    fn join_multicast_v4(&self, multiaddr: Ipv4Addr, interface: Ipv4Addr) -> Result<()>;

    /// Executes an operation of the `IPV6_ADD_MEMBERSHIP` type.
    ///
    /// This function specifies a new multicast group for this socket to join. The addres must
    /// be a valid multicast address, and `interface` is the index of the interface to
    /// join/leave (or 0 to indicate any interface).
    fn join_multicast_v6(&self, multiaddr: &Ipv6Addr, interface: u32) -> Result<()>;

    /// Executes an operation of the `IP_DROP_MEMBERSHIP` type.
    ///
    /// For more information about this option, see [`join_multicast_v4`](UdpSocket::join_multicast_v4).
    fn leave_multicast_v4(&self, multiaddr: Ipv4Addr, interface: Ipv4Addr) -> Result<()>;

    /// Executes an operation of the `IPV6_DROP_MEMBERSHIP` type.
    ///
    /// For more information about this option, see [`join_multicast_v6`](UdpSocket::join_multicast_v6).
    fn leave_multicast_v6(&self, multiaddr: &Ipv6Addr, interface: u32) -> Result<()>;

    /// Returns the local address to which this socket is bound.
    fn local_addr(&self) -> Result<SocketAddr>;

    /// Gets the value of the `IP_MULTICAST_LOOP` option.
    ///
    /// For more information about this option, see [`set_multicast_loop_v4`](UdpSocket::set_multicast_loop_v4).
    fn multicast_loop_v4(&self) -> Result<bool>;

    /// Gets the value of the `IPV6_MULTICAST_LOOP` option.
    ///
    /// For more information about this option, see [`set_multicast_loop_v6`](UdpSocket::set_multicast_loop_v6).
    fn multicast_loop_v6(&self) -> Result<bool>;

    /// Gets the value of the `IP_MULTICAST_TTL` option.
    ///
    /// For more information about this option, see [`set_multicast_ttl_v4`](UdpSocket::set_multicast_ttl_v4).
    fn multicast_ttl_v4(&self) -> Result<u32>;

    /// Receives a single datagram message from the connected address without removing it from the
    /// queue. On success, returns the number of bytes read from whence the data came.
    fn peek(&self, buf: &mut [u8]) -> impl Future<Output = Result<usize>>;

    /// Receives a single datagram message from the socket without removing it from the queue.
    /// On success, returns the number of bytes read and the address from whence the data came.
    fn peek_from(&self, buf: &mut [u8]) -> impl Future<Output = Result<(usize, SocketAddr)>>;

    /// Returns the socket address of the remote peer to which this socket is connected.
    fn peer_addr(&self) -> Result<SocketAddr>;

    /// Receives a single datagram message on the socket from the remote address to which it is
    /// connected. On success, returns the number of bytes read.
    ///
    /// If a message is too long to fit in `buf`, excess bytes may be discarded.
    /// This method will fail if the socket is not [`connect`](UdpSocket::connect)ed.
    fn recv(&self, buf: &mut [u8]) -> impl Future<Output = Result<usize>>;

    /// Receives a single datagram message on the socket. On success, returns the number of
    /// bytes read and the origin.
    ///
    /// If a message is too long to fit in `buf`, excess bytes may be discarded.
    fn recv_from(&self, buf: &mut [u8]) -> impl Future<Output = Result<(usize, SocketAddr)>>;

    /// Sends data on the socket to the remote address to which it is connected.
    ///
    /// On success, returns the number of bytes written.
    ///
    /// This method will fail if the socket is not [`connect`](UdpSocket::connect)ed.
    fn send(&self, buf: &[u8]) -> impl Future<Output = Result<usize>>;

    /// Sends data on the socket to the given address. On success, returns the number of bytes
    /// written.
    ///
    /// Fails if the IP version of the local socket does not match that of `addr`.
    fn send_to(&self, buf: &[u8], addr: impl ToSocketAddrs) -> impl Future<Output = Result<usize>>;

    /// Sets the value of the `SO_BROADCAST` option.
    ///
    /// When enabled, this socket is allowed to send packets to a broadcast address.
    fn set_broadcast(&self, on: bool) -> Result<()>;

    /// Sets the value of the `IP_MULTICAST_LOOP` option.
    ///
    /// If enabled, multicast packets will be looped back to the local socket.
    ///
    /// This may not have any effect on IPv6 sockets.
    fn set_multicast_loop_v4(&self, on: bool) -> Result<()>;

    /// Sets the value of the `IPV6_MULTICAST_LOOP` option.
    ///
    /// Controls whether this socket sees the multicast packets that it sends itself.
    ///
    /// This may not have any effect on IPv4 sockets.
    fn set_multicast_loop_v6(&self, on: bool) -> Result<()>;

    /// Sets the value of the `IP_MULTICAST_TTL` option.
    ///
    /// Indicates the time-to-live value of outgoing multicast packets for this socket. The
    /// default value is 1, which means that multicast packets don't leave the local network
    /// unless explicitly requested.
    ///
    /// This may not have any effect on IPv6 sockets.
    fn set_multicast_ttl_v4(&self, ttl: u32) -> Result<()>;

    /// Sets the value of the `IP_TTL` option.
    ///
    /// This value sets the time-to-live field that is used in every packet sent from this
    /// socket.
    fn set_ttl(&self, ttl: u32) -> Result<()>;

    /// Returns the value of the `IP_TTL` option.
    ///
    /// For more information about this option, see [`set_ttl`](UdpSocket::set_ttl).
    fn ttl(&self) -> Result<u32>;
}

/// A runtime with networking.
pub trait RuntimeNet {
    type TcpStream: TcpStream;
    type TcpListener: TcpListener;
    type UdpSocket: UdpSocket;

    fn get_socket_addrs(kind: SocketAddrsKind) -> impl Future<Output = Result<Vec<SocketAddr>>>;
}
