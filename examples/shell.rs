extern crate rustin;
extern crate futures;

use futures::future::ok;
use futures::sync::mpsc::channel;
use futures::{BoxFuture, Future, Sink, Stream};
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

        let outgoing = OutgoingMessage {
            body: message.body().to_string(),
            target: Target::User(User::new("1", None)),
        };

        tx.send(Action::SendMessage(outgoing));

        Box::new(rx.then(|result| result.expect("receivers cannot produce errors")))
    }
}

fn main() {
    let config = Config;
    let adapter = Shell::new();
    let robot = Robot::new(adapter, config, vec![Echo]);

    if let Err(error) = robot.run().wait() {
        println!("ERROR: {}", error);
    }
}
