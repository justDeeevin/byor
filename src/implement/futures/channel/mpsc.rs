use crate::{channel::mpsc::*, runtime::Futures};

impl<T> Sender<T> for futures::channel::mpsc::Sender<T> {
    type SendError = futures::channel::mpsc::SendError;

    fn is_closed(&self) -> bool {
        self.is_closed()
    }
}

impl<T> SenderExt<T> for futures::channel::mpsc::Sender<T> {
    async fn closed(&mut self) {
        while std::future::poll_fn(|cx| self.poll_ready(cx)).await.is_ok() {}
    }

    fn same_channel(&self, other: &Self) -> bool {
        self.same_receiver(other)
    }
}

impl<T: 'static> BoundedSender<T> for futures::channel::mpsc::Sender<T> {
    type TrySendError = futures::channel::mpsc::TrySendError<T>;

    fn send(&mut self, message: T) -> impl Future<Output = Result<(), Self::SendError>> {
        futures::SinkExt::send(self, message)
    }

    fn try_send(&mut self, message: T) -> Result<(), Self::TrySendError> {
        self.try_send(message)
    }
}

impl<T> Receiver<T> for futures::channel::mpsc::Receiver<T> {
    type TryRecvError = futures::channel::mpsc::TryRecvError;

    fn close(&mut self) {
        self.close()
    }

    fn try_recv(&mut self) -> Result<Option<T>, Self::TryRecvError> {
        self.try_next()
    }
}

impl<T: 'static> Sender<T> for futures::channel::mpsc::UnboundedSender<T> {
    type SendError = futures::channel::mpsc::TrySendError<T>;

    fn is_closed(&self) -> bool {
        self.is_closed()
    }
}

impl<T: 'static> SenderExt<T> for futures::channel::mpsc::UnboundedSender<T> {
    async fn closed(&mut self) {
        while std::future::poll_fn(|cx| self.poll_ready(cx)).await.is_ok() {}
    }

    fn same_channel(&self, other: &Self) -> bool {
        self.same_receiver(other)
    }
}

impl<T: 'static> UnboundedSender<T> for futures::channel::mpsc::UnboundedSender<T> {
    fn send(&self, message: T) -> Result<(), Self::SendError> {
        self.unbounded_send(message)
    }
}

impl<T> Receiver<T> for futures::channel::mpsc::UnboundedReceiver<T> {
    type TryRecvError = futures::channel::mpsc::TryRecvError;

    fn close(&mut self) {
        self.close()
    }

    fn try_recv(&mut self) -> Result<Option<T>, Self::TryRecvError> {
        self.try_next()
    }
}

impl RuntimeMpsc for Futures {
    type BoundedSender<T: 'static> = futures::channel::mpsc::Sender<T>;
    type BoundedReceiver<T> = futures::channel::mpsc::Receiver<T>;

    type UnboundedSender<T: 'static> = futures::channel::mpsc::UnboundedSender<T>;
    type UnboundedReceiver<T> = futures::channel::mpsc::UnboundedReceiver<T>;
}
