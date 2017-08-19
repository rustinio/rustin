extern crate rustin;
extern crate futures;

use futures::stream::once;
use futures::{Future, Stream};
use rustin::{
    Action,
    Callback,
    Config,
    Error,
    IncomingMessage,
    Shell,
    run,
};

struct Echo;

impl Callback for Echo {
    fn call(&self, message: IncomingMessage) -> impl Stream<Item = Action, Error = Error> {
        once(Ok(message.reply(message.body().to_string())))
    }
}

fn main() {
    let config = Config::default();
    let adapter = Shell::new(config);

    if let Err(error) = run(adapter, vec![Echo]).wait() {
        println!("ERROR: {}", error);
    }
}
