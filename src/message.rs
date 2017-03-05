use correspondent::{Source, Target};

#[derive(Clone, Debug)]
pub struct IncomingMessage {
    body: String,
    source: Source,
}

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
