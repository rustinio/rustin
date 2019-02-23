//! The runtime API exposed to callbacks.

use std::fmt::Display;
use std::future::Future;
use std::pin::Pin;

use super::Robot;
use crate::callback::Action;
use crate::message::IncomingMessage;
use crate::room::Room;
use crate::store::{ScopedStore, Store};
use crate::user::User;

/// The API for callbacks to interface with the incoming message and data stores.
pub struct Handle<'a, C, S, K>
where
    S: Store,
{
    message: &'a IncomingMessage,
    store: ScopedStore<S>,
    robot: &'a Robot<C, S, K>,
}

impl<'a, C, S, K> Handle<'a, C, S, K>
where
    S: Store,
{
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
    pub fn get<Key>(
        &self,
        key: Key,
    ) -> Pin<Box<dyn Future<Output = Result<Option<String>, <ScopedStore<S> as Store>::Error>>>>
    where
        Key: AsRef<str> + Display,
    {
        self.store.get(key)
    }

    /// Sets the given key to the given value in the robot's core data store.
    ///
    /// The key will be scoped to the callback's namespace.
    pub fn set<Key, V>(
        &self,
        key: Key,
        value: V,
    ) -> Pin<Box<dyn Future<Output = Result<(), <ScopedStore<S> as Store>::Error>>>>
    where
        Key: Display + Into<String>,
        V: Into<String>,
    {
        self.store.set(key, value)
    }
}
