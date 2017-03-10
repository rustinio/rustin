//! Types for persisting data beyond the execution of the program.

use std::collections::HashMap;
use std::fmt::Display;

/// Persistent storage for the robot.
pub trait Store {
    /// Gets the value of the given key, if any.
    fn get<K>(&self, key: K) -> Option<&str> where K: AsRef<str> + Display;
    /// Sets the given key to the given value.
    fn set<K, V>(&mut self, key: K, value: V) where K: Display + Into<String>, V: Into<String>;
    /// Creates a new `Store` that prepends the given prefix to all key names.
    fn scoped<P>(&mut self, prefix: P) -> ScopedStore<Self> where P: Into<String>, Self: Sized;
    /// Returns the character used to seperate each nested scope created by `scoped`.
    fn seperator(&self) -> char {
        '.'
    }
}

/// A `Store` that lives in program memory, emptying when the program exits.
#[derive(Debug)]
pub struct Memory {
    data: HashMap<String, String>
}

impl Store for Memory {
    fn get<K>(&self, key: K) -> Option<&str> where K: AsRef<str> + Display {
        self.data.get(key.as_ref()).map(|value| value.as_str())
    }

    fn set<K, V>(&mut self, key: K, value: V) where K: Display + Into<String>, V: Into<String> {
        self.data.insert(key.into(), value.into());
    }

    fn scoped<P>(&mut self, prefix: P) -> ScopedStore<Memory> where P: Into<String> {
        ScopedStore {
            parent: self,
            prefix: prefix.into(),
        }
    }
}

/// A `Store` that persists data into a parent store, prepending a prefix to all key names.
#[derive(Debug)]
pub struct ScopedStore<'a, S> where S: Store + 'a {
    parent: &'a mut S,
    prefix: String,
}

impl<'a, S> Store for ScopedStore<'a, S> where S: Store {
    fn get<K>(&self, key: K) -> Option<&str> where K: AsRef<str> + Display {
        let key = format!("{}{}{}", self.prefix, self.parent.seperator(), key);

        self.parent.get(key)
    }

    fn set<K, V>(&mut self, key: K, value: V) where K: Display + Into<String>, V: Into<String> {
        let key = format!("{}{}{}", self.prefix, self.parent.seperator(), key);

        self.parent.set(key, value);
    }

    fn scoped<P>(&mut self, prefix: P) -> ScopedStore<Self> where P: Into<String>, Self: Sized {
        ScopedStore {
            parent: self,
            prefix: prefix.into(),
        }
    }

    fn seperator(&self) -> char {
        self.parent.seperator()
    }
}
