//! Crate `rustin` is an extensible chat bot framework.

#![deny(missing_docs)]
#![feature(arbitrary_self_types, async_await, await_macro, futures_api)]

mod callback;
pub mod chat_service;
mod config;
mod error;
pub mod message;
mod robot;
mod room;
pub mod store;
mod user;

pub use crate::{
    callback::{Action, ActionStream, Callback, FutureActionStream},
    config::Config,
    error::Error,
    robot::{
        Builder,
        Robot,
        handle::Handle,
    },
    room::Room,
    store::Store,
    user::User,
};

#[cfg(test)]
mod tests {
    use futures::future::ok;
    use futures::stream::empty;

    use super::chat_service::{ChatService, Incoming, Success};
    use super::message::{IncomingMessage, OutgoingMessage};
    use super::store::{Memory, Store};
    use super::user::User;
    use super::{Action, Callback, FutureActionStream, Robot};

    #[derive(Clone, Debug)]
    struct NullChat;

    impl ChatService for NullChat {
        fn send_message(&self, _message: OutgoingMessage) -> Success {
            Box::pin(ok(()))
        }

        fn incoming(&self) -> Incoming {
            Box::pin(empty())
        }

        fn user(&self) -> Option<&User> {
            None
        }
    }

    #[test]
    fn manual_stateless_callback() {
        struct Echo;

        impl<S> Callback<S, (IncomingMessage,)> for Echo {
            fn call(&self, message: &IncomingMessage, _store: &S) -> FutureActionStream {
                let body = message.body();
                let reply = message.reply(body);

                reply.into()
            }
        }

        Robot::build(NullChat, Memory::new())
            .callback(Echo)
            .finish();
    }

    #[test]
    fn fn_stateless_callback() {
        fn echo(message: &IncomingMessage) -> FutureActionStream {
            let body = message.body();
            let reply = message.reply(body);

            reply.into()
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
            fn call(&self, message: &IncomingMessage, store: &S) -> FutureActionStream {
                let message = message.clone();
                let id = message.user().id().to_owned();
                let store = store.clone();

                let future = async move {
                    match await!(store.get(&id)) {
                        Ok(Some(id)) => {
                            let reply = message.reply(format!(
                                "Hello again, {}!",
                                message.user().name().unwrap_or(&id)
                            ));
                            let stream = reply.into();

                            Ok(stream)
                        }
                        Ok(None) => {
                            if let Err(_) = await!(store.set(id, "1")) {
                                panic!("store error");
                            }

                            let stream = Action::empty_stream();

                            Ok(stream)
                        }
                        Err(_) => panic!("store error"),
                    }
                };

                Box::pin(future)
            }
        }

        Robot::build(NullChat, Memory::new())
            .callback(WelcomeBack)
            .finish();
    }

    #[test]
    fn fn_stateful_callback() {
        fn echo<S>(message: &IncomingMessage, store: &S) -> FutureActionStream
        where
            S: Store,
        {
            let message = message.clone();
            let id = message.user().id().to_owned();
            let store = store.clone();

            let future = async move {
                match await!(store.get(&id)) {
                    Ok(Some(id)) => {
                        let reply = message.reply(format!(
                            "Hello again, {}!",
                            message.user().name().unwrap_or(&id)
                        ));
                        let stream = reply.into();

                        Ok(stream)
                    }
                    Ok(None) => {
                        if let Err(_) = await!(store.set(id, "1")) {
                            panic!("store error");
                        }

                        let stream = Action::empty_stream();

                        Ok(stream)
                    }
                    Err(_) => panic!("store error"),
                }
            };

            Box::pin(future)
        }

        Robot::build(NullChat, Memory::new())
            .callback(echo)
            .finish();
    }
}
