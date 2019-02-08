//! The type for chat rooms.

/// A chat room.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Room {
    /// A unique identifier for the room.
    id: String,
    /// A human-readable name for the room.
    name: Option<String>,
}

impl Room {
    /// Creates a new `Room`.
    pub fn new<I, N>(id: I, name: Option<N>) -> Self
    where
        I: Into<String>,
        N: Into<String>,
    {
        Room {
            id: id.into(),
            name: name.map(|n| n.into()),
        }
    }

    /// A unique identifier for the room.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// A human-readable name for the room.
    pub fn name(&self) -> Option<&str> {
        self.name.as_ref().map(String::as_str)
    }
}
