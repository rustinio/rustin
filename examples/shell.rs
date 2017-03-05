extern crate rustin;
extern crate futures;

use std::sync::Arc;

use futures::sync::mpsc::channel;
use futures::{Future, Sink, Stream};
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
        let (tx, rx) = channel::<Action>(0);

        let body = message.body().to_string();
        let user = User::new::<&str, &str>("1", None);
        let target = Target::User(user);
        let outgoing = OutgoingMessage::new(target, body);

        tx.send(Action::SendMessage(outgoing));

        Box::new(rx.map_err(|_| Error))
    }
}

fn main() {
    let config = Config;
    let adapter = Shell::new();
    let robot = Robot::new(adapter, config, vec![Arc::new(Echo)]);

    if let Err(error) = robot.run().wait() {
        println!("ERROR: {}", error);
    }
}
