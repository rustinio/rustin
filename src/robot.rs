use std::cell::RefCell;
use std::rc::Rc;

use futures::{Future, Stream};

use callback::{Action, Callback, Callbacks};
use chat_service::ChatService;
use error::Error;
/// A builder for configuring a new `Robot`.
pub struct Builder<C, S> {
    callbacks: Vec<Rc<Box<Callback<S>>>>,
    chat_service: C,
    state: S,
}

impl<C, S> Builder<C, S> {
    /// Adds a callback.
    pub fn callback<T>(mut self, callback: T) -> Self where T: Callback<S> + 'static {
        self.callbacks.push(Rc::new(Box::new(callback)));

        self
    }

    /// Creates a `Robot` from the builder.
    pub fn finish(self) -> Robot<C, S> {
        Robot {
            callbacks: Callbacks::new(self.callbacks),
            chat_service: Rc::new(self.chat_service),
            state: Rc::new(RefCell::new(self.state)),
        }
    }
}

/// The primary driver of a program using Rustin.
pub struct Robot<C, S> {
    callbacks: Callbacks<S>,
    chat_service: Rc<C>,
    state: Rc<RefCell<S>>,
}

impl<C, S> Robot<C, S> where C: ChatService + 'static, S: 'static {
    /// Begins constructing a `Robot`.
    pub fn build(chat_service: C, state: S) -> Builder<C, S> {
        Builder {
            callbacks: Vec::new(),
            chat_service,
            state,
        }
    }

    /// Starts the robot, connecting to the chat service and listening for incoming messages.
    pub fn run(self) -> Box<Future<Item = (), Error = Error>> {
        Box::new(self.chat_service.incoming().for_each(move |message| {
            let chat_service = self.chat_service.clone();
            let state = self.state.clone();

            self.callbacks.clone().for_each(move |callback| {
                let chat_service = chat_service.clone();
                let message = message.clone();
                let state = state.clone();

                callback.call(message, state).for_each(move |action| {
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
