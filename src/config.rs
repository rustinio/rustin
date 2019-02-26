//! Configuration data for Rustin.

/// Configuration data for a `Robot`.
#[derive(Clone, Debug)]
pub struct Config {
    /// An alias for the robot.
    ///
    /// The robot normally determines that a message is directed to it by looking for its username
    /// or display name at the beginning of a message. If an alias is set, its presence at the
    /// beginning of the message will also be treated as a directed message.
    ///
    /// This is commonly set to a single character like `/` or `!` as a short way of sending
    /// commands to the robot in chat.
    pub alias: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            alias: None,
        }
    }
}
