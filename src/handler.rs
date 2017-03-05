use futures::Stream;

use error::Error;
use message::{IncomingMessage, OutgoingMessage};

pub trait Handler {
    fn call(&self, message: IncomingMessage) -> Box<Stream<Item = Action, Error = Error>>;
    fn box_clone(&self) -> Box<Handler>;
}

#[derive(Debug)]
pub enum Action {
    SendMessage(OutgoingMessage),
}

impl Clone for Box<Handler> {
    fn clone(&self) -> Box<Handler> {
        self.box_clone()
    }
}
