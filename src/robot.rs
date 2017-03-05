use std::sync::Arc;

use futures::stream::iter;
use futures::{Future, Stream};

use adapter::Adapter;
use config::Config;
use error::Error;
use handler::{Action, Handler};
use message::IncomingMessage;

/// The primary entry point for running Rustin.
pub struct Robot<A> where A: Adapter {
    adapter: Arc<A>,
    config: Config,
    handlers: Vec<Arc<Handler>>,
}

impl<A> Robot<A> where A: Adapter {
    pub fn new(adapter: A, config: Config, handlers: Vec<Arc<Handler>>) -> Self {
        Robot {
            adapter: Arc::new(adapter),
            config: config,
            handlers: handlers,
        }
    }

    pub fn run(self) -> Box<Future<Item = (), Error = Error>> {
        let result = self.adapter.clone().incoming().for_each(move |message| {
            let handlers = Box::new(iter(self.handlers.clone().into_iter().map(Ok)));

            dispatch(self.adapter.clone(), handlers, message).for_each(|_| {
                Ok(())
            })
        });

        Box::new(result)
    }
}

fn dispatch<A, H>(adapter: Arc<A>, handlers: Box<H>, message: IncomingMessage)
-> Box<Stream<Item = (), Error = Error>>
where A: Adapter, H: Stream<Item = Arc<Handler>, Error = Error> + 'static {
    let results = handlers.and_then(move |handler| {
        let actions = handler.call(message.clone());

        process_actions(adapter.clone(), actions)
    });

    Box::new(results)
}

fn process_actions<A>(adapter: Arc<A>, actions: Box<Stream<Item = Action, Error = Error>>)
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
