use room::Room;
use user::User;

/// The source of an incoming message.
#[derive(Clone, Debug)]
pub enum Source {
    /// A direct message from a user.
    User(User),
    /// A message from a user in room.
    UserInRoom(User, Room),
}

/// The target of an outgoing message.
#[derive(Clone, Debug)]
pub enum Target {
    /// A message to a room.
    Room(Room),
    /// A message to a user.
    User(User),
    /// A message to a specific user in a room.
    UserInRoom(User, Room),
}
