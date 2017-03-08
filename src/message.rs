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
    pub fn new(source: Source, body: String) -> Self {
        IncomingMessage {
            body: body,
            source: source,
        }
    }

    pub fn body(&self) -> &str {
        &self.body
    }

    pub fn reply<S>(&self, body: S) -> Action where S: Into<String> {
        Action::SendMessage(OutgoingMessage {
            body: body.into(),
            target: match self.source {
                Source::Room(ref room) => Target::Room(room.clone()),
                Source::User(ref user) => Target::User(user.clone()),
                Source::UserInRoom(_, ref room) => Target::Room(room.clone()),
            },
        })
    }
}

impl OutgoingMessage {
    pub fn new(target: Target, body: String) -> Self {
        OutgoingMessage {
            body: body,
            target: target,
        }
    }

    pub fn body(&self) -> &str {
        &self.body
    }
}
