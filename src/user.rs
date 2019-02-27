//! The type for chat users.

/// A user of a chat service.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct User {
    /// A unique identifier for the user.
    ///
    /// This identifier is a username in most cases.
    id: String,
    /// A username for the user.
    ///
    /// This value may be the same as `id` for services that do not distinguish between ID and
    /// username.
    username: Option<String>,
    /// A human-readable name for the user.
    ///
    /// This value may be the same as `username` for services that do not distinguish between
    /// username and display name.
    display_name: Option<String>,
}

impl User {
    /// Creates a new `User`.
    pub fn new(id: &str, username: Option<&str>, display_name: Option<&str>) -> Self {
        User {
            id: id.to_owned(),
            username: username.map(|u| u.to_owned()),
            display_name: display_name.map(|d| d.to_owned()),
        }
    }

    /// A unique identifier for the user.
    ///
    /// This identifier is a username in most cases.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// A username for the user.
    ///
    /// This value may be the same as `id` for services that do not distinguish between ID and
    /// username.
    pub fn username(&self) -> Option<&str> {
        self.username.as_ref().map(String::as_str).or(Some(self.id()))
    }

    /// A human-readable name for the user.
    ///
    /// This value may be the same as `username` for services that do not distinguish between
    /// username and display name.
    pub fn display_name(&self) -> Option<&str> {
        self.display_name.as_ref().map(String::as_str).or(self.username())
    }
}
