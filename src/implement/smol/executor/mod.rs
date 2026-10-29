#[cfg(feature = "exec")]
mod multi_thread;

#[cfg(feature = "local-exec")]
mod local;

impl<T: 'static> crate::executor::Handle for smol::Task<T> {
    type Out = T;

    fn detach(self) {
        self.detach();
    }

    fn io_result(out: Self::Output) -> std::io::Result<T> {
        Result::Ok(out)
    }
}
