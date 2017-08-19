//! Crate `rustin` is an extensible chat bot framework.

#![deny(missing_docs)]

extern crate futures;

mod callback;
pub mod chat_service;
mod config;
mod error;
pub mod message;
mod robot;
mod room;
pub mod storage;
mod user;

pub use callback::{Action, Callback};
pub use config::Config;
pub use error::Error;
pub use robot::{Builder, Robot};
pub use room::Room;
pub use user::User;
