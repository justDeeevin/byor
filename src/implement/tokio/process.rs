use crate::process::*;
use std::{
    ffi::OsStr,
    io::Result,
    process::{ExitStatus, Output, Stdio},
};
use tokio::process::{ChildStderr, ChildStdin, ChildStdout};
use tokio_util::compat::{
    Compat, TokioAsyncReadCompatExt as ReadCompat, TokioAsyncWriteCompatExt as WriteCompat,
};

impl Command for tokio::process::Command {
    type Child = tokio::process::Child;

    fn new(program: impl AsRef<OsStr>) -> Self {
        tokio::process::Command::new(program)
    }

    fn arg(&mut self, arg: impl AsRef<OsStr>) -> &mut Self {
        self.arg(arg)
    }

    fn args(&mut self, args: impl IntoIterator<Item = impl AsRef<OsStr>>) -> &mut Self {
        self.args(args)
    }

    fn env(&mut self, key: impl AsRef<OsStr>, val: impl AsRef<std::ffi::OsStr>) -> &mut Self {
        self.env(key, val)
    }

    fn envs(
        &mut self,
        vars: impl IntoIterator<Item = (impl AsRef<OsStr>, impl AsRef<std::ffi::OsStr>)>,
    ) -> &mut Self {
        self.envs(vars)
    }

    fn env_remove(&mut self, key: impl AsRef<OsStr>) -> &mut Self {
        self.env_remove(key)
    }

    fn env_clear(&mut self) -> &mut Self {
        self.env_clear()
    }

    fn current_dir(&mut self, dir: impl AsRef<std::path::Path>) -> &mut Self {
        self.current_dir(dir)
    }

    fn stdin(&mut self, cfg: impl Into<Stdio>) -> &mut Self {
        self.stdin(cfg)
    }

    fn stdout(&mut self, cfg: impl Into<Stdio>) -> &mut Self {
        self.stdout(cfg)
    }

    fn stderr(&mut self, cfg: impl Into<Stdio>) -> &mut Self {
        self.stderr(cfg)
    }

    fn kill_on_drop(&mut self, kill_on_drop: bool) -> &mut Self {
        self.kill_on_drop(kill_on_drop)
    }

    fn spawn(&mut self) -> Result<Self::Child> {
        self.spawn()
    }

    fn status(&mut self) -> impl Future<Output = Result<ExitStatus>> {
        self.status()
    }

    fn output(&mut self) -> impl Future<Output = Result<Output>> {
        self.output()
    }
}

#[cfg(unix)]
impl CommandExt for tokio::process::Command {
    fn uid(&mut self, uid: u32) -> &mut Self {
        self.uid(uid)
    }

    fn gid(&mut self, gid: u32) -> &mut Self {
        self.gid(gid)
    }

    fn arg0(&mut self, arg0: impl AsRef<OsStr>) -> &mut Self {
        self.arg0(arg0)
    }
}

impl Child for tokio::process::Child {
    type Stdin = Compat<ChildStdin>;
    type StdinRef<'a> = Compat<&'a mut ChildStdin>;
    type Stdout = Compat<ChildStdout>;
    type StdoutRef<'a> = Compat<&'a mut ChildStdout>;
    type Stderr = Compat<ChildStderr>;
    type StderrRef<'a> = Compat<&'a mut ChildStderr>;

    fn stdin(&mut self) -> Option<Self::StdinRef<'_>> {
        self.stdin.as_mut().map(WriteCompat::compat_write)
    }

    fn take_stdin(&mut self) -> Option<Self::Stdin> {
        self.stdin.take().map(WriteCompat::compat_write)
    }

    fn stdout(&mut self) -> Option<Self::StdoutRef<'_>> {
        self.stdout.as_mut().map(ReadCompat::compat)
    }

    fn take_stdout(&mut self) -> Option<Self::Stdout> {
        self.stdout.take().map(ReadCompat::compat)
    }

    fn stderr(&mut self) -> Option<Self::StderrRef<'_>> {
        self.stderr.as_mut().map(ReadCompat::compat)
    }

    fn take_stderr(&mut self) -> Option<Self::Stderr> {
        self.stderr.take().map(ReadCompat::compat)
    }

    fn id(&self) -> Option<u32> {
        self.id()
    }

    fn kill(&mut self) -> Result<()> {
        self.start_kill()
    }

    fn try_status(&mut self) -> Result<Option<ExitStatus>> {
        self.try_wait()
    }

    fn status(&mut self) -> impl Future<Output = Result<ExitStatus>> {
        self.wait()
    }

    fn output(self) -> impl Future<Output = Result<Output>> {
        self.wait_with_output()
    }
}

impl AsyncStdio for Compat<ChildStdin> {
    async fn into_stdio(self) -> Result<Stdio> {
        self.into_inner().try_into()
    }
}

impl AsyncStdio for Compat<ChildStdout> {
    async fn into_stdio(self) -> Result<Stdio> {
        self.into_inner().try_into()
    }
}

impl AsyncStdio for Compat<ChildStderr> {
    async fn into_stdio(self) -> Result<Stdio> {
        self.into_inner().try_into()
    }
}
