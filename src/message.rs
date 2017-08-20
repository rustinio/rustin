//! Types for sending and receiving messages.

use std::fmt::{Display, Formatter, Result as FmtResult};

use callback::Action;
use room::Room;
use user::User;

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
            body: body,
            source: source,
        }
    }

    /// The body of the message.
    pub fn body(&self) -> &str {
        &self.body
    }

    /// Creates an outgoing message action targeting the source of the incoming message.
    pub fn reply<S>(&self, body: S) -> Action where S: Into<String> {
        Action::SendMessage(OutgoingMessage {
            body: body.into(),
            target: match self.source {
                Source::User(ref user) => Target::User(user.clone()),
                Source::UserInRoom(_, ref room) => Target::Room(room.clone()),
            },
        })
    }

    /// Creates an outgoing message action directly targeting the source of the incoming message.
    pub fn reply_privately<S>(&self, body: S) -> Action where S: Into<String> {
        Action::SendMessage(OutgoingMessage {
            body: body.into(),
            target: match self.source {
                Source::User(ref user) => Target::User(user.clone()),
                Source::UserInRoom(ref user, _) => Target::User(user.clone()),
            },
        })
    }

    /// Creates an outgoing message action targeting the source of the incoming message and, if the
    /// incoming message came from a room, prefixing the reply with the user's name.
    pub fn reply_with_mention<S>(&self, body: S) -> Action where S: Into<String> {
        Action::SendMessage(OutgoingMessage {
            body: body.into(),
            target: match self.source {
                Source::User(ref user) => Target::User(user.clone()),
                Source::UserInRoom(ref user, ref room) => {
                    Target::UserInRoom(user.clone(), room.clone())
                }
            },
        })
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
    pub fn new(target: Target, body: String) -> Self {
        OutgoingMessage {
            body: body,
            target: target,
        }
    }

    /// The body of the message.
    pub fn body(&self) -> &str {
        &self.body
    }
}

impl Display for OutgoingMessage {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
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
