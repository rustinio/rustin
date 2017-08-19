//! Types for extending Rustin's behavior.

use std::fmt::Debug;
use std::rc::Rc;

use futures::{Async, Poll, Stream};

use error::Error;
use message::{IncomingMessage, OutgoingMessage};

/// A callback that receives incoming messages and reacts to them however it wishes.
pub trait Callback: Debug {
    /// Invokes the callback with the incoming message that triggered it.
    fn call(&self, message: IncomingMessage) -> Box<Stream<Item = Action, Error = Error>>;
}

/// An action that a callback can take in response to an incoming message.
#[derive(Debug)]
pub enum Action {
    /// Sends a message to the chat service.
    SendMessage(OutgoingMessage),
}

/// An asynchronous stream of callbacks.
#[derive(Clone, Debug)]
pub struct Callbacks {
    index: usize,
    inner: Rc<Vec<Rc<Box<Callback>>>>,
}

impl Callbacks {
    /// Creates a new `Callbacks` from a vector of callbacks.
    pub fn new(callbacks: Vec<Box<Callback>>) -> Self {
        Callbacks {
            index: 0,
            inner: Rc::new(callbacks.into_iter().map(|callback| Rc::new(callback)).collect()),
        }
    }
}

impl Stream for Callbacks {
    type Item = Rc<Box<Callback>>;
    type Error = Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        if self.index >= self.inner.len() {
            return Ok(Async::Ready(None));
        } else {
            self.index += 1;

            return Ok(Async::Ready(Some(self.inner[self.index].clone())));
        }
    }
}
