use std::{
    ffi::OsString,
    fs::{FileType, Metadata, Permissions},
    io::Result,
    os::fd::AsRawFd,
    path::{Path, PathBuf},
};

use futures::{AsyncRead, AsyncSeek, AsyncWrite, Stream};

pub trait Fs {
    type File: File;

    fn canonocalize(path: impl AsRef<Path>) -> impl Future<Output = Result<PathBuf>>;
    fn copy(from: impl AsRef<Path>, to: impl AsRef<Path>) -> impl Future<Output = Result<u64>>;
    fn create_dir(path: impl AsRef<Path>) -> impl Future<Output = Result<()>>;
    fn create_dir_all(path: impl AsRef<Path>) -> impl Future<Output = Result<()>>;
    fn hard_link(
        target: impl AsRef<Path>,
        link: impl AsRef<Path>,
    ) -> impl Future<Output = Result<()>>;
    fn metadata(path: impl AsRef<Path>) -> impl Future<Output = Result<Metadata>>;
    fn read(path: impl AsRef<Path>) -> impl Future<Output = Result<Vec<u8>>>;
    fn read_dir(
        path: impl AsRef<Path>,
    ) -> impl Future<Output = Result<impl Stream<Item = Result<impl DirEntry>>>>;
    fn read_link(path: impl AsRef<Path>) -> impl Future<Output = Result<PathBuf>>;
    fn read_to_string(path: impl AsRef<Path>) -> impl Future<Output = Result<String>>;
    fn remove_dir(path: impl AsRef<Path>) -> impl Future<Output = Result<()>>;
    fn remove_dir_all(path: impl AsRef<Path>) -> impl Future<Output = Result<()>>;
    fn remove_file(path: impl AsRef<Path>) -> impl Future<Output = Result<()>>;
    fn rename(from: impl AsRef<Path>, to: impl AsRef<Path>) -> impl Future<Output = Result<()>>;
    fn set_permissions(
        path: impl AsRef<Path>,
        perm: Permissions,
    ) -> impl Future<Output = Result<()>>;
    fn symlink_metadata(path: impl AsRef<Path>) -> impl Future<Output = Result<Metadata>>;
    fn write(
        path: impl AsRef<Path>,
        contents: impl AsRef<[u8]>,
    ) -> impl Future<Output = Result<()>>;
}

pub trait DirEntry {
    fn file_name(&self) -> OsString;
    fn file_type(&self) -> impl Future<Output = Result<FileType>>;
    fn metadata(&self) -> impl Future<Output = Result<Metadata>>;
    fn path(&self) -> PathBuf;
}

pub trait File: AsRawFd + AsyncRead + AsyncWrite + AsyncSeek {
    fn create(path: impl AsRef<Path>) -> impl Future<Output = Result<Self>>
    where
        Self: Sized;
    fn metadata(&self) -> impl Future<Output = Result<Metadata>>;
    fn open(path: impl AsRef<Path>) -> impl Future<Output = Result<Self>>
    where
        Self: Sized;
    fn set_len(&self, len: u64) -> impl Future<Output = Result<()>>;
    fn set_permissions(&self, perm: Permissions) -> impl Future<Output = Result<()>>;
    fn sync_all(&self) -> impl Future<Output = Result<()>>;
    fn sync_data(&self) -> impl Future<Output = Result<()>>;
}
