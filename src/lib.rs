//! Crate `rustin` is an extensible chat bot framework.

#![deny(missing_docs)]
#![feature(
    arbitrary_self_types,
    async_await,
    await_macro,
    existential_type,
    futures_api,
    pin
)]

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
    use futures::{Future, Stream};

    use super::chat_service::ChatService;
    use super::message::{IncomingMessage, OutgoingMessage};
    use super::storage::{Memory, Store};
    use super::{ActionStream, Callback, Error, Robot, Room};

    #[derive(Clone, Debug)]
    struct NullChat;

    impl ChatService for NullChat {
        existential type Success: Future<Output = Result<(), Error>>;

        existential type Incoming: Stream<Item = Result<IncomingMessage, Error>>;

        fn join(&self, _room: &Room) -> Self::Success {
            ok(())
        }

        fn part(&self, _room: &Room) -> Self::Success {
            ok(())
        }

        fn send_message(&self, _message: OutgoingMessage) -> Self::Success {
            ok(())
        }

        fn incoming(&self) -> Self::Incoming {
            empty()
        }
    }

    #[test]
    fn manual_stateless_callback() {
        struct Echo;

        impl<S> Callback<S> for Echo {
            fn call(&self, message: IncomingMessage, _store: S) -> ActionStream {
                Box::new(once(Ok(message.reply(message.body()))))
            }
        }

        Robot::build(NullChat, Memory::new())
            .callback(Echo)
            .finish();
    }

    #[test]
    fn fn_stateless_callback() {
        fn echo(message: IncomingMessage) -> ActionStream {
            Box::new(once(Ok(message.reply(message.body()))))
        }

        Robot::build(NullChat, Memory::new())
            .callback(echo)
            .finish();
    }

    #[test]
    fn manual_stateful_callback() {
        struct WelcomeBack;

        impl<S> Callback<S> for WelcomeBack
        where
            S: Store,
        {
            fn call(&self, message: IncomingMessage, state: S) -> ActionStream {
                let id = message.user().id();

                if state.get(id).is_some() {
                    Box::new(once(Ok(message.reply(format!(
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
}
