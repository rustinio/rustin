//! Crate `rustin` is an extensible chat bot framework.

#![deny(missing_docs)]
#![feature(arbitrary_self_types, async_await, await_macro, futures_api, pin)]

mod callback;
pub mod chat_service;
mod config;
mod error;
pub mod message;
mod robot;
mod room;
pub mod storage;
mod user;

pub use crate::callback::{Action, ActionStream, Callback};
pub use crate::config::Config;
pub use crate::error::Error;
pub use crate::robot::{Builder, Robot};
pub use crate::room::Room;
pub use crate::user::User;

#[cfg(test)]
mod tests {
    use futures::future::ok;
    use futures::stream::{empty, once};

    use super::chat_service::{ChatService, Incoming, Success};
    use super::message::{IncomingMessage, OutgoingMessage};
    use super::storage::{Memory, Store};
    use super::{ActionStream, Callback, Robot, Room};

    #[derive(Clone, Debug)]
    struct NullChat;

    impl ChatService for NullChat {
        fn join(&self, _room: &Room) -> Success {
            Box::new(ok(()))
        }

        fn part(&self, _room: &Room) -> Success {
            Box::new(ok(()))
        }

        fn send_message(&self, _message: OutgoingMessage) -> Success {
            Box::new(ok(()))
        }

        fn incoming(&self) -> Incoming {
            Box::new(empty())
        }
    }

    #[test]
    fn manual_stateless_callback() {
        struct Echo;

        impl<S> Callback<S, (IncomingMessage,)> for Echo {
            fn call(&self, message: &IncomingMessage, _store: &mut S) -> ActionStream {
                Box::new(once(ok(message.reply(message.body()))))
            }
        }

        Robot::build(NullChat, Memory::new())
            .callback(Echo)
            .finish();
    }

    #[test]
    fn fn_stateless_callback() {
        fn echo(message: &IncomingMessage) -> ActionStream {
            Box::new(once(ok(message.reply(message.body()))))
        }

        Robot::build(NullChat, Memory::new())
            .callback(echo)
            .finish();
    }

    #[test]
    fn manual_stateful_callback() {
        struct WelcomeBack;

        impl<S> Callback<S, (IncomingMessage, S)> for WelcomeBack
        where
            S: Store,
        {
            fn call(&self, message: &IncomingMessage, state: &mut S) -> ActionStream {
                let id = message.user().id();

                if state.get(id).is_some() {
                    Box::new(once(ok(message.reply(format!(
                        "Hello again, {}!",
                        message.user().name().unwrap_or(id)
                    )))))
                } else {
                    state.set(id, "1");

                    Box::new(empty())
                }
            }
        }

        Robot::build(NullChat, Memory::new())
            .callback(WelcomeBack)
            .finish();
    }

    #[test]
    fn fn_stateful_callback() {
        fn echo<S>(message: &IncomingMessage, state: &mut S) -> ActionStream where S: Store {
            let id = message.user().id();

            if state.get(id).is_some() {
                Box::new(once(ok(message.reply(format!(
                    "Hello again, {}!",
                    message.user().name().unwrap_or(id)
                )))))
            } else {
                state.set(id, "1");

                Box::new(empty())
            }
        }

        Robot::build(NullChat, Memory::new())
            .callback(echo)
            .finish();
    }
}
