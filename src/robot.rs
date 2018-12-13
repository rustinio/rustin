use std::pin::Unpin;

use futures::stream::{iter, StreamExt};

use crate::callback::{Action, Callback};
use crate::chat_service::ChatService;
use crate::error::Error;

/// A builder for configuring a new `Robot`.
pub struct Builder<C, S> {
    callbacks: Vec<Box<Callback<S>>>,
    chat_service: C,
    state: S,
}

impl<C, S> Builder<C, S> {
    /// Adds a callback.
    pub fn callback<T>(mut self, callback: T) -> Self
    where
        T: Callback<S> + 'static,
    {
        self.callbacks.push(Box::new(callback));

        self
    }

    /// Creates a `Robot` from the builder.
    pub fn finish(self) -> Robot<C, S> {
        Robot {
            callbacks: self.callbacks,
            chat_service: self.chat_service,
            state: self.state,
        }
    }
}

/// The primary driver of a program using Rustin.
pub struct Robot<C, S> {
    callbacks: Vec<Box<Callback<S>>>,
    chat_service: C,
    state: S,
}

impl<C, S> Robot<C, S>
where
    C: ChatService + Unpin + 'static,
    <C as ChatService>::Incoming: Unpin,
    S: 'static,
{
    /// Begins constructing a `Robot`.
    pub fn build(chat_service: C, state: S) -> Builder<C, S> {
        Builder {
            callbacks: Vec::new(),
            chat_service,
            state,
        }
    }

    /// Starts the robot, connecting to the chat service and listening for incoming messages.
    pub async fn run(mut self) -> Result<(), Error> {
        let mut incoming_messages = self.chat_service.incoming();
        let mut callbacks = iter(self.callbacks.into_iter());

        while let Some(Ok(message)) = await!(StreamExt::next(&mut incoming_messages)) {
            while let Some(callback) = await!(callbacks.next()) {
                let mut actions = callback.call(&message, &mut self.state);

                while let Some(Ok(action)) = await!(StreamExt::next(&mut actions)) {
                    match action {
                        Action::SendMessage(outgoing) => {
                            await!(self.chat_service.send_message(outgoing))?;
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
