//! A channel used for sending a single message between asynchronous tasks.
//!
//! Smol does not provide a oneshot channel, so it does not implement [`RuntimeOneshot`]. However,
//! an implementation using futures's oneshot is provided under the `smol-oneshot` feature if one is
//! needed.

use std::{
    error::Error,
    task::{Context, Poll},
};

pub trait Sender {
    type Item;

    /// Attempts to send a value on this channel, returning it back if it could not be sent.
    ///
    /// This method consumes self as only one value may ever be sent on a oneshot channel.
    fn send(self, t: Self::Item) -> Result<(), Self::Item>;

    /// Waits for the associated [`Receiver`] handle to close.
    ///
    /// A receiver is closed when [`close`](Receiver::close) is called or when it is dropped.
    ///
    /// This is a utility wrapping [`poll_closed`](Sender::poll_closed) to expose a future.
    fn closed(&mut self) -> impl Future<Output = ()> {
        std::future::poll_fn(|cx| self.poll_closed(cx))
    }

    /// Returns `true` if the associated [`Receiver`] handle has been closed.
    ///
    /// A receiver is closed when [`close`](Receiver::close) is called or when it is dropped.
    fn is_closed(&self) -> bool;

    /// Checks whether the oneshot channel has been closed, and if not, schedules the `Waker` in the provided `Context` to receive a notification when the channel is closed.
    fn poll_closed(&mut self, cx: &mut Context<'_>) -> Poll<()>;
}

/// Await a oneshot receiver to yield a value.
pub trait Receiver: Future<Output = Result<Self::Item, Self::RecvError>> {
    type Item;
    type TryRecvError: Error;
    type RecvError: Error;

    /// Closes the channel, preventing the associated [`Sender`] from sending a value.
    fn close(&mut self);

    /// Attempts to receive a message outside of the context of a task.
    ///
    /// Returns `Ok(Some(T))` if a value was received, `Ok(None)` if the channel is empty, and
    /// `Err` if the channel is closed.
    fn try_recv(&mut self) -> Result<Option<Self::Item>, Self::TryRecvError>;
}

/// A runtime with a oneshot channel.
pub trait RuntimeOneshot {
    type OneshotSender<T>: Sender<Item = T>;
    type OneshotReceiver<T>: Receiver<Item = T>;

    fn channel<T>() -> (Self::OneshotSender<T>, Self::OneshotReceiver<T>);
}
