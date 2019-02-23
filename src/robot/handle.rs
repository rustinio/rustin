//! The runtime API exposed to callbacks.

use std::fmt::Display;
use std::future::Future;
use std::pin::Pin;

use crate::callback::Action;
use crate::message::IncomingMessage;
use crate::room::Room;
use crate::store::{ScopedStore, Store};
use crate::user::User;

/// The API for callbacks to interface with the incoming message and data stores.
pub struct Handle<S>
where
    S: Store
{
    message: IncomingMessage,
    namespace: &'static str,
    scoped_store: ScopedStore<S>,
    store: S,
}

impl<S> Handle<S>
where
    S: Store,
{
    /// Creates a new `Handle`.
    pub fn new(message: IncomingMessage, namespace: &'static str, store: S) -> Self {
        Handle {
            message,
            namespace,
            scoped_store: ScopedStore::new(store.clone(), namespace),
            store,
        }
    }

    /// The body of the incoming message.
    pub fn message_body(&self) -> &str {
        self.message.body()
    }

    /// Replies to the incoming message.
    pub fn reply<B>(&self, body: B) -> Action
    where
        B: Into<String>,
    {
        self.message.reply(body)
    }

    /// Replies directly to the sender of the incoming message.
    pub fn reply_privately<B>(&self, body: B) -> Action
    where
        B: Into<String>,
    {
        self.message.reply_privately(body)
    }

    /// Replies to the incoming message, prefixing the message with the recipient's name if it will
    /// be delivered to a room.
    pub fn reply_with_mention<B>(&self, body: B) -> Action
    where
        B: Into<String>,
    {
        self.message.reply_with_mention(body)
    }

    /// The room the incoming message was sent from, if any.
    pub fn room(&self) -> Option<&Room> {
        self.message.room()
    }

    /// The user that sent the incoming message.
    pub fn user(&self) -> &User {
        self.message.user()
    }

    /// Gets the value of the given key, if any, from the robot's core data store.
    ///
    /// The key will be scoped to the callback's namespace.
    pub fn get<K>(
        &self,
        key: K,
    ) -> Pin<Box<dyn Future<Output = Result<Option<String>, <ScopedStore<S> as Store>::Error>>>>
    where
        K: AsRef<str> + Display,
    {
        self.scoped_store.get(key)
    }

    /// Sets the given key to the given value in the robot's core data store.
    ///
    /// The key will be scoped to the callback's namespace.
    pub fn set<K, V>(
        &self,
        key: K,
        value: V,
    ) -> Pin<Box<dyn Future<Output = Result<(), <ScopedStore<S> as Store>::Error>>>>
    where
        K: Display + Into<String>,
        V: Into<String>,
    {
        self.scoped_store.set(key, value)
    }
}
