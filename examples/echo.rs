use std::sync::Arc;

use futures::executor::block_on;
use regex::Regex;

use rustin::{
    chat_service::{ChatService, Shell},
    message::IncomingMessage,
    store::Memory,
    CallbackFuture,
    Robot,
    Route,
    Store,
};

fn echo<C, S>(chat_service: Arc<C>, message: &IncomingMessage, _store: S) -> CallbackFuture
where
    C: ChatService,
    S: Store,
{
    let body = message.body();
    let reply = message.reply(body);

    chat_service.send_message(reply)
}

fn main() -> Result<(), regex::Error> {
    let chat_service = Shell::default();
    let store = Memory::new();
    let echo_route = Route::new(Regex::new(r".*")?, "echo", echo);
    let robot = Robot::build(chat_service, store).route(echo_route).finish();

    if let Err(error) = block_on(robot.run()) {
        println!("ERROR: {}", error);
    }

    Ok(())
}
