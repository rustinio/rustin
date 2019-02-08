//! Types for connecting Rustin to a chat service.

use std::marker::Unpin;

use futures::{Future, Stream};

use crate::error::Error;
use crate::message::{IncomingMessage, OutgoingMessage};
use crate::room::Room;

pub use self::shell::Shell;

mod shell;

/// A type that handles the implementation details of the Robot API for a particular chat service.
///
/// This is the most basic form of chat service that supports sending and receiving messages in a
/// single room only.
pub trait ChatService {
    /// Sends a message to a chat room or user.
    fn send_message(&self, message: OutgoingMessage) -> Success;

    /// Connects to the chat service and listens for incoming messages.
    fn incoming(&self) -> Incoming;
}

/// A `ChatService` that supports joining and parting from multiple rooms.
pub trait MultiRoomChatService: ChatService {
    /// Makes Rustin join a chat room.
    fn join(&self, room: &Room) -> Success;

    /// Makes Rustin part from a chat room.
    fn part(&self, room: &Room) -> Success;
}

/// A type indicating a successful operation with the chat service that has no meaningful return value.
pub type Success = Box<dyn Future<Output = Result<(), Error>> + Unpin>;

/// An asynchronous stream of incoming messages.
pub type Incoming = Box<dyn Stream<Item = Result<IncomingMessage, Error>> + Unpin>;
