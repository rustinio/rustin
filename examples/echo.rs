use futures::executor::block_on;
use rustin::chat_service::Shell;
use rustin::message::IncomingMessage;
use rustin::store::Memory;
use rustin::{FutureActionStream, Robot};

fn echo(message: &IncomingMessage) -> FutureActionStream {
    let body = message.body();
    let reply = message.reply(body);

    reply.into()
}

fn main() {
    let chat_service = Shell::default();
    let store = Memory::new();
    let robot = Robot::build(chat_service, store).callback(echo).finish();

    if let Err(error) = block_on(robot.run()) {
        println!("ERROR: {}", error);
    }
}
