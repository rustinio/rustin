extern crate futures;
extern crate rustin;

use futures::stream::once;
use futures::{Future, Stream};
use rustin::chat_service::Shell;
use rustin::message::IncomingMessage;
use rustin::storage::Memory;
use rustin::{Action, Callback, Error, Robot};

struct Echo;

impl Callback for Echo {
    fn call(&self, message: IncomingMessage) -> Box<Stream<Item = Action, Error = Error>> {
        Box::new(once(Ok(message.reply(message.body().to_string()))))
    }
}

fn main() {
    let robot = Robot::new(Shell, Memory::new(), vec![Box::new(Echo)]);

    if let Err(error) = robot.run().wait() {
        println!("ERROR: {}", error);
    }
}
