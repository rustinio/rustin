//! Types for extending Rustin's behavior.

use std::pin::Pin;

use futures::{
    future::{ok, ready},
    stream::{empty, once},
    Future,
    Stream,
};

use crate::error::Error;
use crate::message::{IncomingMessage, OutgoingMessage};
use crate::store::Store;

/// A callback that receives incoming messages and reacts to them however it wishes.
pub trait Callback<S, K> {
    /// Invokes the callback with the incoming message that triggered it.
    fn call(&self, message: &IncomingMessage, store: &S) -> FutureActionStream;
}

impl<F, S> Callback<S, (IncomingMessage,)> for F
where
    F: Fn(&IncomingMessage) -> FutureActionStream,
{
    fn call(&self, message: &IncomingMessage, _store: &S) -> FutureActionStream {
        self(message)
    }
}

impl<F, S> Callback<S, (IncomingMessage, S)> for F
where
    F: Fn(&IncomingMessage, &S) -> FutureActionStream,
    S: Store,
{
    fn call(&self, message: &IncomingMessage, store: &S) -> FutureActionStream {
        self(message, store)
    }
}

/// An action that a callback can take in response to an incoming message.
#[derive(Debug)]
pub enum Action {
    /// Sends a message to the chat service.
    SendMessage(OutgoingMessage),
}

impl Action {
    /// Convenience method for creating an empty `ActionStream`.
    pub fn empty_stream() -> ActionStream {
        Box::pin(empty())
    }

    /// Convenience method for converting a single action into an `ActionStream`.
    pub fn into_action_stream(self) -> ActionStream {
        Box::pin(once(ready(self)))
    }

    /// Convenience method for converting a single action into a `FutureActionStream`.
    pub fn into_future_action_stream(self) -> FutureActionStream {
        Box::pin(ok(self.into_action_stream()))
    }
}

/// An asynchronous stream of actions initiated by a callback.
pub type ActionStream = Pin<Box<dyn Stream<Item = Action>>>;

/// A future that resolves to an `ActionStream` or an `Error`. This type is returned by callbacks.
pub type FutureActionStream = Pin<Box<dyn Future<Output = Result<ActionStream, Error>>>>;
