use futures::future::ok;
use futures::executor::block_on;
use futures::stream::once;
use rustin::chat_service::Shell;
use rustin::message::IncomingMessage;
use rustin::storage::Memory;
use rustin::{ActionStream, Robot};

fn echo(message: IncomingMessage) -> ActionStream {
    Box::new(once(ok(message.reply(message.body()))))
}

fn main() {
    let robot = Robot::build(Shell, Memory::new()).callback(echo).finish();

    if let Err(error) = block_on(robot.run()) {
        println!("ERROR: {}", error);
    }
}
