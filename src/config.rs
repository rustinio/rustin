//! Configuration data for Rustin.

/// Configuration data for a `Robot`.
#[derive(Clone, Debug)]
pub struct Config {
    /// The name of the robot as displayed to human users.
    pub name: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            name: "Rustin".to_string(),
        }
    }
}
