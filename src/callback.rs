//! Types for extending Rustin's behavior.

use std::pin::Pin;

use futures::{
    future::{ok, ready},
    stream::{empty, once},
    Future,
    Stream,
};

use crate::{
    error::Error,
    message::OutgoingMessage,
    robot::handle::Handle,
    store::Store,
};

/// A callback that receives incoming messages and reacts to them however it wishes.
pub trait Callback<S> {
    /// Invokes the callback with the incoming message that triggered it.
    fn call(&self, handle: Handle<S>) -> FutureActionStream where S: Store;

    /// Returns the prefix that should be used for namespacing any data stored by the callback.
    fn prefix(&self) -> &'static str;
}

impl<F, S> Callback<S> for F
where
    F: Fn(Handle<S>) -> FutureActionStream,
    S: Store,
{
    fn call(&self, handle: Handle<S>) -> FutureActionStream {
        self(handle)
    }

    fn prefix(&self) -> &'static str {
        "anonymous"
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
}

impl From<Action> for ActionStream {
    fn from(action: Action) -> Self {
        Box::pin(once(ready(action)))
    }
}

impl From<Action> for FutureActionStream {
    fn from(action: Action) -> Self {
        Box::pin(ok(action.into()))
    }
}

/// An asynchronous stream of actions initiated by a callback.
pub type ActionStream = Pin<Box<dyn Stream<Item = Action>>>;

/// A future that resolves to an `ActionStream` or an `Error`. This type is returned by callbacks.
pub type FutureActionStream = Pin<Box<dyn Future<Output = Result<ActionStream, Error>>>>;
