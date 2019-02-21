//! Types for persisting data.

use std::collections::HashMap;
use std::error::Error as StdError;
use std::fmt::Display;
use std::future::Future;
use std::marker::Unpin;

use futures::future::ok;

use crate::error::Error;

/// Persistent data storage for the robot.
pub trait Store {
    /// An error encountered when interacting with the underlying data store.
    type Error: StdError;

    /// Gets the value of the given key, if any.
    fn get<K>(&self, key: K) -> Box<dyn Future<Output = Result<Option<String>, Self::Error>> + Unpin>
    where
        K: AsRef<str> + Display;
    /// Sets the given key to the given value.
    fn set<K, V>(&mut self, key: K, value: V) -> Box<dyn Future<Output = Result<(), Self::Error>> + Unpin>
    where
        K: Display + Into<String>,
        V: Into<String>;
    /// Creates a new `Store` that prepends the given prefix to all key names.
    fn scoped<P>(&mut self, prefix: P) -> ScopedStore<'_, Self>
    where
        P: Into<String>,
        Self: Sized;
    /// Returns the character used to seperate each nested scope created by `scoped`.
    fn seperator(&self) -> char {
        '.'
    }
}

/// A `Store` that lives in program memory, emptying when the program exits.
#[derive(Debug)]
pub struct Memory {
    data: HashMap<String, String>,
}

impl Memory {
    /// Creates a new `Memory`.
    pub fn new() -> Self {
        Memory {
            data: HashMap::new(),
        }
    }
}

impl Store for Memory {
    type Error = Error;

    fn get<K>(&self, key: K) -> Box<dyn Future<Output = Result<Option<String>, Self::Error>> + Unpin>
    where
        K: AsRef<str> + Display,
    {
        Box::new(ok(self.data.get(key.as_ref()).map(|value| value.clone())))
    }

    fn set<K, V>(&mut self, key: K, value: V) -> Box<dyn Future<Output = Result<(), Self::Error>> + Unpin>
    where
        K: Display + Into<String>,
        V: Into<String>,
    {
        self.data.insert(key.into(), value.into());
        Box::new(ok(()))
    }

    fn scoped<P>(&mut self, prefix: P) -> ScopedStore<'_, Memory>
    where
        P: Into<String>,
    {
        ScopedStore {
            parent: self,
            prefix: prefix.into(),
        }
    }
}

/// A `Store` that persists data into a parent store, prepending a prefix to all key names.
#[derive(Debug)]
pub struct ScopedStore<'a, S>
where
    S: Store,
{
    parent: &'a mut S,
    prefix: String,
}

impl<'a, S> Store for ScopedStore<'a, S>
where
    S: Store,
{
    type Error = S::Error;

    fn get<K>(&self, key: K) -> Box<dyn Future<Output = Result<Option<String>, Self::Error>> + Unpin>
    where
        K: AsRef<str> + Display,
    {
        let key = format!("{}{}{}", self.prefix, self.parent.seperator(), key);

        self.parent.get(key)
    }

    fn set<K, V>(&mut self, key: K, value: V) -> Box<dyn Future<Output = Result<(), Self::Error>> + Unpin>
    where
        K: Display + Into<String>,
        V: Into<String>,
    {
        let key = format!("{}{}{}", self.prefix, self.parent.seperator(), key);

        self.parent.set(key, value)
    }

    fn scoped<P>(&mut self, prefix: P) -> ScopedStore<'_, Self>
    where
        P: Into<String>,
        Self: Sized,
    {
        ScopedStore {
            parent: self,
            prefix: prefix.into(),
        }
    }

    fn seperator(&self) -> char {
        self.parent.seperator()
    }
}
