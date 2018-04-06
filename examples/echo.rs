extern crate futures;
extern crate rustin;

use futures::executor::block_on;
use futures::stable::StableFuture;
use futures::stream::once;
use rustin::chat_service::Shell;
use rustin::message::IncomingMessage;
use rustin::storage::Memory;
use rustin::{ActionStream, Robot};

fn echo(message: IncomingMessage) -> ActionStream {
    Box::new(once(Ok(message.reply(message.body()))))
}

fn main() {
    let robot = Robot::build(Shell, Memory::new())
        .callback(echo)
        .finish();

    if let Err(error) = block_on(robot.run().pin_local()) {
        println!("ERROR: {}", error);
    }
}
