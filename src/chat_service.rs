//! Types for connecting Rustin to a chat service.

use futures::{Future, Stream};

use crate::error::Error;
use crate::message::{IncomingMessage, OutgoingMessage};
use crate::room::Room;

pub use self::shell::Shell;

mod shell;

/// A type that handles the implementation details of the Robot API for a particular chat service.
pub trait ChatService {
    /// A type indicating a successful operation with the chat service that has no meaningful
    /// return value.
    type Success: Future<Output = Result<(), Error>>;

    /// An asynchronous stream of incoming messages.
    type Incoming: Stream<Item = Result<IncomingMessage, Error>>;

    /// Makes Rustin join a chat room.
    fn join(&self, room: &Room) -> Self::Success;

    /// Makes Rustin part from a chat room.
    fn part(&self, room: &Room) -> Self::Success;

    /// Sends a message to a chat room or user.
    fn send_message(&self, message: OutgoingMessage) -> Self::Success;

    /// Connects to the chat service and listens for incoming messages.
    fn incoming(&self) -> Self::Incoming;
}
