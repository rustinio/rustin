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
    robot::{Builder, Robot},
    room::Room,
    user::User,
    store::Store,
};

#[cfg(test)]
mod tests {
    use futures::future::{ok, ready};
    use futures::prelude::*;
    use futures::stream::{empty, once};

    use super::chat_service::{ChatService, Incoming, Success};
    use super::message::{IncomingMessage, OutgoingMessage};
    use super::store::{Memory, Store};
    use super::{ActionStream, Callback, FutureActionStream, Robot};

    #[derive(Clone, Debug)]
    struct NullChat;

    impl ChatService for NullChat {
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
            fn call(&self, message: &IncomingMessage, _store: &mut S) -> FutureActionStream {
                let body = message.body();
                let reply = message.reply(body);
                let future_reply = ready(reply);
                let stream = Box::new(once(future_reply)) as ActionStream;

                Box::new(ok(stream))
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
                let future_reply = ready(reply);
                let stream = Box::new(once(future_reply)) as ActionStream;

                Box::new(ok(stream))
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
            fn call(&self, message: &IncomingMessage, store: &mut S) -> FutureActionStream {
                let id = message.user().id();

                Box::new(store.get(id).then(|result| {
                    match result {
                        Ok(Some(id)) => {
                            let reply = message.reply(format!(
                                "Hello again, {}!",
                                message.user().name().unwrap_or(&id)
                                ));
                            let future_reply = ready(reply);
                            let stream = Box::new(once(future_reply)) as ActionStream;

                            Box::new(ok(stream))
                        }
                        Ok(None) => {
                            if let Err(_) = await!(store.set(id, "1")) {
                                panic!("store error");
                            }

                            Box::new(empty())
                        }
                        Err(_) => panic!("store error"),
                    }
                }))
            }
        }

        Robot::build(NullChat, Memory::new())
            .callback(WelcomeBack)
            .finish();
    }

    // #[test]
    // fn fn_stateful_callback() {
    //     fn echo<S>(message: &IncomingMessage, store: &mut S) -> FutureActionStream where S: Store {
    //         let id = message.user().id();

    //         store.get(id).then(|result| {
    //             match result {
    //                 Ok(Some(id)) =>  {
    //                     Box::new(once(ok(message.reply(format!(
    //                         "Hello again, {}!",
    //                         message.user().name().unwrap_or(&id)
    //                     ))))) as FutureActionStream
    //                 }
    //                 Ok(None) => {
    //                     store.set(id, "1");

    //                     Box::new(empty()) as FutureActionStream
    //                 }
    //                 _ => panic!("store error"),
    //             }
    //         }).into_stream()
    //     }

    //     Robot::build(NullChat, Memory::new())
    //         .callback(echo)
    //         .finish();
    // }
}
