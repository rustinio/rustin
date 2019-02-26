//! Types for persisting data.

use std::collections::HashMap;
use std::error::Error as StdError;
use std::fmt::Display;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, RwLock};

use futures::future::{err, ok};

use crate::result::Error;

/// Persistent data storage for the robot.
pub trait Store: Clone + Send + Sync + 'static {
    /// An error encountered when interacting with the underlying data store.
    type Error: StdError + Send + Sync + 'static;

    /// Gets the value of the given key, if any.
    fn get<K>(&self, key: K) -> Pin<Box<dyn Future<Output = Result<Option<String>, Self::Error>>>>
    where
        K: AsRef<str> + Display;
    /// Sets the given key to the given value.
    fn set<K, V>(&self, key: K, value: V) -> Pin<Box<dyn Future<Output = Result<(), Self::Error>>>>
    where
        K: Display + Into<String>,
        V: Into<String>;
    /// Creates a new `Store` that prepends the given prefix to all key names.
    fn scoped<P>(&self, prefix: P) -> ScopedStore<Self>
    where
        P: Into<String>,
        Self: Sized;
    /// Returns the character used to seperate each nested scope created by `scoped`.
    fn seperator(&self) -> char {
        '.'
    }
}

/// A `Store` that lives in program memory, emptying when the program exits.
#[derive(Clone, Debug)]
pub struct Memory {
    data: Arc<RwLock<HashMap<String, String>>>,
}

impl Memory {
    /// Creates a new `Memory`.
    pub fn new() -> Self {
        Memory {
            data: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl Store for Memory {
    type Error = Error;

    fn get<K>(&self, key: K) -> Pin<Box<dyn Future<Output = Result<Option<String>, Self::Error>>>>
    where
        K: AsRef<str> + Display,
    {
        let future = match self.data.read() {
            Ok(data) => ok(data.get(key.as_ref()).map(|value| value.clone())),
            Err(_) => err(Error),
        };

        Box::pin(future)
    }

    fn set<K, V>(&self, key: K, value: V) -> Pin<Box<dyn Future<Output = Result<(), Self::Error>>>>
    where
        K: Display + Into<String>,
        V: Into<String>,
    {
        let future = match self.data.write() {
            Ok(mut data) => {
                data.insert(key.into(), value.into());

                ok(())
            }
            Err(_) => err(Error),
        };

        Box::pin(future)
    }

    fn scoped<P>(&self, prefix: P) -> ScopedStore<Memory>
    where
        P: Into<String>,
    {
        ScopedStore {
            parent: self.clone(),
            prefix: prefix.into(),
        }
    }
}

/// A `Store` that persists data into a parent store, prepending a prefix to all key names.
#[derive(Clone, Debug)]
pub struct ScopedStore<S>
where
    S: Store,
{
    parent: S,
    prefix: String,
}

impl<S> ScopedStore<S>
where
    S: Store,
{
    /// Creates a new `ScopedStore`.
    pub fn new<P>(store: S, prefix: P) -> Self
    where
        P: Into<String>,
    {
        ScopedStore {
            parent: store,
            prefix: prefix.into(),
        }
    }
}

impl<S> Store for ScopedStore<S>
where
    S: Store,
{
    type Error = S::Error;

    fn get<K>(&self, key: K) -> Pin<Box<dyn Future<Output = Result<Option<String>, Self::Error>>>>
    where
        K: AsRef<str> + Display,
    {
        let key = format!("{}{}{}", self.prefix, self.parent.seperator(), key);

        self.parent.get(key)
    }

    fn set<K, V>(&self, key: K, value: V) -> Pin<Box<dyn Future<Output = Result<(), Self::Error>>>>
    where
        K: Display + Into<String>,
        V: Into<String>,
    {
        let key = format!("{}{}{}", self.prefix, self.parent.seperator(), key);

        self.parent.set(key, value)
    }

    fn scoped<P>(&self, prefix: P) -> ScopedStore<Self>
    where
        P: Into<String>,
        Self: Sized,
    {
        ScopedStore {
            parent: self.clone(),
            prefix: prefix.into(),
        }
    }

    fn seperator(&self) -> char {
        self.parent.seperator()
    }
}
