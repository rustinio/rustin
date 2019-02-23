/// Routes match incoming messages to callbacks.

use regex::Regex;

use crate::{
    callback::{Callback, FutureActionStream},
    robot::handle::Handle,
    store::Store,
};

pub struct Route<S> {
    callback: Box<dyn Callback<S>>,
    namespace: &'static str,
    pattern: Regex,
}

impl<S> Route<S> {
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

    pub fn pattern(&self) -> &Regex {
        &self.pattern
    }

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
