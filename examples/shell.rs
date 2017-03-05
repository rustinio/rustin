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
    OutgoingMessage,
    Robot,
    Shell,
    Target,
    User,
};

struct Echo;

impl Handler for Echo {
    fn call(&self, message: IncomingMessage) -> Box<Stream<Item = Action, Error = Error>> {
        let body = message.body().to_string();
        let user = User::new::<&str, &str>("1", None);
        let target = Target::User(user);
        let outgoing = OutgoingMessage::new(target, body);

        Box::new(once(Ok(Action::SendMessage(outgoing))))
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
