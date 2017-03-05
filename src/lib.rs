#![allow(dead_code)]

extern crate futures;

mod adapter;
mod config;
mod correspondent;
mod error;
mod handler;
mod message;
mod robot;
mod room;
mod user;

pub use adapter::{Adapter, Shell};
pub use config::Config;
pub use correspondent::{Source, Target};
pub use error::Error;
pub use handler::{Action, Handler};
pub use message::{IncomingMessage, OutgoingMessage};
pub use robot::Robot;
pub use room::Room;
pub use user::User;
