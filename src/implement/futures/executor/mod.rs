#[cfg(feature = "exec")]
mod multi_thread;

#[cfg(feature = "local-exec")]
mod local;

impl<T: 'static> crate::executor::Handle for futures::future::RemoteHandle<T> {
    type Out = T;

    fn detach(self) {
        self.forget()
    }

    fn io_result(out: Self::Output) -> std::io::Result<T> {
        Result::Ok(out)
    }
}
