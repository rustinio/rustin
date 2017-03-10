#![allow(dead_code)]

extern crate futures;

mod adapter;
mod correspondent;
mod error;
mod handler;
mod message;
mod robot;
mod room;
mod user;

pub use adapter::{Adapter, Shell};
pub use correspondent::{Source, Target};
pub use error::Error;
pub use handler::{Action, Handler};
pub use message::{IncomingMessage, OutgoingMessage};
pub use robot::{Config, run};
pub use room::Room;
pub use user::User;
