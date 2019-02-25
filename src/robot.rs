use std::sync::Arc;

use futures::stream::StreamExt;

use crate::{
    callback::Callback,
    chat_service::ChatService,
    error::Error,
    route::Route,
    store::Store,
};
/// A builder for configuring a new `Robot`.
pub struct Builder<C, S>
where
    C: ChatService,
    S: Store,
{
    chat_service: C,
    routes: Vec<Route<C, S>>,
    store: S,
}

impl<C, S> Builder<C, S>
where
    C: ChatService,
    S: Store,
{
    /// Adds a route.
    pub fn route(mut self, route: Route<C, S>) -> Self
where {
        self.routes.push(route);
        self
    }

    /// Creates a `Robot` from the builder.
    pub fn finish(self) -> Robot<C, S> {
        Robot {
            chat_service: Arc::new(self.chat_service),
            routes: self.routes,
            store: self.store,
        }
    }
}

/// The primary driver of a program using Rustin.
pub struct Robot<C, S>
where
    C: ChatService,
    S: Store,
{
    chat_service: Arc<C>,
    routes: Vec<Route<C, S>>,
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
                await!(route.call(self.chat_service.clone(), &message, self.store.clone()))?
            }
        }

        Ok(())
    }
}
