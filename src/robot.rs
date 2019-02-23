use futures::stream::StreamExt;

use crate::{
    callback::{Action, Callback},
    chat_service::ChatService,
    error::Error,
    route::Route,
    store::Store,
};
use self::handle::Handle;

pub mod handle;

/// A builder for configuring a new `Robot`.
pub struct Builder<C, S>
where
    S: Store
{
    chat_service: C,
    routes: Vec<Route<S>>,
    store: S,
}

impl<C, S> Builder<C, S>
where
    S: Store
{

    /// Adds a route.
    pub fn route(mut self, route: Route<S>) -> Self
    where
    {
        self.routes.push(route);
        self
    }

    /// Creates a `Robot` from the builder.
    pub fn finish(self) -> Robot<C, S> {
        Robot {
            chat_service: self.chat_service,
            routes: self.routes,
            store: self.store,
        }
    }
}

/// The primary driver of a program using Rustin.
pub struct Robot<C, S>
where
    S: Store
{
    chat_service: C,
    routes: Vec<Route<S>>,
    store: S,
}

impl<C, S> Robot<C, S>
where
    C: ChatService,
    S: Store,
{
    /// Begins constructing a `Robot`.
    pub fn build(chat_service: C, store: S) -> Builder<C, S> {
        Builder {
            chat_service,
            routes: Vec::new(),
            store,
        }
    }

    /// Starts the robot, connecting to the chat service and listening for incoming messages.
    pub async fn run(self) -> Result<(), Error> {
        let mut incoming_messages = self.chat_service.incoming();

        while let Some(Ok(message)) = await!(StreamExt::next(&mut incoming_messages)) {
            for route in &self.routes {
                let handle = Handle::new(
                    message.clone(),
                    route.namespace(),
                    self.store.clone(),
                );

                if let Ok(mut actions) = await!(route.call(handle)) {
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
