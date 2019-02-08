//! Types for extending Rustin's behavior.

use std::marker::Unpin;

use futures::Stream;

use crate::error::Error;
use crate::message::{IncomingMessage, OutgoingMessage};
use crate::storage::Store;

/// A callback that receives incoming messages and reacts to them however it wishes.
pub trait Callback<S, K> {
    /// Invokes the callback with the incoming message that triggered it.
    fn call(&self, message: &IncomingMessage, store: &mut S) -> ActionStream;
}

impl<F, S> Callback<S, (IncomingMessage,)> for F
where
    F: Fn(&IncomingMessage) -> ActionStream,
{
    fn call(&self, message: &IncomingMessage, _store: &mut S) -> ActionStream {
        self(message)
    }
}

impl<F, S> Callback<S, (IncomingMessage, S)> for F
where
    F: Fn(&IncomingMessage, &mut S) -> ActionStream,
    S: Store,
{
    fn call(&self, message: &IncomingMessage, store: &mut S) -> ActionStream {
        self(message, store)
    }
}

/// An action that a callback can take in response to an incoming message.
#[derive(Debug)]
pub enum Action {
    /// Sends a message to the chat service.
    SendMessage(OutgoingMessage),
}

/// An asynchronous stream of actions. This type is returned by callbacks.
pub type ActionStream = Box<dyn Stream<Item = Result<Action, Error>> + Unpin>;
