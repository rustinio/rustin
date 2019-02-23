use futures::executor::block_on;
use regex::Regex;

use rustin::{
    chat_service::Shell,
    store::Memory,
    FutureActionStream,
    Handle,
    Robot,
    Route,
    Store,
};

fn echo<S>(handle: Handle<S>) -> FutureActionStream
where
    S: Store
{
    let body = handle.message_body();
    let reply = handle.reply(body);

    reply.into()
}

fn main() -> Result<(), regex::Error> {
    let chat_service = Shell::default();
    let store = Memory::new();
    let echo_route = Route::new(Regex::new(r".*")?, "echo", echo);
    let robot = Robot::build(chat_service, store)
        .route(echo_route)
        .finish();

    if let Err(error) = block_on(robot.run()) {
        println!("ERROR: {}", error);
    }

    Ok(())
}
