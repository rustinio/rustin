use futures::Stream;

use error::Error;
use message::{IncomingMessage, OutgoingMessage};

/// A callback that receives incoming messages and reacts to them however it wishes.
pub trait Handler {
    fn call(&self, message: IncomingMessage) -> Box<Stream<Item = Action, Error = Error>>;
    fn box_clone(&self) -> Box<Handler>;
}

/// An action that a handler can take in response to an incoming message.
#[derive(Debug)]
pub enum Action {
    SendMessage(OutgoingMessage),
}

impl Clone for Box<Handler> {
    fn clone(&self) -> Box<Handler> {
        self.box_clone()
    }
}
