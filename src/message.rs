use handler::Action;
use correspondent::{Source, Target};

/// An incoming chat message.
#[derive(Clone, Debug)]
pub struct IncomingMessage {
    body: String,
    source: Source,
}

/// An outgoing chat message.
#[derive(Clone, Debug)]
pub struct OutgoingMessage {
    body: String,
    target: Target,
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
