use futures_channel::oneshot::Canceled;

use crate::{
    channel::oneshot::{Receiver, RuntimeOneshot, Sender},
    runtime::Smol,
};

impl<T> Receiver for futures_channel::oneshot::Receiver<T> {
    type Item = T;
    type TryRecvError = Canceled;
    type RecvError = Canceled;

    fn close(&mut self) {
        self.close()
    }

    fn try_recv(&mut self) -> Result<Option<Self::Item>, Self::TryRecvError> {
        self.try_recv()
    }
}

impl<T> Sender for futures_channel::oneshot::Sender<T> {
    type Item = T;

    fn send(self, t: Self::Item) -> Result<(), Self::Item> {
        self.send(t)
    }

    fn is_closed(&self) -> bool {
        self.is_canceled()
    }

    fn poll_closed(&mut self, cx: &mut std::task::Context<'_>) -> std::task::Poll<()> {
        self.poll_canceled(cx)
    }
}

impl RuntimeOneshot for Smol {
    type OneshotSender<T> = futures_channel::oneshot::Sender<T>;
    type OneshotReceiver<T> = futures_channel::oneshot::Receiver<T>;

    fn channel<T>() -> (Self::OneshotSender<T>, Self::OneshotReceiver<T>) {
        futures_channel::oneshot::channel()
    }
}
