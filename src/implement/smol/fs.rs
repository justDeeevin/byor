use crate::fs::*;
use futures::Stream;

impl Fs for crate::runtime::Smol {
    type File = smol::fs::File;

    fn canonocalize(
        path: impl AsRef<std::path::Path>,
    ) -> impl Future<Output = std::io::Result<std::path::PathBuf>> {
        smol::fs::canonicalize(path)
    }
    fn copy(
        from: impl AsRef<std::path::Path>,
        to: impl AsRef<std::path::Path>,
    ) -> impl Future<Output = std::io::Result<u64>> {
        smol::fs::copy(from, to)
    }
    fn create_dir(path: impl AsRef<std::path::Path>) -> impl Future<Output = std::io::Result<()>> {
        smol::fs::create_dir(path)
    }
    fn create_dir_all(
        path: impl AsRef<std::path::Path>,
    ) -> impl Future<Output = std::io::Result<()>> {
        smol::fs::create_dir_all(path)
    }
    fn hard_link(
        target: impl AsRef<std::path::Path>,
        link: impl AsRef<std::path::Path>,
    ) -> impl Future<Output = std::io::Result<()>> {
        smol::fs::hard_link(target, link)
    }
    fn metadata(
        path: impl AsRef<std::path::Path>,
    ) -> impl Future<Output = std::io::Result<std::fs::Metadata>> {
        smol::fs::metadata(path)
    }
    fn read(path: impl AsRef<std::path::Path>) -> impl Future<Output = std::io::Result<Vec<u8>>> {
        smol::fs::read(path)
    }
    fn read_dir(
        path: impl AsRef<std::path::Path>,
    ) -> impl Future<Output = std::io::Result<impl Stream<Item = std::io::Result<impl DirEntry>>>>
    {
        smol::fs::read_dir(path)
    }
    fn read_link(
        path: impl AsRef<std::path::Path>,
    ) -> impl Future<Output = std::io::Result<std::path::PathBuf>> {
        smol::fs::read_link(path)
    }
    fn read_to_string(
        path: impl AsRef<std::path::Path>,
    ) -> impl Future<Output = std::io::Result<String>> {
        smol::fs::read_to_string(path)
    }
    fn remove_dir(path: impl AsRef<std::path::Path>) -> impl Future<Output = std::io::Result<()>> {
        smol::fs::remove_dir(path)
    }
    fn remove_dir_all(
        path: impl AsRef<std::path::Path>,
    ) -> impl Future<Output = std::io::Result<()>> {
        smol::fs::remove_dir_all(path)
    }
    fn remove_file(path: impl AsRef<std::path::Path>) -> impl Future<Output = std::io::Result<()>> {
        smol::fs::remove_file(path)
    }
    fn rename(
        from: impl AsRef<std::path::Path>,
        to: impl AsRef<std::path::Path>,
    ) -> impl Future<Output = std::io::Result<()>> {
        smol::fs::rename(from, to)
    }
    fn set_permissions(
        path: impl AsRef<std::path::Path>,
        perm: std::fs::Permissions,
    ) -> impl Future<Output = std::io::Result<()>> {
        smol::fs::set_permissions(path, perm)
    }
    fn symlink_metadata(
        path: impl AsRef<std::path::Path>,
    ) -> impl Future<Output = std::io::Result<std::fs::Metadata>> {
        smol::fs::symlink_metadata(path)
    }
    fn write(
        path: impl AsRef<std::path::Path>,
        contents: impl AsRef<[u8]>,
    ) -> impl Future<Output = std::io::Result<()>> {
        smol::fs::write(path, contents)
    }
}

impl DirEntry for smol::fs::DirEntry {
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

impl File for smol::fs::File {
    fn create(path: impl AsRef<std::path::Path>) -> impl Future<Output = std::io::Result<Self>>
    where
        Self: Sized,
    {
        smol::fs::File::create(path)
    }
    fn open(path: impl AsRef<std::path::Path>) -> impl Future<Output = std::io::Result<Self>>
    where
        Self: Sized,
    {
        smol::fs::File::open(path)
    }
    fn set_len(&self, len: u64) -> impl Future<Output = std::io::Result<()>> {
        self.set_len(len)
    }
    fn set_permissions(
        &self,
        perm: std::fs::Permissions,
    ) -> impl Future<Output = std::io::Result<()>> {
        self.set_permissions(perm)
    }
    fn sync_all(&self) -> impl Future<Output = std::io::Result<()>> {
        self.sync_all()
    }
    fn sync_data(&self) -> impl Future<Output = std::io::Result<()>> {
        self.sync_data()
    }
    fn metadata(&self) -> impl Future<Output = std::io::Result<std::fs::Metadata>> {
        self.metadata()
    }
}
