#[cfg(feature = "exec")]
mod multi_thread;

#[cfg(feature = "local-exec")]
mod local;

impl<T: 'static> crate::executor::Handle<T> for smol::Task<T> {
    type Wrap<U> = U;

    fn detach(self) {
        self.detach();
    }
}
