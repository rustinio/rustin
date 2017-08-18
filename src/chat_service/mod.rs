//! Types for connecting Rustin to a chat service.

use futures::{Future, Stream};

use error::Error;
use message::{IncomingMessage, OutgoingMessage};
use room::Room;

pub use self::shell::Shell;

mod shell;

/// A type that handles the implementation details of the Robot API for a particular chat service.
pub trait ChatService: Clone {
    /// Makes Rustin join a chat room.
    fn join(&self, room: &Room) -> Box<Future<Item = (), Error = Error>>;

    /// Makes Rustin part from a chat room.
    fn part(&self, room: &Room) -> Box<Future<Item = (), Error = Error>>;

    /// Sends a message to a chat room or user.
    fn send_message(&self, message: OutgoingMessage) -> Box<Future<Item = (), Error = Error>>;

    /// Connects to the chat service and listens for incoming messages.
    fn incoming(&self) -> Box<Stream<Item = IncomingMessage, Error = Error>>;
}
