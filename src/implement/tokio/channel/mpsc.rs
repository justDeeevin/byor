use crate::{channel::mpsc::*, runtime::Tokio};
use tokio::sync::mpsc::{Sender as TokioSender, UnboundedSender as TokioUnboundedSender};

impl<T> Sender<T> for TokioSender<T> {
    type SendError = tokio::sync::mpsc::error::SendError<T>;

    fn is_closed(&self) -> bool {
        self.is_closed()
    }
}

impl<T> SenderExt<T> for TokioSender<T> {
    fn closed(&mut self) -> impl Future<Output = ()> {
        TokioSender::closed(self)
    }

    fn same_channel(&self, other: &Self) -> bool {
        self.same_channel(other)
    }
}

impl<T> BoundedSender<T> for TokioSender<T> {
    type TrySendError = tokio::sync::mpsc::error::TrySendError<T>;

    fn send(&mut self, message: T) -> impl Future<Output = Result<(), Self::SendError>> {
        TokioSender::send(self, message)
    }

    fn try_send(&mut self, message: T) -> Result<(), Self::TrySendError> {
        TokioSender::try_send(self, message)
    }
}

impl<T> Receiver<T> for tokio_stream::wrappers::ReceiverStream<T> {
    type TryRecvError = tokio::sync::mpsc::error::TryRecvError;

    fn close(&mut self) {
        self.close()
    }

    fn try_recv(&mut self) -> Result<Option<T>, Self::TryRecvError> {
        match self.as_mut().try_recv() {
            Ok(t) => Ok(Some(t)),
            Err(Self::TryRecvError::Empty) => Ok(None),
            Err(Self::TryRecvError::Disconnected) => Err(Self::TryRecvError::Disconnected),
        }
    }
}

impl<T> Sender<T> for TokioUnboundedSender<T> {
    type SendError = <TokioSender<T> as Sender<T>>::SendError;

    fn is_closed(&self) -> bool {
        self.is_closed()
    }
}

impl<T> SenderExt<T> for TokioUnboundedSender<T> {
    fn closed(&mut self) -> impl Future<Output = ()> {
        TokioUnboundedSender::closed(self)
    }

    fn same_channel(&self, other: &Self) -> bool {
        self.same_channel(other)
    }
}

impl<T> UnboundedSender<T> for TokioUnboundedSender<T> {
    fn send(&self, message: T) -> Result<(), Self::SendError> {
        self.send(message)
    }
}

impl<T> Receiver<T> for tokio_stream::wrappers::UnboundedReceiverStream<T> {
    type TryRecvError = tokio::sync::mpsc::error::TryRecvError;

    fn close(&mut self) {
        self.close()
    }

    fn try_recv(&mut self) -> Result<Option<T>, Self::TryRecvError> {
        match self.as_mut().try_recv() {
            Ok(t) => Ok(Some(t)),
            Err(Self::TryRecvError::Empty) => Ok(None),
            Err(Self::TryRecvError::Disconnected) => Err(Self::TryRecvError::Disconnected),
        }
    }
}

impl RuntimeMpsc for Tokio {
    type BoundedSender<T: 'static> = tokio::sync::mpsc::Sender<T>;
    type BoundedReceiver<T> = tokio_stream::wrappers::ReceiverStream<T>;

    type UnboundedSender<T: 'static> = tokio::sync::mpsc::UnboundedSender<T>;
    type UnboundedReceiver<T> = tokio_stream::wrappers::UnboundedReceiverStream<T>;
}
