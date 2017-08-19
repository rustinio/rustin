use std::rc::Rc;

use futures::{Future, Stream};

use callback::{Action, Callback, Callbacks};
use chat_service::ChatService;
use error::Error;
use storage::Store;

/// The primary driver of a program using Rustin.
pub struct Robot<C, S> {
    chat_service: Rc<C>,
    callbacks: Callbacks,
    storage: S,
}

impl<C, S> Robot<C, S> where C: ChatService + 'static, S: Store + 'static {
    /// Creates a new `Robot`.
    pub fn new(chat_service: C, storage: S, callbacks: Vec<Box<Callback>>) -> Self {
        Robot {
            chat_service: Rc::new(chat_service),
            callbacks: Callbacks::new(callbacks),
            storage,
        }
    }

    /// Starts the robot, connecting to the chat service and listening for incoming messages.
    pub fn run(self) -> Box<Future<Item = (), Error = Error>> {
        Box::new(self.chat_service.incoming().for_each(move |message| {
            let chat_service = self.chat_service.clone();

            self.callbacks.clone().for_each(move |callback| {
                let chat_service = chat_service.clone();
                let message = message.clone();

                callback.call(message).for_each(move |action| {
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
