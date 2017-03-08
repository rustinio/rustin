extern crate rustin;
extern crate futures;

use std::rc::Rc;

use futures::stream::once;
use futures::{Future, Stream};
use rustin::{
    Action,
    Config,
    Error,
    Handler,
    IncomingMessage,
    Robot,
    Shell,
};

struct Echo;

impl Handler for Echo {
    fn call(&self, message: IncomingMessage) -> Box<Stream<Item = Action, Error = Error>> {
        Box::new(once(Ok(message.reply(message.body().to_string()))))
    }
}

fn main() {
    let config = Config::default();
    let adapter = Shell::new();
    let robot = Robot::new(adapter, config, vec![Rc::new(Echo)]);

    if let Err(error) = robot.run().wait() {
        println!("ERROR: {}", error);
    }
}
