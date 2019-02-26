//! Types for extending Rustin's behavior.

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use crate::{message::IncomingMessage, result::Error, store::Store};

/// A callback that receives incoming messages and reacts to them however it wishes.
pub trait Callback<C, S> {
    /// Invokes the callback with the incoming message that triggered it.
    fn call(&self, chat: Arc<C>, message: &IncomingMessage, store: S) -> CallbackFuture;
}

impl<F, C, S> Callback<C, S> for F
where
    F: Fn(Arc<C>, &IncomingMessage, S) -> CallbackFuture,
    S: Store,
{
    fn call(&self, chat: Arc<C>, message: &IncomingMessage, store: S) -> CallbackFuture {
        self(chat, message, store)
    }
}

/// The type returned by callbacks.
pub type CallbackFuture = Pin<Box<dyn Future<Output = Result<(), Error>>>>;
