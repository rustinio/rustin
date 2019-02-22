use futures::stream::StreamExt;

use crate::callback::{Action, Callback};
use crate::chat_service::ChatService;
use crate::error::Error;
use crate::store::Store;

/// A builder for configuring a new `Robot`.
pub struct Builder<C, S, K> {
    callbacks: Vec<Box<dyn Callback<S, K>>>,
    chat_service: C,
    store: S,
}

impl<C, S, K> Builder<C, S, K> {
    /// Adds a callback.
    pub fn callback<T>(mut self, callback: T) -> Self
    where
        T: Callback<S, K> + 'static,
        S: Store,
    {
        self.callbacks.push(Box::new(callback));

        self
    }

    /// Creates a `Robot` from the builder.
    pub fn finish(self) -> Robot<C, S, K> {
        Robot {
            callbacks: self.callbacks,
            chat_service: self.chat_service,
            store: self.store,
        }
    }
}

/// The primary driver of a program using Rustin.
pub struct Robot<C, S, K> {
    callbacks: Vec<Box<dyn Callback<S, K>>>,
    chat_service: C,
    store: S,
}

impl<C, S, K> Robot<C, S, K>
where
    C: ChatService,
    S: Store,
{
    /// Begins constructing a `Robot`.
    pub fn build(chat_service: C, store: S) -> Builder<C, S, K> {
        Builder {
            callbacks: Vec::new(),
            chat_service,
            store,
        }
    }

    /// Starts the robot, connecting to the chat service and listening for incoming messages.
    pub async fn run(self) -> Result<(), Error> {
        let mut incoming_messages = self.chat_service.incoming();

        while let Some(Ok(message)) = await!(StreamExt::next(&mut incoming_messages)) {
            for callback in &self.callbacks {
                if let Ok(mut actions) = await!(callback.call(&message, &self.store)) {
                    while let Some(action) = await!(StreamExt::next(&mut actions)) {
                        match action {
                            Action::SendMessage(outgoing) => {
                                await!(self.chat_service.send_message(outgoing))?;
                            }
                        }
                    }
                }
                // TODO: Handle errors from callbacks.
            }
        }

        Ok(())
    }
}
