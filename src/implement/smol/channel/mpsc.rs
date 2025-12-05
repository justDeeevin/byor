use crate::{channel::mpsc::*, runtime::Smol};
use smol::channel::{Receiver as SmolReceiver, Sender as SmolSender};

impl<T> Sender<T> for smol::channel::Sender<T> {
    type SendError = smol::channel::SendError<T>;

    fn is_closed(&self) -> bool {
        self.is_closed()
    }
}

impl<T: 'static> BoundedSender<T> for SmolSender<T> {
    type TrySendError = smol::channel::TrySendError<T>;

    fn send(&mut self, message: T) -> impl Future<Output = Result<(), Self::SendError>> {
        SmolSender::send(self, message)
    }

    fn try_send(&mut self, message: T) -> Result<(), Self::TrySendError> {
        SmolSender::try_send(self, message)
    }
}

impl<T> Receiver<T> for smol::channel::Receiver<T> {
    type TryRecvError = smol::channel::TryRecvError;

    fn close(&mut self) {
        SmolReceiver::close(self);
    }

    fn try_recv(&mut self) -> Result<Option<T>, Self::TryRecvError> {
        match SmolReceiver::try_recv(self) {
            Ok(message) => Ok(Some(message)),
            Err(Self::TryRecvError::Empty) => Ok(None),
            Err(Self::TryRecvError::Closed) => Err(Self::TryRecvError::Closed),
        }
    }
}

impl<T> UnboundedSender<T> for SmolSender<T> {
    fn send(&self, message: T) -> Result<(), Self::SendError> {
        self.force_send(message).map(|_| ())
    }
}

impl RuntimeMpsc for Smol {
    type BoundedSender<T: 'static> = smol::channel::Sender<T>;
    type BoundedReceiver<T> = smol::channel::Receiver<T>;

    fn bounded_channel<T: 'static>(
        buffer: usize,
    ) -> (Self::BoundedSender<T>, Self::BoundedReceiver<T>) {
        smol::channel::bounded(buffer)
    }

    type UnboundedSender<T: 'static> = smol::channel::Sender<T>;
    type UnboundedReceiver<T> = smol::channel::Receiver<T>;

    fn unbounded_channel<T: 'static>() -> (Self::UnboundedSender<T>, Self::UnboundedReceiver<T>) {
        smol::channel::unbounded()
    }
}
