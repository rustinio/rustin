//! Types for sending and receiving messages.

use std::fmt::{Display, Formatter, Result as FmtResult};

use crate::room::Room;
use crate::user::User;

/// An incoming chat message.
#[derive(Clone, Debug)]
pub struct IncomingMessage {
    body: String,
    source: Source,
}

impl IncomingMessage {
    /// Creates a new `IncomingMessage`.
    pub fn new(source: Source, body: String) -> Self {
        IncomingMessage {
            body,
            source: source,
        }
    }

    /// The body of the message.
    pub fn body(&self) -> &str {
        &self.body
    }

    /// Creates an `OutgoingMessage` targeting the source of the incoming message.
    pub fn reply<B>(&self, body: B) -> OutgoingMessage
    where
        B: Into<String>,
    {
        let target = match self.source {
            Source::User(ref user) => Target::User(user.clone()),
            Source::UserInRoom(_, ref room) => Target::Room(room.clone()),
        };

        OutgoingMessage::new(target, body)
    }

    /// Creates an `OutgoingMessage` directly targeting the source of the incoming message.
    pub fn reply_privately<B>(&self, body: B) -> OutgoingMessage
    where
        B: Into<String>,
    {
        let target = match self.source {
            Source::User(ref user) => Target::User(user.clone()),
            Source::UserInRoom(ref user, _) => Target::User(user.clone()),
        };

        OutgoingMessage::new(target, body)
    }

    /// Creates an `OutgoingMessage` targeting the source of the incoming message and, if the
    /// incoming message came from a room, prefixing the reply with the user's name.
    pub fn reply_with_mention<B>(&self, body: B) -> OutgoingMessage
    where
        B: Into<String>,
    {
        let target = match self.source {
            Source::User(ref user) => Target::User(user.clone()),
            Source::UserInRoom(ref user, ref room) => {
                Target::UserInRoom(user.clone(), room.clone())
            }
        };

        OutgoingMessage::new(target, body)
    }

    /// The room the message was sent from, if any.
    pub fn room(&self) -> Option<&Room> {
        self.source.room()
    }

    /// The user that sent the message.
    pub fn user(&self) -> &User {
        self.source.user()
    }
}

/// An outgoing chat message.
#[derive(Clone, Debug)]
pub struct OutgoingMessage {
    body: String,
    target: Target,
}

impl OutgoingMessage {
    /// Creates a new `OutgoingMessage`.
    pub fn new<B>(target: Target, body: B) -> Self
    where
        B: Into<String>,
    {
        OutgoingMessage {
            body: body.into(),
            target: target,
        }
    }

    /// The body of the message.
    pub fn body(&self) -> &str {
        &self.body
    }
}

impl Display for OutgoingMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.body)
    }
}

/// The source of an incoming message.
#[derive(Clone, Debug)]
pub enum Source {
    /// A direct message from a user.
    User(User),
    /// A message from a user in room.
    UserInRoom(User, Room),
}

impl Source {
    /// The user that sent the message.
    pub fn user(&self) -> &User {
        match *self {
            Source::User(ref user) => user,
            Source::UserInRoom(ref user, _) => user,
        }
    }

    /// The room the message was sent from, if any.
    pub fn room(&self) -> Option<&Room> {
        match *self {
            Source::User(_) => None,
            Source::UserInRoom(_, ref room) => Some(room),
        }
    }
}

/// The target of an outgoing message.
#[derive(Clone, Debug)]
pub enum Target {
    /// A message to a room.
    Room(Room),
    /// A message to a user.
    User(User),
    /// A message to a specific user in a room.
    UserInRoom(User, Room),
}
