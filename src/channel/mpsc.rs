//! A multi-producer, single-consumer queue for sending values between asynchronous tasks.

use futures::Stream;

/// Base sender behavior for both unbounded and bounded channels.
pub trait Sender<T>: Clone {
    // TODO: this doesn't need to be an associated type
    type SendError: std::error::Error;

    /// Returns whether the channel is closed.
    fn is_closed(&self) -> bool;
}

/// More extensive behavior for `Sender` implemented by tokio and futures (that is, only _not_
/// implemented by smol).
pub trait SenderExt<T>: Sender<T> {
    /// Completes when the receiver has closed.
    fn closed(&mut self) -> impl Future<Output = ()>;

    /// Checks whether the given channel sends to the same receiver as this channel.
    fn same_channel(&self, other: &Self) -> bool;
}

/// A sender to a channel with a maximum capacity.
pub trait BoundedSender<T>: Sender<T> {
    type TrySendError: std::error::Error;

    /// Sends message, waiting until there is capacity.
    ///
    /// Returns `Err` with the given value if the channel is closed.
    fn send(&mut self, message: T) -> impl Future<Output = Result<(), Self::SendError>>;

    /// Attempts to immediately send a message on the channel.
    fn try_send(&mut self, message: T) -> Result<(), Self::TrySendError>;
}

/// Receiver behavior for both unbounded and bounded channels.
pub trait Receiver<T>: Stream<Item = T> {
    type TryRecvError: std::error::Error;

    /// Closes the channel, preventing any further messages from being sent.
    ///
    /// This allows the receiver to halt incoming messages while being able to drain any pending
    /// ones.
    fn close(&mut self);

    /// Tries to immediately receive a message from the channel.
    ///
    /// Returns `Ok(T)` if it can receive a message, `Ok(None)` if the channel is empty, and `Err`
    /// if the channel is closed.
    fn try_recv(&mut self) -> Result<Option<T>, Self::TryRecvError>;
}

/// A sender to a channel without a maximum capacity.
pub trait UnboundedSender<T>: Sender<T> {
    /// Sends a message.
    ///
    /// Note that this is not marked as async—this method will never block because the channel will
    /// never be full.
    ///
    /// Returrns `Err` with the given value if the channel is closed.
    fn send(&self, message: T) -> Result<(), Self::SendError>;
}
