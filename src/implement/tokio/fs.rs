use crate::fs::*;
use futures::Stream;
use tokio_util::compat::{Compat, TokioAsyncReadCompatExt};

impl Fs for crate::runtime::Tokio {
    type File = Compat<tokio::fs::File>;

    fn canonocalize(
        path: impl AsRef<std::path::Path>,
    ) -> impl Future<Output = std::io::Result<std::path::PathBuf>> {
        tokio::fs::canonicalize(path)
    }
    fn copy(
        from: impl AsRef<std::path::Path>,
        to: impl AsRef<std::path::Path>,
    ) -> impl Future<Output = std::io::Result<u64>> {
        tokio::fs::copy(from, to)
    }
    fn create_dir(path: impl AsRef<std::path::Path>) -> impl Future<Output = std::io::Result<()>> {
        tokio::fs::create_dir(path)
    }
    fn create_dir_all(
        path: impl AsRef<std::path::Path>,
    ) -> impl Future<Output = std::io::Result<()>> {
        tokio::fs::create_dir_all(path)
    }
    fn hard_link(
        target: impl AsRef<std::path::Path>,
        link: impl AsRef<std::path::Path>,
    ) -> impl Future<Output = std::io::Result<()>> {
        tokio::fs::hard_link(target, link)
    }
    fn metadata(
        path: impl AsRef<std::path::Path>,
    ) -> impl Future<Output = std::io::Result<std::fs::Metadata>> {
        tokio::fs::metadata(path)
    }
    fn read(path: impl AsRef<std::path::Path>) -> impl Future<Output = std::io::Result<Vec<u8>>> {
        tokio::fs::read(path)
    }
    async fn read_dir(
        path: impl AsRef<std::path::Path>,
    ) -> std::io::Result<impl Stream<Item = std::io::Result<impl DirEntry>>> {
        Ok(tokio_stream::wrappers::ReadDirStream::new(
            tokio::fs::read_dir(path).await?,
        ))
    }
    fn read_link(
        path: impl AsRef<std::path::Path>,
    ) -> impl Future<Output = std::io::Result<std::path::PathBuf>> {
        tokio::fs::read_link(path)
    }
    fn read_to_string(
        path: impl AsRef<std::path::Path>,
    ) -> impl Future<Output = std::io::Result<String>> {
        tokio::fs::read_to_string(path)
    }
    fn remove_dir(path: impl AsRef<std::path::Path>) -> impl Future<Output = std::io::Result<()>> {
        tokio::fs::remove_dir(path)
    }
    fn remove_dir_all(
        path: impl AsRef<std::path::Path>,
    ) -> impl Future<Output = std::io::Result<()>> {
        tokio::fs::remove_dir_all(path)
    }
    fn remove_file(path: impl AsRef<std::path::Path>) -> impl Future<Output = std::io::Result<()>> {
        tokio::fs::remove_file(path)
    }
    fn rename(
        from: impl AsRef<std::path::Path>,
        to: impl AsRef<std::path::Path>,
    ) -> impl Future<Output = std::io::Result<()>> {
        tokio::fs::rename(from, to)
    }
    fn set_permissions(
        path: impl AsRef<std::path::Path>,
        perm: std::fs::Permissions,
    ) -> impl Future<Output = std::io::Result<()>> {
        tokio::fs::set_permissions(path, perm)
    }
    fn symlink_metadata(
        path: impl AsRef<std::path::Path>,
    ) -> impl Future<Output = std::io::Result<std::fs::Metadata>> {
        tokio::fs::symlink_metadata(path)
    }
    fn write(
        path: impl AsRef<std::path::Path>,
        contents: impl AsRef<[u8]>,
    ) -> impl Future<Output = std::io::Result<()>> {
        tokio::fs::write(path, contents)
    }
}

impl DirEntry for tokio::fs::DirEntry {
    fn file_name(&self) -> std::ffi::OsString {
        self.file_name()
    }
    fn file_type(&self) -> impl Future<Output = std::io::Result<std::fs::FileType>> {
        self.file_type()
    }
    fn metadata(&self) -> impl Future<Output = std::io::Result<std::fs::Metadata>> {
        self.metadata()
    }
    fn path(&self) -> std::path::PathBuf {
        self.path()
    }
}

impl File for Compat<tokio::fs::File> {
    async fn create(path: impl AsRef<std::path::Path>) -> std::io::Result<Self> {
        Ok(tokio::fs::File::create(path).await?.compat())
    }
    async fn open(path: impl AsRef<std::path::Path>) -> std::io::Result<Self> {
        Ok(tokio::fs::File::open(path).await?.compat())
    }
    fn set_len(&self, len: u64) -> impl Future<Output = std::io::Result<()>> {
        self.get_ref().set_len(len)
    }
    fn set_permissions(
        &self,
        perm: std::fs::Permissions,
    ) -> impl Future<Output = std::io::Result<()>> {
        self.get_ref().set_permissions(perm)
    }
    fn sync_all(&self) -> impl Future<Output = std::io::Result<()>> {
        self.get_ref().sync_all()
    }
    fn sync_data(&self) -> impl Future<Output = std::io::Result<()>> {
        self.get_ref().sync_data()
    }
    fn metadata(&self) -> impl Future<Output = std::io::Result<std::fs::Metadata>> {
        self.get_ref().metadata()
    }
}
