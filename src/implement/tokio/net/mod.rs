use crate::{net::*, runtime::Tokio};
use tokio_util::compat::{Compat, TokioAsyncReadCompatExt};

#[cfg(unix)]
mod unix;

impl TcpStream for Compat<tokio::net::TcpStream> {
    async fn connect(addr: impl ToSocketAddrs) -> std::io::Result<Self> {
        Ok(
            tokio::net::TcpStream::connect(Tokio::get_socket_addrs(addr.kind()).await?.as_slice())
                .await?
                .compat(),
        )
    }
    fn local_addr(&self) -> std::io::Result<std::net::SocketAddr> {
        self.get_ref().local_addr()
    }
    fn nodelay(&self) -> std::io::Result<bool> {
        self.get_ref().nodelay()
    }
    fn peek(&self, buf: &mut [u8]) -> impl std::future::Future<Output = std::io::Result<usize>> {
        self.get_ref().peek(buf)
    }
    fn peer_addr(&self) -> std::io::Result<std::net::SocketAddr> {
        self.get_ref().peer_addr()
    }
    fn set_nodelay(&self, nodelay: bool) -> std::io::Result<()> {
        self.get_ref().set_nodelay(nodelay)
    }
    fn set_ttl(&self, ttl: u32) -> std::io::Result<()> {
        self.get_ref().set_ttl(ttl)
    }
    fn ttl(&self) -> std::io::Result<u32> {
        self.get_ref().ttl()
    }
}

impl TcpListener for tokio::net::TcpListener {
    async fn accept(&self) -> std::io::Result<(impl TcpStream, std::net::SocketAddr)> {
        self.accept()
            .await
            .map(|(stream, addr)| (stream.compat(), addr))
    }
    async fn bind(addr: impl ToSocketAddrs) -> std::io::Result<Self> {
        tokio::net::TcpListener::bind(Tokio::get_socket_addrs(addr.kind()).await?.as_slice()).await
    }
    fn local_addr(&self) -> std::io::Result<std::net::SocketAddr> {
        self.local_addr()
    }
    fn set_ttl(&self, ttl: u32) -> std::io::Result<()> {
        self.set_ttl(ttl)
    }
    fn ttl(&self) -> std::io::Result<u32> {
        self.ttl()
    }
}

impl UdpSocket for tokio::net::UdpSocket {
    async fn bind(addr: impl ToSocketAddrs) -> std::io::Result<Self> {
        tokio::net::UdpSocket::bind(Tokio::get_socket_addrs(addr.kind()).await?.as_slice()).await
    }
    fn broadcast(&self) -> std::io::Result<bool> {
        self.broadcast()
    }
    async fn connect(&self, addr: impl ToSocketAddrs) -> std::io::Result<()> {
        self.connect(Tokio::get_socket_addrs(addr.kind()).await?.as_slice())
            .await
    }
    fn join_multicast_v4(
        &self,
        multiaddr: std::net::Ipv4Addr,
        interface: std::net::Ipv4Addr,
    ) -> std::io::Result<()> {
        self.join_multicast_v4(multiaddr, interface)
    }
    fn join_multicast_v6(
        &self,
        multiaddr: &std::net::Ipv6Addr,
        interface: u32,
    ) -> std::io::Result<()> {
        self.join_multicast_v6(multiaddr, interface)
    }
    fn leave_multicast_v4(
        &self,
        multiaddr: std::net::Ipv4Addr,
        interface: std::net::Ipv4Addr,
    ) -> std::io::Result<()> {
        self.leave_multicast_v4(multiaddr, interface)
    }
    fn leave_multicast_v6(
        &self,
        multiaddr: &std::net::Ipv6Addr,
        interface: u32,
    ) -> std::io::Result<()> {
        self.leave_multicast_v6(multiaddr, interface)
    }
    fn local_addr(&self) -> std::io::Result<std::net::SocketAddr> {
        self.local_addr()
    }
    fn multicast_loop_v4(&self) -> std::io::Result<bool> {
        self.multicast_loop_v4()
    }
    fn multicast_loop_v6(&self) -> std::io::Result<bool> {
        self.multicast_loop_v6()
    }
    fn multicast_ttl_v4(&self) -> std::io::Result<u32> {
        self.multicast_ttl_v4()
    }
    fn peek(&self, buf: &mut [u8]) -> impl std::future::Future<Output = std::io::Result<usize>> {
        self.peek(buf)
    }
    fn peek_from(
        &self,
        buf: &mut [u8],
    ) -> impl std::future::Future<Output = std::io::Result<(usize, std::net::SocketAddr)>> {
        self.peek_from(buf)
    }
    fn peer_addr(&self) -> std::io::Result<std::net::SocketAddr> {
        self.peer_addr()
    }
    fn recv(&self, buf: &mut [u8]) -> impl std::future::Future<Output = std::io::Result<usize>> {
        self.recv(buf)
    }
    fn recv_from(
        &self,
        buf: &mut [u8],
    ) -> impl std::future::Future<Output = std::io::Result<(usize, std::net::SocketAddr)>> {
        self.recv_from(buf)
    }
    fn send(&self, buf: &[u8]) -> impl std::future::Future<Output = std::io::Result<usize>> {
        self.send(buf)
    }
    async fn send_to(&self, buf: &[u8], addr: impl ToSocketAddrs) -> std::io::Result<usize> {
        self.send_to(buf, Tokio::get_socket_addrs(addr.kind()).await?.as_slice())
            .await
    }
    fn set_broadcast(&self, on: bool) -> std::io::Result<()> {
        self.set_broadcast(on)
    }
    fn set_multicast_loop_v4(&self, on: bool) -> std::io::Result<()> {
        self.set_multicast_loop_v4(on)
    }
    fn set_multicast_loop_v6(&self, on: bool) -> std::io::Result<()> {
        self.set_multicast_loop_v6(on)
    }
    fn set_multicast_ttl_v4(&self, ttl: u32) -> std::io::Result<()> {
        self.set_multicast_ttl_v4(ttl)
    }
    fn set_ttl(&self, ttl: u32) -> std::io::Result<()> {
        self.set_ttl(ttl)
    }
    fn ttl(&self) -> std::io::Result<u32> {
        self.ttl()
    }
}

impl RuntimeNet for Tokio {
    type TcpStream = Compat<tokio::net::TcpStream>;
    type TcpListener = tokio::net::TcpListener;
    type UdpSocket = tokio::net::UdpSocket;

    async fn get_socket_addrs(
        kind: SocketAddrsKind<'_>,
    ) -> std::io::Result<Vec<std::net::SocketAddr>> {
        use tokio::net::lookup_host;
        match kind {
            SocketAddrsKind::Slice(addrs) => lookup_host(addrs).await.map(Iterator::collect),
            SocketAddrsKind::StrPort(host, port) => {
                lookup_host((host, port)).await.map(Iterator::collect)
            }
            SocketAddrsKind::IpAddr(addr, port) => {
                lookup_host((addr, port)).await.map(Iterator::collect)
            }
            SocketAddrsKind::Ipv4Addr(addr, port) => {
                lookup_host((addr, port)).await.map(Iterator::collect)
            }
            SocketAddrsKind::Ipv6Addr(addr, port) => {
                lookup_host((addr, port)).await.map(Iterator::collect)
            }
            SocketAddrsKind::Str(host) => lookup_host(host).await.map(Iterator::collect),
            SocketAddrsKind::SocketAddr(addr) => lookup_host(addr).await.map(Iterator::collect),
            SocketAddrsKind::SocketAddrV4(addr) => lookup_host(addr).await.map(Iterator::collect),
            SocketAddrsKind::SocketAddrV6(addr) => lookup_host(addr).await.map(Iterator::collect),
        }
    }
}
