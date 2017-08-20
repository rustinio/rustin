//! Types for extending Rustin's behavior.

use std::cell::RefCell;
use std::rc::Rc;

use futures::{Async, Poll, Stream};

use error::Error;
use message::{IncomingMessage, OutgoingMessage};

/// A callback that receives incoming messages and reacts to them however it wishes.
pub trait Callback<S> {
    /// Invokes the callback with the incoming message that triggered it.
    fn call(&self, message: IncomingMessage, state: Rc<RefCell<S>>) -> ActionStream;
}

impl<F, S> Callback<S> for F where F: Fn(IncomingMessage) -> ActionStream {
    fn call(&self, message: IncomingMessage, _state: Rc<RefCell<S>>) -> ActionStream {
        self(message)
    }
}

/// An action that a callback can take in response to an incoming message.
#[derive(Debug)]
pub enum Action {
    /// Sends a message to the chat service.
    SendMessage(OutgoingMessage),
}

/// An asynchronous stream of actions. This type is returned by callbacks.
pub type ActionStream = Box<Stream<Item = Action, Error = Error>>;

/// An asynchronous stream of callbacks.
pub struct Callbacks<S> {
    index: usize,
    inner: Rc<Vec<Rc<Box<Callback<S>>>>>,
}

impl<S> Callbacks<S> {
    /// Creates a new `Callbacks` from a vector of callbacks.
    pub fn new(callbacks: Vec<Rc<Box<Callback<S>>>>) -> Self {
        Callbacks {
            index: 0,
            inner: Rc::new(callbacks),
        }
    }
}

impl<S> Clone for Callbacks<S> {
    fn clone(&self) -> Self {
        Callbacks {
            index: 0,
            inner: self.inner.clone(),
        }
    }
}

impl<S> Stream for Callbacks<S> {
    type Item = Rc<Box<Callback<S>>>;
    type Error = Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        if self.index < self.inner.len() {
            let callback = self.inner[self.index].clone();
            self.index += 1;

            return Ok(Async::Ready(Some(callback)));
        } else {
            return Ok(Async::Ready(None));
        }
    }
}
