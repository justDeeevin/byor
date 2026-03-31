use crate::fs::*;
use futures_lite::Stream;
use tokio_util::compat::{Compat, TokioAsyncReadCompatExt};

impl Fs for crate::runtime::Tokio {
    type File = Compat<tokio::fs::File>;
    type DirEntry = tokio::fs::DirEntry;

    fn canonicalize(
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
    ) -> std::io::Result<impl Stream<Item = std::io::Result<Self::DirEntry>>> {
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
    type OpenOptions = tokio::fs::OpenOptions;

    async fn create(path: impl AsRef<std::path::Path>) -> std::io::Result<Self> {
        Ok(tokio::fs::File::create(path).await?.compat())
    }
    async fn create_new(path: impl AsRef<std::path::Path>) -> std::io::Result<Self> {
        Ok(tokio::fs::File::create_new(path).await?.compat())
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

impl OpenOptions for tokio::fs::OpenOptions {
    fn new() -> Self {
        Self::new()
    }
    fn read(&mut self, read: bool) -> &mut Self {
        self.read(read)
    }
    fn write(&mut self, write: bool) -> &mut Self {
        self.write(write)
    }
    fn append(&mut self, append: bool) -> &mut Self {
        self.append(append)
    }
    fn truncate(&mut self, truncate: bool) -> &mut Self {
        self.truncate(truncate)
    }
    fn create(&mut self, create: bool) -> &mut Self {
        self.create(create)
    }
    fn create_new(&mut self, create_new: bool) -> &mut Self {
        self.create_new(create_new)
    }
    async fn open(
        &self,
        path: impl AsRef<std::path::Path>,
    ) -> std::io::Result<impl File<OpenOptions = Self>> {
        Ok(self.open(path).await?.compat())
    }
}
