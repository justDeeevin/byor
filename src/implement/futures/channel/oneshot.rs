use crate::{channel::oneshot::*, runtime::Futures};
use std::task::{Context, Poll};

impl<T> Sender<T> for futures::channel::oneshot::Sender<T> {
    fn send(self, t: T) -> Result<(), T> {
        self.send(t)
    }

    fn is_closed(&self) -> bool {
        self.is_canceled()
    }

    fn poll_closed(&mut self, cx: &mut Context<'_>) -> Poll<()> {
        self.poll_canceled(cx)
    }
}

impl<T> Receiver<T> for futures::channel::oneshot::Receiver<T> {
    type TryRecvError = futures::channel::oneshot::Canceled;
    type RecvError = futures::channel::oneshot::Canceled;

    fn close(&mut self) {
        self.close()
    }

    fn try_recv(&mut self) -> Result<Option<T>, Self::TryRecvError> {
        self.try_recv()
    }
}

impl RuntimeOneshot for Futures {
    type OneshotSender<T> = futures::channel::oneshot::Sender<T>;
    type OneshotReceiver<T> = futures::channel::oneshot::Receiver<T>;

    fn channel<T>() -> (Self::OneshotSender<T>, Self::OneshotReceiver<T>) {
        futures::channel::oneshot::channel()
    }
}
