/// Configuration data for the robot.
#[derive(Clone, Debug)]
pub struct Config {
    /// The name of the robot as displayed to human users.
    pub name: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            name: "Rustin".to_string()
        }
    }
}
