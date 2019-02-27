use std::sync::Arc;

use futures::executor::block_on;

use rustin::{chat_service::Shell, prelude::*, store::Memory};

fn echo<C, S>(chat_service: Arc<C>, message: &IncomingMessage, _store: S) -> Success
where
    C: ChatService,
    S: Store,
{
    let body = message.body();
    let reply = message.reply(body);

    chat_service.send_message(reply)
}

fn main() -> Result<(), Error> {
    let chat_service = Shell::default();
    let store = Memory::new();
    let echo_route = Route::new(r".*", true, "echo", echo)?;
    let robot = Robot::build(Config::default(), chat_service, store)
        .route(echo_route)
        .finish();

    if let Err(error) = block_on(robot.run()) {
        println!("ERROR: {}", error);
    }

    Ok(())
}
