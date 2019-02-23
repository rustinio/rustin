//! Routes match incoming messages to callbacks.

use regex::Regex;

use crate::{
    callback::{Callback, FutureActionStream},
    robot::handle::Handle,
    store::Store,
};

/// A route is a regular expression to match against incoming messages and a callback to call when
/// a match is found.
pub struct Route<S> {
    callback: Box<dyn Callback<S>>,
    namespace: &'static str,
    pattern: Regex,
}

impl<S> Route<S> {
    /// Constructs a new `Route`.
    pub fn new<C>(pattern: Regex, namespace: &'static str, callback: C) -> Self
    where
        C: Callback<S> + 'static
    {
        Route {
            callback: Box::new(callback),
            namespace,
            pattern,
        }
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

impl<S> Callback<S> for Route<S>
where
    S: Store
{
    fn call(&self, handle: Handle<S>) -> FutureActionStream {
        self.callback.call(handle)
    }
}
