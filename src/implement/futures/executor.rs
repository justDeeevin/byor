use crate::executor::*;

impl Executor for futures::executor::ThreadPool {
    type TaskWrap<T> = T;
    type Task<T: 'static> = futures::future::RemoteHandle<T>;

    type Mutex<T: ?Sized> = futures::lock::Mutex<T>;

    type BoundedSender<T: 'static> = futures::channel::mpsc::Sender<T>;
    type BoundedReceiver<T> = futures::channel::mpsc::Receiver<T>;

    type UnboundedSender<T: 'static> = futures::channel::mpsc::UnboundedSender<T>;
    type UnboundedReceiver<T> = futures::channel::mpsc::UnboundedReceiver<T>;

    fn spawn<T: Send + 'static, F: Future<Output = T> + Send + 'static>(
        &self,
        future: F,
    ) -> Self::Task<T> {
        futures::task::SpawnExt::spawn_with_handle(self, future).unwrap()
    }
}
