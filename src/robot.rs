use std::rc::Rc;

use futures::stream::iter;
use futures::{Future, Stream};

use adapter::Adapter;
use error::Error;
use handler::{Action, Handler};
use message::IncomingMessage;

/// Configuration data for the robot.
#[derive(Clone, Debug)]
pub struct Config {
    /// The name of the robot as displayed to human users.
    pub name: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            name: "Rustin".to_string()
        }
    }
}

/// The primary entry point for running Rustin.
pub fn run<A>(adapter: A, handlers: Vec<Rc<Handler>>)
-> Box<Future<Item = (), Error = Error>> where A: Adapter {
    let result = adapter.incoming().for_each(move |message| {
        let handlers = Box::new(iter(handlers.clone().into_iter().map(Ok)));

        dispatch(adapter.clone(), handlers, message).for_each(|_| {
            Ok(())
        })
    });

    Box::new(result)
}

fn dispatch<A, H>(adapter: A, handlers: Box<H>, message: IncomingMessage)
-> Box<Stream<Item = (), Error = Error>>
where A: Adapter, H: Stream<Item = Rc<Handler>, Error = Error> + 'static {
    let results = handlers.and_then(move |handler| {
        let actions = handler.call(message.clone());

        process_actions(adapter.clone(), actions)
    });

    Box::new(results)
}

fn process_actions<A>(adapter: A, actions: Box<Stream<Item = Action, Error = Error>>)
-> Box<Future<Item = (), Error = Error>> where A: Adapter {
    let processed_actions = actions.for_each(move |action| {
        match action {
            Action::SendMessage(outgoing) => {
                adapter.send_message(outgoing)
            }
        }
    });

    Box::new(processed_actions)
}
