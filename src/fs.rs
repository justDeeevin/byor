//! Filysystem operations.
//!
//! This is essentially an async-compatible copy of the [`std::fs`] module. See the documentation
//! there for more details.

use futures_lite::{AsyncRead, AsyncSeek, AsyncWrite, Stream};
use std::{
    ffi::OsString,
    fs::{FileType, Metadata, Permissions},
    io::Result,
    os::fd::AsRawFd,
    path::{Path, PathBuf},
};

pub trait Fs {
    /// The file struct associated with this runtime.
    type File: File;

    /// Returns the canonical, absolute form of a path with all intermediate components normalized
    /// and symbolic links resolved.
    fn canonocalize(path: impl AsRef<Path>) -> impl Future<Output = Result<PathBuf>>;

    /// Copies the contents of one file to another. This function will also copy the permission
    /// bits of the original file to the destination file.
    ///
    /// This function will **overwrite** the contents of `to`.
    ///
    /// Note that if from and to both point to the same file, then the file will likely get truncated by this operation.
    ///
    /// On success, the total number of bytes copied is returned and it is equal to the length of the to file as reported by metadata.
    fn copy(from: impl AsRef<Path>, to: impl AsRef<Path>) -> impl Future<Output = Result<u64>>;

    /// Creates a new, empty directory at theh provided path.
    fn create_dir(path: impl AsRef<Path>) -> impl Future<Output = Result<()>>;

    /// Recursively create a directory and all of its parent components if they are missing.
    ///
    /// This function is not atomic. If it returns an error, any parent components it was able to
    /// create will remain.
    fn create_dir_all(path: impl AsRef<Path>) -> impl Future<Output = Result<()>>;

    /// Creates a new hard link on the filesystem.
    fn hard_link(
        target: impl AsRef<Path>,
        link: impl AsRef<Path>,
    ) -> impl Future<Output = Result<()>>;

    /// Given a path, queries the file system to get information about a file, directory, etc.
    ///
    /// This function will traverse symbolic links to query information about their targets. **If
    /// you wish to query information about a link itself, use [`symlink_metadata`](Fs::symlink_metadata)** instead.
    fn metadata(path: impl AsRef<Path>) -> impl Future<Output = Result<Metadata>>;

    /// Reads the entire contents of a file into a bytes vector.
    fn read(path: impl AsRef<Path>) -> impl Future<Output = Result<Vec<u8>>>;

    /// Returns an stream of the entries within a directory.
    ///
    /// The order of entries is unspecified.
    fn read_dir(
        path: impl AsRef<Path>,
    ) -> impl Future<Output = Result<impl Stream<Item = Result<impl DirEntry>>>>;

    /// Reads a symbolic link, returning the file to which it points.
    fn read_link(path: impl AsRef<Path>) -> impl Future<Output = Result<PathBuf>>;

    /// Reads the entire contents of a file into a string.
    fn read_to_string(path: impl AsRef<Path>) -> impl Future<Output = Result<String>>;

    /// Removes an **empty** directory.
    ///
    /// If you want to remove a non-empty directory as well as all of its contents recursively, use
    /// [`remove_dir_all`](Fs::remove_dir_all) instead.
    fn remove_dir(path: impl AsRef<Path>) -> impl Future<Output = Result<()>>;

    /// Removes a directory after recursiveely removing all of its contents. **Use carefully!**
    ///
    /// This function does **not** follow symbolic links, simply removing the links themselves.
    fn remove_dir_all(path: impl AsRef<Path>) -> impl Future<Output = Result<()>>;

    /// Removes a file from the filesystem.
    fn remove_file(path: impl AsRef<Path>) -> impl Future<Output = Result<()>>;

    /// Renames a file or directory, replacing any existing file or directory at the new path.
    ///
    /// This will not work if the new name is on a different storage device.
    fn rename(from: impl AsRef<Path>, to: impl AsRef<Path>) -> impl Future<Output = Result<()>>;

    /// Changes the permissions found on a file or directory.
    ///
    /// # Symlinks
    ///
    /// On UNIX-like systems, this function will update the permission bits of a symlink's target.
    ///
    /// Note that this behavior can lead to privilege escalation vulnerabilities, where the ability to create a symlink in one directory allows you to cause the permissions of another file or directory to be modified.
    ///
    /// For this reason, using this function with symlinks should be avoided. When possible, permissions should be set at creation time instead.
    fn set_permissions(
        path: impl AsRef<Path>,
        perm: Permissions,
    ) -> impl Future<Output = Result<()>>;

    /// Queries the metadata about a file without following symlinks.
    fn symlink_metadata(path: impl AsRef<Path>) -> impl Future<Output = Result<Metadata>>;

    /// Writes a slice as the entire contents of a file.
    ///
    /// This will create a file if it does not exist, and will entirely replace its contents if it does.
    fn write(
        path: impl AsRef<Path>,
        contents: impl AsRef<[u8]>,
    ) -> impl Future<Output = Result<()>>;
}

/// An entry returned by [`read_dir`](Fs::read_dir).
pub trait DirEntry {
    /// Returns the file name of this directory entry without any leading path components.
    fn file_name(&self) -> OsString;

    /// Returns the file type of the file at which this entry points.
    ///
    /// This will not traverse symlinks.
    fn file_type(&self) -> impl Future<Output = Result<FileType>>;

    /// Returns the metadata fot the file at which this entry points.
    ///
    /// This will not traverse symlinks. To do so, use [`Fs::metadata`] or [`File::metadata`].
    fn metadata(&self) -> impl Future<Output = Result<Metadata>>;

    /// Returns the full path to the file that this entry represents.
    fn path(&self) -> PathBuf;
}

/// An object providing access to an open file on the filesystem.
///
/// An instance of a `File` can be read from and/or written to depending on the options with which
/// it was opened.
///
/// Files are automatically closed when they are dropped. Errors produced when closing are ignored
/// when this happens. Use the method [`sync_all`](File::sync_all) if these errors must be manually
/// handled.
///
/// `File` does not buffer reads and writes. For efficiency, consider using a
/// [`BufReader`](futures_lite::io::BufReader) or [`BufWriter`](futures_lite::io::BufWriter) when performing many
/// small read or write calls, unless unbuffered reads and writes are required.
pub trait File: AsRawFd + AsyncRead + AsyncWrite + AsyncSeek {
    /// Opens a file in write-only mode.
    ///
    /// Creates a file if it does not exist, and truncates it if it does.
    fn create(path: impl AsRef<Path>) -> impl Future<Output = Result<Self>>
    where
        Self: Sized;

    /// Queries metadata about the underlying file.
    fn metadata(&self) -> impl Future<Output = Result<Metadata>>;

    /// Opens a file in read-only mode.
    ///
    /// If you only need to read the entire file contents, consider using [`Fs::read`] or
    /// [`Fs::read_to_string`] instead.
    fn open(path: impl AsRef<Path>) -> impl Future<Output = Result<Self>>
    where
        Self: Sized;

    /// Truncates or extends the underlying file, updating the sive of this file to become `size`.
    ///
    /// When extending a file, the contents are zero-filled.
    ///
    /// The file's cursor isn't changed. In particular, if the cursor was at the end and the file
    /// is shrunk using this operation, the cursor will now be past the end.
    fn set_len(&self, len: u64) -> impl Future<Output = Result<()>>;

    /// Changes the permissions of the underlying file.
    fn set_permissions(&self, perm: Permissions) -> impl Future<Output = Result<()>>;

    /// Syncs all OS-internal file content and metadata to disk.
    ///
    /// If synchronizing the metadata is not required, use [`sync_data`](File::sync_data) instead.
    fn sync_all(&self) -> impl Future<Output = Result<()>>;

    /// This function is similar to [`sync_all`](File::sync_all), except that it might not
    /// synchronize file metadata to the filesystem.
    fn sync_data(&self) -> impl Future<Output = Result<()>>;
}
