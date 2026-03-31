//! Asynchronous child process management.

use futures_lite::{AsyncRead, AsyncWrite};
use std::{
    ffi::OsStr,
    io::Result,
    os::fd::AsRawFd,
    path::Path,
    process::{ExitStatus, Output, Stdio},
};

/// A builder for spawning child processes.
pub trait Command {
    type Child: Child;

    /// Constructs a new command for launching the program at the given path with the following
    /// default configuration:
    /// - No arguments
    /// - Inherit its parent's environment
    /// - Inherit its parent's working directory
    /// - Inherit stdin/stdout/stderr for [`spawn`](Self::spawn) or [`status`](Self::status),
    ///   but create a pipe for [`output`](Self::output).
    ///
    /// If `program` is not an absolute path, the `PATH` will be searched in an OS-defined way.
    fn new(program: impl AsRef<OsStr>) -> Self;

    /// Adds a single argument to pass to the program.
    ///
    /// Only one argument can be passed per use. So instead of:
    /// ```no_run
    /// let mut command = Command::new("sh");
    /// command.arg("-C /path/to/repo");
    /// ```
    /// usage would be:
    /// ```no_run
    /// let mut command = tCommand::new("sh");
    /// command.arg("-C").arg("/path/to/repo");
    /// ```
    ///
    /// To pass multiple arguments, see [`args`](Self::args).
    fn arg(&mut self, arg: impl AsRef<OsStr>) -> &mut Self;

    /// Adds multitle arguments to pass to the program.
    ///
    /// To pass a single argument, see [`arg`](Self::arg).
    fn args(&mut self, args: impl IntoIterator<Item = impl AsRef<OsStr>>) -> &mut Self;

    /// Inserts or updates an environment variable mapping.
    ///
    /// Note that environment variable names are case-insensitive (but case-preserving) on
    /// Windows, and case-sensitive on all other platforms.
    fn env(&mut self, key: impl AsRef<OsStr>, val: impl AsRef<OsStr>) -> &mut Self;

    /// Adds or updates multiple environment variable mappings.
    fn envs(
        &mut self,
        vars: impl IntoIterator<Item = (impl AsRef<OsStr>, impl AsRef<OsStr>)>,
    ) -> &mut Self;

    /// Removes an environment variable mapping.
    fn env_remove(&mut self, key: impl AsRef<OsStr>) -> &mut Self;

    /// Removes all environment variable mappings.
    fn env_clear(&mut self) -> &mut Self;

    #[cfg_attr(not(feature = "fs"), allow(rustdoc::broken_intra_doc_links))]
    /// Sets the working directory for the child process.
    ///
    /// # Platform-specific behavior
    /// If the program path is relative (e.g., `"./script.sh"`), it’s ambiguous whether it should be interpreted relative to the parent’s working directory or relative to `current_dir`. The behavior in this case is platform specific and unstable, and it’s recommended to use [`canonicalize`](super::fs::Fs::canonicalize) to get an absolute program path instead.
    fn current_dir(&mut self, dir: impl AsRef<Path>) -> &mut Self;

    /// Configures the child process's standard input (stdin) handle.
    ///
    /// Defaults to [`Stdio::inherit`]
    fn stdin(&mut self, cfg: impl Into<Stdio>) -> &mut Self;

    /// Configures the child process's standard output (stdout) handle.
    ///
    /// Defaults to [`Stdio::inherit`] when used with [`spawn`](Self::spawn) or
    /// [`status`](Self::status), and defaults to [`Stdio::piped`] when used with
    /// [`output`](Self::output).
    fn stdout(&mut self, cfg: impl Into<Stdio>) -> &mut Self;

    /// Configures the child process's standard error (stderr) handle.
    ///
    /// Defaults to [`Stdio::inherit`] when used with [`spawn`](Self::spawn) or
    /// [`status`](Self::status), and defaults to [`Stdio::piped`] when used with
    /// [`output`](Self::output).
    fn stderr(&mut self, cfg: impl Into<Stdio>) -> &mut Self;

    /// Configures whether to kill the process when its corresponding [`Child`] handle is dropped.
    ///
    /// Defaults to `false`.
    ///
    /// # Caveats
    ///
    /// On Unix platforms processes must be “reaped” by their parent process after they have exited in order to release all OS resources. A child process which has exited, but has not yet been reaped by its parent is considered a “zombie” process. Such processes continue to count against limits imposed by the system, and having too many zombie processes present can prevent additional processes from being spawned.
    ///
    /// Although issuing a `kill` signal to the child process is a synchronous operation, the resulting zombie process cannot be `.await`ed inside of the destructor to avoid blocking other tasks. Both tokio and smol will, on a best-effort basis, attempt to reap and clean up such processes in the background, but no additional guarantees are made with regard to how quickly or how often this procedure will take place.
    ///
    /// If stronger guarantees are required, it is recommended to avoid dropping a `Child` handle where possible, and instead utilize [`child.status().await`](Child::status) or [`child.kill(); child.status().await`](Child::kill) where possible.
    fn kill_on_drop(&mut self, kill_on_drop: bool) -> &mut Self;

    /// Executes the command as a child process, returning a handle to it.
    fn spawn(&mut self) -> Result<Self::Child>;

    /// Executes the command, waits for it to exit, and returns its exit status.
    ///
    /// The destructor of the returned future will kill the child if [`kill_on_drop`](Self::kill_on_drop) is enabled.
    fn status(&mut self) -> impl Future<Output = Result<ExitStatus>>;

    /// Executes the command, waits for it to exit, and collects its output.
    ///
    /// > **Note:** this method, unlike in `std`, will unconditionally configure stdout and
    /// > stderr to be pipes, even if they have been previously configured. If this is not
    /// > desired, then [`spawn`](Self::spawn) should be used in combination with
    /// > [`Child::output`].
    ///
    /// The destructor of the returned future will kill the child if [`kill_on_drop`](Self::kill_on_drop) is enabled.
    fn output(&mut self) -> impl Future<Output = Result<Output>>;
}

#[cfg(unix)]
/// Unix-specific extensions to [`Command`].
pub trait CommandExt: Command {
    /// Sets the child process's user ID. This translates to a `setuid` call in the child
    /// process. Failure in the `setuid` call will cause the spawn to fail.
    fn uid(&mut self, uid: u32) -> &mut Self;

    /// Similar to [`uid`](Self::uid), but sets the group ID of the child process. This has the
    /// same semantics as the uid field.
    fn gid(&mut self, gid: u32) -> &mut Self;

    /// Sets the executable argument.
    ///
    /// Sets the first process argument, `argv[0]`, to something other than the default
    /// executable path.
    fn arg0(&mut self, arg0: impl AsRef<OsStr>) -> &mut Self;
}

/// A spawned child process.
///
/// If dropped, the child process will still run in the background unless [`kill_on_drop`](Command::kill_on_drop) is enabled.
pub trait Child {
    type Stdin: AsyncStdio + AsyncWrite;
    type StdinRef<'a>: AsyncWrite
    where
        Self: 'a;
    type Stdout: AsyncStdio + AsyncRead;
    type StdoutRef<'a>: AsyncRead
    where
        Self: 'a;
    type Stderr: AsyncStdio + AsyncRead;
    type StderrRef<'a>: AsyncRead
    where
        Self: 'a;

    /// The handle for writing to the child's standard input (stdin), if it has been captured.
    fn stdin(&mut self) -> Option<Self::StdinRef<'_>>;
    /// Takes ownership of the handle to stdin.
    fn take_stdin(&mut self) -> Option<Self::Stdin>;
    /// The handle for reading from the child's standard output (stdout), if it has been captured.
    fn stdout(&mut self) -> Option<Self::StdoutRef<'_>>;
    /// Takes ownership of the handle to stdout.
    fn take_stdout(&mut self) -> Option<Self::Stdout>;
    /// The handle for reading from the child's standard error (stderr), if it has been captured.
    fn stderr(&mut self) -> Option<Self::StderrRef<'_>>;
    /// Takes ownership of the handle to stderr.
    fn take_stderr(&mut self) -> Option<Self::Stderr>;

    /// Returns the OS-assigned process ID of this child while it is still running.
    ///
    /// # Tokio
    ///
    /// Under tokio, once the child has been polled to completion, this will return `None`. This is done to
    /// avoid confusion on platforms like Unix where the OS ID could be reused once the process
    /// has completed.
    fn id(&self) -> Option<u32>;

    /// Forces the child process to exit, **but does not wait for it to do so**[^1].
    ///
    /// On Unix, this is the equivalent to sending a `SIGKILL`.
    ///
    /// [^1]: Note that tokio's kill method is async, waiting for the process to die. **byor's kill corresponds to tokio's
    /// `start_kill`**
    fn kill(&mut self) -> Result<()>;

    /// Attempts to retrieve the exit status of the child if it has already exited.
    ///
    /// Unlike [`status`](Self::status), this method will not drop the stdin handle.
    fn try_status(&mut self) -> Result<Option<ExitStatus>>;

    /// Waits for the child to exit, returning its exit status.
    ///
    /// The stdin handle to the child process, if any, will be dropped before waiting. This
    /// helps avoid deadlock; it ensures that the child does not block waiting for input from
    /// the parent while the parent waits for the child to exit.
    fn status(&mut self) -> impl Future<Output = Result<ExitStatus>>;

    /// Waits for the child to exit and collects its output.
    ///
    /// The stdin handle to the child process will be dropped before waiting. See
    /// [`status`](Self::status) for details.
    fn output(self) -> impl Future<Output = Result<Output>>;
}

/// Behavior for all child process stdio (stdin, stdout, stderr) handles.
pub trait AsyncStdio: AsRawFd {
    fn into_stdio(self) -> impl Future<Output = Result<Stdio>>;
}
