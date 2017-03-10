/// A chat room.
#[derive(Clone, Debug)]
pub struct Room {
    /// A unique identifier for the room.
    pub id: String,
    /// A human-readable name for the room.
    pub name: Option<String>,
}
