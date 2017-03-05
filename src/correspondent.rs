use room::Room;
use user::User;

/// The source of an incoming message.
#[derive(Clone, Debug)]
pub enum Source {
    Room(Room),
    User(User),
    UserInRoom(User, Room),
}

/// The target of an outgoing message.
#[derive(Clone, Debug)]
pub enum Target {
    Room(Room),
    User(User),
    UserInRoom(User, Room),
}
