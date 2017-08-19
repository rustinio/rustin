//! Crate `rustin` is an extensible chat bot framework.

#![deny(missing_debug_implementations)]
#![deny(missing_docs)]
#![deny(warnings)]

extern crate futures;

pub mod callback;
pub mod chat_service;
mod config;
mod error;
pub mod message;
mod robot;
mod room;
pub mod storage;
mod user;

pub use config::Config;
pub use error::Error;
pub use robot::Robot;
pub use room::Room;
pub use user::User;
