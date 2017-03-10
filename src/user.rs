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
    pub fn new<I, N>(id: I, name: Option<N>) -> Self where I: Into<String>, N: Into<String> {
        User {
            id: id.into(),
            name: name.map(|n| n.into()),
        }
    }
}
