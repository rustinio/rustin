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

/// A route determines whether or not to invoke a callback by matching incoming messages against a
/// set of criteria.
pub struct Route<C, S>
where
    C: ChatService,
{
    callback: Box<dyn Callback<C, S>>,
    eavesdrop: bool,
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
    pub fn new<Cbk>(pattern: &str, eavesdrop: bool, namespace: &'static str, callback: Cbk) -> Result<Self, Error>
    where
        Cbk: Callback<C, S> + 'static,
    {
        let regex = Regex::new(pattern)?;

        Ok(Route {
            callback: Box::new(callback),
            eavesdrop,
            namespace,
            pattern: regex,
        })
    }

    /// Whether or not the robot should "eavesdrop" to look for this message.
    ///
    /// When `true`, the message does not need to be directed to the robot by name or alias.
    pub fn eavesdrop(&self) -> bool {
        self.eavesdrop
    }

    /// The namespace to use for any data persisted within the callback.
    pub fn namespace(&self) -> &'static str {
        self.namespace
    }

    /// The route's regular expression.
    pub fn pattern(&self) -> &Regex {
        &self.pattern
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
