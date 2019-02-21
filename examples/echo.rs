use futures::executor::block_on;
use futures::future::{ok, ready};
use futures::stream::once;
use rustin::chat_service::Shell;
use rustin::message::IncomingMessage;
use rustin::store::Memory;
use rustin::{ActionStream, FutureActionStream, Robot};

fn echo(message: &IncomingMessage) -> FutureActionStream {
    let body = message.body();
    let reply = message.reply(body);
    let future_reply = ready(reply);
    let stream = Box::new(once(future_reply)) as ActionStream;

    Box::new(ok(stream))
}

fn main() {
    let robot = Robot::build(Shell, Memory::new()).callback(echo).finish();

    if let Err(error) = block_on(robot.run()) {
        println!("ERROR: {}", error);
    }
}
