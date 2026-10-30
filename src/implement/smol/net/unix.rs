use crate::net::unix::*;

impl UnixStream for smol::net::unix::UnixStream {
    fn connect(path: impl AsRef<std::path::Path>) -> impl Future<Output = std::io::Result<Self>> {
        Self::connect(path)
    }

    fn pair() -> std::io::Result<(Self, Self)> {
        Self::pair()
    }

    fn local_addr(&self) -> std::io::Result<std::os::unix::net::SocketAddr> {
        self.local_addr()
    }

    fn peer_addr(&self) -> std::io::Result<std::os::unix::net::SocketAddr> {
        self.peer_addr()
    }

    fn shutdown(&self, how: std::net::Shutdown) -> std::io::Result<()> {
        self.shutdown(how)
    }
}

impl UnixListener for smol::net::unix::UnixListener {
    type Stream = smol::net::unix::UnixStream;

    fn bind(path: impl AsRef<std::path::Path>) -> std::io::Result<Self> {
        Self::bind(path)
    }

    fn accept(
        &self,
    ) -> impl Future<Output = std::io::Result<(Self::Stream, std::os::unix::net::SocketAddr)>> {
        self.accept()
    }

    fn local_addr(&self) -> std::io::Result<std::os::unix::net::SocketAddr> {
        self.local_addr()
    }
}
