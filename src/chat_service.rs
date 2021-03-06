//! Types for connecting Rustin to a chat service.

use std::future::Future;
use std::pin::Pin;

use futures::Stream;

use crate::message::{IncomingMessage, OutgoingMessage};
use crate::result::{Error, Success};
use crate::room::Room;
use crate::user::User;

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
    fn incoming(&self, alias: Option<String>) -> Incoming;

    /// Returns a `User` representing the robot.
    ///
    /// May be absent if not applicable or if called before the robot has connected to the chat
    /// service and retrieved data about itself.
    fn user(&self) -> Pin<Box<dyn Future<Output = Result<User, Error>>>>;
}

/// A `ChatService` that supports joining and parting from multiple rooms.
pub trait MultiRoomChatService: ChatService {
    /// Makes Rustin join a chat room.
    fn join(&self, room: &Room) -> Success;

    /// Makes Rustin part from a chat room.
    fn part(&self, room: &Room) -> Success;
}

/// An asynchronous stream of incoming messages.
pub type Incoming = Pin<Box<dyn Stream<Item = Result<IncomingMessage, Error>>>>;
