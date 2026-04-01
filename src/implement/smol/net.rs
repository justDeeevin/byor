use crate::{net::*, runtime::Smol};
use std::{
    io::Result,
    net::{Ipv4Addr, Ipv6Addr, SocketAddr},
};

impl TcpStream for smol::net::TcpStream {
    async fn connect(addr: impl ToSocketAddrs) -> Result<Self> {
        smol::net::TcpStream::connect(Smol::get_socket_addrs(addr.kind()).await?.as_slice()).await
    }
    fn local_addr(&self) -> Result<SocketAddr> {
        self.local_addr()
    }
    fn nodelay(&self) -> Result<bool> {
        self.nodelay()
    }
    fn peek(&self, buf: &mut [u8]) -> impl Future<Output = Result<usize>> {
        self.peek(buf)
    }
    fn peer_addr(&self) -> Result<SocketAddr> {
        self.peer_addr()
    }
    fn set_nodelay(&self, nodelay: bool) -> Result<()> {
        self.set_nodelay(nodelay)
    }
    fn set_ttl(&self, ttl: u32) -> Result<()> {
        self.set_ttl(ttl)
    }
    fn ttl(&self) -> Result<u32> {
        self.ttl()
    }
}

impl TcpListener for smol::net::TcpListener {
    async fn accept(&self) -> Result<(impl TcpStream, SocketAddr)> {
        self.accept().await
    }
    async fn bind(addr: impl ToSocketAddrs) -> Result<Self> {
        smol::net::TcpListener::bind(Smol::get_socket_addrs(addr.kind()).await?.as_slice()).await
    }
    fn local_addr(&self) -> Result<SocketAddr> {
        self.local_addr()
    }
    fn set_ttl(&self, ttl: u32) -> Result<()> {
        self.set_ttl(ttl)
    }
    fn ttl(&self) -> Result<u32> {
        self.ttl()
    }
}

impl UdpSocket for smol::net::UdpSocket {
    async fn bind(addr: impl ToSocketAddrs) -> Result<Self> {
        smol::net::UdpSocket::bind(Smol::get_socket_addrs(addr.kind()).await?.as_slice()).await
    }
    fn broadcast(&self) -> Result<bool> {
        self.broadcast()
    }
    async fn connect(&self, addr: impl ToSocketAddrs) -> Result<()> {
        self.connect(Smol::get_socket_addrs(addr.kind()).await?.as_slice())
            .await
    }
    fn join_multicast_v4(&self, multiaddr: Ipv4Addr, interface: Ipv4Addr) -> Result<()> {
        self.join_multicast_v4(multiaddr, interface)
    }
    fn join_multicast_v6(&self, multiaddr: &Ipv6Addr, interface: u32) -> Result<()> {
        self.join_multicast_v6(multiaddr, interface)
    }
    fn leave_multicast_v4(&self, multiaddr: Ipv4Addr, interface: Ipv4Addr) -> Result<()> {
        self.leave_multicast_v4(multiaddr, interface)
    }
    fn leave_multicast_v6(&self, multiaddr: &Ipv6Addr, interface: u32) -> Result<()> {
        self.leave_multicast_v6(multiaddr, interface)
    }
    fn local_addr(&self) -> Result<SocketAddr> {
        self.local_addr()
    }
    fn multicast_loop_v4(&self) -> Result<bool> {
        self.multicast_loop_v4()
    }
    fn multicast_loop_v6(&self) -> Result<bool> {
        self.multicast_loop_v6()
    }
    fn multicast_ttl_v4(&self) -> Result<u32> {
        self.multicast_ttl_v4()
    }
    fn peek(&self, buf: &mut [u8]) -> impl Future<Output = Result<usize>> {
        self.peek(buf)
    }
    fn peek_from(&self, buf: &mut [u8]) -> impl Future<Output = Result<(usize, SocketAddr)>> {
        self.peek_from(buf)
    }
    fn peer_addr(&self) -> Result<SocketAddr> {
        self.peer_addr()
    }
    fn recv(&self, buf: &mut [u8]) -> impl Future<Output = Result<usize>> {
        self.recv(buf)
    }
    fn recv_from(&self, buf: &mut [u8]) -> impl Future<Output = Result<(usize, SocketAddr)>> {
        self.recv_from(buf)
    }
    fn send(&self, buf: &[u8]) -> impl Future<Output = Result<usize>> {
        self.send(buf)
    }
    async fn send_to(&self, buf: &[u8], addr: impl ToSocketAddrs) -> Result<usize> {
        self.send_to(buf, Smol::get_socket_addrs(addr.kind()).await?.as_slice())
            .await
    }
    fn set_broadcast(&self, on: bool) -> Result<()> {
        self.set_broadcast(on)
    }
    fn set_multicast_loop_v4(&self, on: bool) -> Result<()> {
        self.set_multicast_loop_v4(on)
    }
    fn set_multicast_loop_v6(&self, on: bool) -> Result<()> {
        self.set_multicast_loop_v6(on)
    }
    fn set_multicast_ttl_v4(&self, ttl: u32) -> Result<()> {
        self.set_multicast_ttl_v4(ttl)
    }
    fn set_ttl(&self, ttl: u32) -> Result<()> {
        self.set_ttl(ttl)
    }
    fn ttl(&self) -> Result<u32> {
        self.ttl()
    }
}

impl RuntimeNet for Smol {
    type TcpStream = smol::net::TcpStream;
    type TcpListener = smol::net::TcpListener;
    type UdpSocket = smol::net::UdpSocket;

    async fn get_socket_addrs(kind: SocketAddrsKind<'_>) -> Result<Vec<std::net::SocketAddr>> {
        use smol::net::resolve;
        match kind {
            SocketAddrsKind::Slice(addrs) => resolve(addrs).await,
            SocketAddrsKind::StrPort(host, port) => resolve((host, port)).await,
            SocketAddrsKind::IpAddr(addr, port) => resolve((addr, port)).await,
            SocketAddrsKind::Ipv4Addr(addr, port) => resolve((addr, port)).await,
            SocketAddrsKind::Ipv6Addr(addr, port) => resolve((addr, port)).await,
            SocketAddrsKind::Str(host) => resolve(host).await,
            SocketAddrsKind::SocketAddr(addr) => resolve(addr).await,
            SocketAddrsKind::SocketAddrV4(addr) => resolve(addr).await,
            SocketAddrsKind::SocketAddrV6(addr) => resolve(addr).await,
        }
    }
}
