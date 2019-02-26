//! Routes match incoming messages to callbacks.

use std::sync::Arc;

use regex::Regex;

use crate::{
    callback::{Callback, CallbackFuture},
    chat_service::ChatService,
    message::IncomingMessage,
    result::Error,
    store::Store,
};

/// A route is a regular expression to match against incoming messages and a callback to call when
/// a match is found.
pub struct Route<C, S>
where
    C: ChatService,
{
    callback: Box<dyn Callback<C, S>>,
    namespace: &'static str,
    pattern: Regex,
}

impl<C, S> Route<C, S>
where
    C: ChatService,
{
    /// Constructs a new `Route`.
    ///
    /// # Errors
    ///
    /// Returns an error if the provided pattern can't be turned into a regular expression.
    pub fn new<Cbk>(pattern: &str, namespace: &'static str, callback: Cbk) -> Result<Self, Error>
    where
        Cbk: Callback<C, S> + 'static,
    {
        let regex = Regex::new(pattern)?;

        Ok(Route {
            callback: Box::new(callback),
            namespace,
            pattern: regex,
        })
    }

    /// The route's regular expression.
    pub fn pattern(&self) -> &Regex {
        &self.pattern
    }

    /// The namespace to use for any data persisted within the callback.
    pub fn namespace(&self) -> &'static str {
        self.namespace
    }
}

impl<C, S> Callback<C, S> for Route<C, S>
where
    C: ChatService,
    S: Store,
{
    fn call(&self, chat: Arc<C>, message: &IncomingMessage, store: S) -> CallbackFuture {
        self.callback.call(chat, message, store)
    }
}
