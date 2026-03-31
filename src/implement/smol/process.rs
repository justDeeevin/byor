use crate::process::*;
#[cfg(unix)]
use async_process::unix::CommandExt as SmolCmdExt;
use async_process::{ChildStderr, ChildStdin, ChildStdout};
use std::{
    ffi::OsStr,
    io::Result,
    process::{ExitStatus, Output, Stdio},
};

impl Command for async_process::Command {
    type Child = async_process::Child;

    fn new(program: impl AsRef<OsStr>) -> Self {
        async_process::Command::new(program)
    }

    fn arg(&mut self, arg: impl AsRef<OsStr>) -> &mut Self {
        self.arg(arg)
    }

    fn args(&mut self, args: impl IntoIterator<Item = impl AsRef<OsStr>>) -> &mut Self {
        self.args(args)
    }

    fn env(&mut self, key: impl AsRef<OsStr>, val: impl AsRef<OsStr>) -> &mut Self {
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
impl CommandExt for async_process::Command {
    fn uid(&mut self, uid: u32) -> &mut Self {
        SmolCmdExt::uid(self, uid)
    }

    fn gid(&mut self, gid: u32) -> &mut Self {
        SmolCmdExt::gid(self, gid)
    }

    fn arg0(&mut self, arg0: impl AsRef<OsStr>) -> &mut Self {
        SmolCmdExt::arg0(self, arg0)
    }
}

impl Child for async_process::Child {
    type Stdin = ChildStdin;
    type StdinRef<'a> = &'a mut ChildStdin;
    type Stdout = ChildStdout;
    type StdoutRef<'a> = &'a mut ChildStdout;
    type Stderr = ChildStderr;
    type StderrRef<'a> = &'a mut ChildStderr;

    fn stdin(&mut self) -> Option<Self::StdinRef<'_>> {
        self.stdin.as_mut()
    }

    fn take_stdin(&mut self) -> Option<Self::Stdin> {
        self.stdin.take()
    }

    fn stdout(&mut self) -> Option<Self::StdoutRef<'_>> {
        self.stdout.as_mut()
    }

    fn take_stdout(&mut self) -> Option<Self::Stdout> {
        self.stdout.take()
    }

    fn stderr(&mut self) -> Option<Self::StderrRef<'_>> {
        self.stderr.as_mut()
    }

    fn take_stderr(&mut self) -> Option<Self::Stderr> {
        self.stderr.take()
    }

    fn id(&self) -> Option<u32> {
        Some(self.id())
    }

    fn kill(&mut self) -> Result<()> {
        self.kill()
    }

    fn try_status(&mut self) -> Result<Option<ExitStatus>> {
        self.try_status()
    }

    fn status(&mut self) -> impl Future<Output = Result<ExitStatus>> {
        self.status()
    }

    fn output(self) -> impl Future<Output = Result<Output>> {
        self.output()
    }
}

impl AsyncStdio for ChildStdin {
    fn into_stdio(self) -> impl Future<Output = Result<Stdio>> {
        self.into_stdio()
    }
}

impl AsyncStdio for ChildStdout {
    fn into_stdio(self) -> impl Future<Output = Result<Stdio>> {
        self.into_stdio()
    }
}

impl AsyncStdio for ChildStderr {
    fn into_stdio(self) -> impl Future<Output = Result<Stdio>> {
        self.into_stdio()
    }
}
