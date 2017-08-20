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

#[cfg(test)]
mod tests {
    use futures::future::ok;
    use futures::stream::{empty, once};
    use futures::{Future, Stream};

    use super::chat_service::ChatService;
    use super::message::{IncomingMessage, OutgoingMessage};
    use super::storage::Memory;
    use super::{Action, Callback, Error, Robot, Room};

    #[derive(Clone, Debug)]
    struct NullChat;

    impl ChatService for NullChat {
        fn join(&self, _room: &Room) -> Box<Future<Item = (), Error = Error>> {
            Box::new(ok(()))
        }

        fn part(&self, _room: &Room) -> Box<Future<Item = (), Error = Error>> {
            Box::new(ok(()))
        }

        fn send_message(&self, _message: OutgoingMessage) -> Box<Future<Item = (), Error = Error>> {
            Box::new(ok(()))
        }

        fn incoming(&self) -> Box<Stream<Item = IncomingMessage, Error = Error>> {
            Box::new(empty())
        }
    }

    #[test]
    fn manual_stateless_callback() {
        struct Echo;

        impl Callback for Echo {
            fn call(&self, message: IncomingMessage) -> Box<Stream<Item = Action, Error = Error>> {
                Box::new(once(Ok(message.reply(message.body()))))
            }
        }

        Robot::build(NullChat, Memory::new())
            .callback(Echo)
            .finish();
    }

    #[test]
    fn fn_stateless_callback() {
        fn echo(message: IncomingMessage) -> Box<Stream<Item = Action, Error = Error>> {
            Box::new(once(Ok(message.reply(message.body()))))
        }

        Robot::build(NullChat, Memory::new())
            .callback(echo)
            .finish();
    }
}
