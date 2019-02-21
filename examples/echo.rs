use futures::executor::block_on;
use rustin::chat_service::Shell;
use rustin::message::IncomingMessage;
use rustin::store::Memory;
use rustin::{FutureActionStream, Robot};

fn echo(message: &IncomingMessage) -> FutureActionStream {
    let body = message.body();
    let reply = message.reply(body);

    reply.into_future_action_stream()
}

fn main() {
    let robot = Robot::build(Shell, Memory::new()).callback(echo).finish();

    if let Err(error) = block_on(robot.run()) {
        println!("ERROR: {}", error);
    }
}
