use futures::stream::iter;
use futures::sync::mpsc::channel;
use futures::{Future, Sink, Stream};

use adapter::Adapter;
use config::Config;
use error::Error;
use handler::{Action, Handler};
use message::IncomingMessage;

/// The primary entry point for running Rustin.
pub struct Robot<A> where A: Adapter {
    adapter: A,
    config: Config,
    handlers: Vec<Box<Handler>>,
}

impl<A> Robot<A> where A: Adapter {
    pub fn new(adapter: A, config: Config, handlers: Vec<Box<Handler>>) -> Self {
        Robot {
            adapter: adapter,
            config: config,
            handlers: handlers,
        }
    }

    fn dispatch(&self, message: IncomingMessage) -> Box<Stream<Item = (), Error = Error>> {
        let handlers = iter(self.handlers.clone().into_iter().map(Ok));

        let results = handlers.and_then(move |handler| {
            let actions = handler.call(message);

            self.process_actions(actions)
        });

        Box::new(results)
    }

    fn process_actions(&self, actions: Box<Stream<Item = Action, Error = Error>>)
    -> Box<Future<Item = (), Error = Error>> {
        let processed_actions = actions.for_each(move |action| {
            match action {
                Action::SendMessage(outgoing) => {
                    self.adapter.send_message(outgoing)
                }
            }
        });

        Box::new(processed_actions)
    }

    pub fn run(self) -> Box<Future<Item = (), Error = Error>> {
        let result = self.adapter.incoming().for_each(move |message| {
            self.dispatch(message).for_each(|_| {
                Ok(())
            })
        });

        Box::new(result)
    }
}
