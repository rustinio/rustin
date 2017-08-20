extern crate futures;
extern crate rustin;

use futures::stream::once;
use futures::{Future, Stream};
use rustin::chat_service::Shell;
use rustin::message::IncomingMessage;
use rustin::storage::Memory;
use rustin::{Action, Error, Robot};

fn echo(message: IncomingMessage) -> Box<Stream<Item = Action, Error = Error>> {
    Box::new(once(Ok(message.reply(message.body()))))
}

fn main() {
    let robot = Robot::build(Shell, Memory::new())
        .callback(echo)
        .finish();

    if let Err(error) = robot.run().wait() {
        println!("ERROR: {}", error);
    }
}
