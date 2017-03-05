use futures::{Future, Stream};

use error::Error;
use message::{IncomingMessage, OutgoingMessage};
use room::Room;

mod shell;

pub use self::shell::Shell;

pub trait Adapter: 'static {
    fn join(&self, room: &Room) -> Box<Future<Item = (), Error = Error>>;
    fn part(&self, room: &Room) -> Box<Future<Item = (), Error = Error>>;
    fn send_message(&self, message: OutgoingMessage) -> Box<Future<Item = (), Error = Error>>;
    fn send_messages(&self, messages: &[OutgoingMessage]) -> Box<Future<Item = (), Error = Error>>;
    fn incoming(&self) -> Box<Stream<Item = IncomingMessage, Error = Error>>;
}
