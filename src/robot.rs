use std::rc::Rc;

use futures::{Future, Stream};

use chat_service::ChatService;
use error::Error;
use handler::{Action, Handler, HandlerStream};
use storage::Store;

/// The primary driver of a program using Rustin.
#[derive(Debug)]
pub struct Robot<C, S> {
    chat_service: Rc<C>,
    handlers: HandlerStream,
    storage: S,
}

impl<C, S> Robot<C, S> where C: ChatService + 'static, S: Store + 'static {
    /// Creates a new `Robot`.
    pub fn new(chat_service: C, storage: S, handlers: Vec<Box<Handler>>) -> Self {
        Robot {
            chat_service: Rc::new(chat_service),
            handlers: HandlerStream::new(handlers),
            storage,
        }
    }

    /// Starts the robot, connecting to the chat service and listening for incoming messages.
    pub fn run(self) -> Box<Future<Item = (), Error = Error>> {
        Box::new(self.chat_service.incoming().for_each(move |message| {
            let chat_service = self.chat_service.clone();

            self.handlers.clone().for_each(move |handler| {
                let chat_service = chat_service.clone();
                let message = message.clone();

                handler.call(message).for_each(move |action| {
                    match action {
                        Action::SendMessage(outgoing) => {
                            chat_service.send_message(outgoing)
                        }
                    }
                })
            })
        }))
    }
}
