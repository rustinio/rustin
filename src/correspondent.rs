use room::Room;
use user::User;

#[derive(Clone, Debug)]
pub enum Source {
    Room(Room),
    User(User),
    UserInRoom(User, Room),
}

#[derive(Clone, Debug)]
pub enum Target {
    Room(Room),
    User(User),
    UserInRoom(User, Room),
}
