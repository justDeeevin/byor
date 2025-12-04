use crate::{channel::oneshot::*, runtime::Tokio};
use std::task::{Context, Poll};

impl<T> Sender<T> for tokio::sync::oneshot::Sender<T> {
    fn send(self, t: T) -> Result<(), T> {
        self.send(t)
    }

    fn is_closed(&self) -> bool {
        self.is_closed()
    }

    fn poll_closed(&mut self, cx: &mut Context<'_>) -> Poll<()> {
        self.poll_closed(cx)
    }
}

impl<T> Receiver<T> for tokio::sync::oneshot::Receiver<T> {
    type TryRecvError = tokio::sync::oneshot::error::TryRecvError;
    type RecvError = tokio::sync::oneshot::error::RecvError;

    fn close(&mut self) {
        self.close()
    }

    fn try_recv(&mut self) -> Result<Option<T>, Self::TryRecvError> {
        match self.try_recv() {
            Ok(t) => Ok(Some(t)),
            Err(Self::TryRecvError::Closed) => Err(Self::TryRecvError::Closed),
            Err(Self::TryRecvError::Empty) => Ok(None),
        }
    }
}

impl RuntimeOneshot for Tokio {
    type OneshotSender<T> = tokio::sync::oneshot::Sender<T>;
    type OneshotReceiver<T> = tokio::sync::oneshot::Receiver<T>;
}
