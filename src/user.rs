//! The type for chat users.

/// A user of a chat service.
#[derive(Clone, Debug)]
pub struct User {
    /// A unique identifier for the user.
    id: String,
    /// A human-readable name for the user.
    name: Option<String>,
}

impl User {
    /// Creates a new `User`.
    pub fn new<I, N>(id: I, name: Option<N>) -> Self
    where
        I: Into<String>,
        N: Into<String>,
    {
        User {
            id: id.into(),
            name: name.map(|n| n.into()),
        }
    }

    /// A unique identifier for the user.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// A human-readable name for the user.
    pub fn name(&self) -> Option<&str> {
        self.name.as_ref().map(String::as_str)
    }
}
