//! Crate `rustin` is an extensible chat bot framework.

#![deny(missing_debug_implementations)]
#![deny(missing_docs)]
#![deny(warnings)]

extern crate futures;

pub mod adapter;
mod correspondent;
mod error;
mod handler;
mod message;
mod robot;
mod room;
pub mod storage;
mod user;

#[doc(inline)]
pub use adapter::{Adapter, Shell};
pub use correspondent::{Source, Target};
pub use error::Error;
pub use handler::{Action, Handler};
pub use message::{IncomingMessage, OutgoingMessage};
pub use robot::{Config, run};
pub use room::Room;
#[doc(inline)]
pub use storage::{Memory, Store};
pub use user::User;
