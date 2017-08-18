//! Types for extending Rustin's behavior.

use std::fmt::Debug;
use std::rc::Rc;

use futures::{Async, Poll, Stream};

use error::Error;
use message::{IncomingMessage, OutgoingMessage};

/// A callback that receives incoming messages and reacts to them however it wishes.
pub trait Handler: Debug {
    /// Invokes the handler with the incoming message that triggered it.
    fn call(&self, message: IncomingMessage) -> Box<Stream<Item = Action, Error = Error>>;
}

/// An action that a handler can take in response to an incoming message.
#[derive(Debug)]
pub enum Action {
    /// Sends a message to the chat service.
    SendMessage(OutgoingMessage),
}

/// An asynchronous stream of handlers.
#[derive(Clone, Debug)]
pub struct HandlerStream {
    index: usize,
    inner: Rc<Vec<Rc<Box<Handler>>>>,
}

impl HandlerStream {
    /// Creates a new `HandlerStream` from a vector of handlers.
    pub fn new(handlers: Vec<Box<Handler>>) -> Self {
        HandlerStream {
            index: 0,
            inner: Rc::new(handlers.into_iter().map(|handler| Rc::new(handler)).collect()),
        }
    }
}

impl Stream for HandlerStream {
    type Item = Rc<Box<Handler>>;
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
