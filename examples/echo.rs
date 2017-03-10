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
    Shell,
    run,
};

struct Echo;

impl Handler for Echo {
    fn call(&self, message: IncomingMessage) -> Box<Stream<Item = Action, Error = Error>> {
        Box::new(once(Ok(message.reply(message.body().to_string()))))
    }
}

fn main() {
    let config = Config::default();
    let adapter = Shell::new(config);

    if let Err(error) = run(adapter, vec![Rc::new(Echo)]).wait() {
        println!("ERROR: {}", error);
    }
}
