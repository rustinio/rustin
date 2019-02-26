//! Crate `rustin` is an extensible chat bot framework.

#![deny(missing_docs)]
#![feature(arbitrary_self_types, async_await, await_macro, futures_api)]

mod callback;
pub mod chat_service;
mod config;
pub mod message;
mod result;
mod robot;
pub mod room;
mod route;
pub mod store;
pub mod user;

pub use crate::{
    callback::Callback,
    chat_service::ChatService,
    config::Config,
    message::IncomingMessage,
    result::{Error, Success},
    robot::Robot,
    route::Route,
    store::Store,
};

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use regex::Regex;

    use futures::{future::ok, stream::empty};

    use super::{
        callback::Callback,
        chat_service::{ChatService, Incoming},
        message::{IncomingMessage, OutgoingMessage},
        result::{Error, Success},
        robot::Robot,
        route::Route,
        store::{Memory, Store},
        user::User,
    };

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
    fn manual_callback() {
        struct WelcomeBack;

        impl<C, S> Callback<C, S> for WelcomeBack
        where
            C: ChatService + 'static,
            S: Store,
        {
            fn call(&self, chat: Arc<C>, message: &IncomingMessage, store: S) -> Success {
                let message = message.clone();
                let id = message.user().id().to_owned();

                let future = async move {
                    match await!(store.get(&id)) {
                        Ok(Some(id)) => {
                            chat.send_message(message.reply(format!(
                                "Hello again, {}!",
                                message.user().name().unwrap_or(&id)
                            )));

                            Ok(())
                        }
                        Ok(None) => match await!(store.set(id, "1")) {
                            Ok(_) => Ok(()),
                            Err(_) => Err(Error),
                        },
                        Err(_) => Err(Error),
                    }
                };

                Box::pin(future)
            }
        }

        Robot::build(NullChat, Memory::new())
            .route(Route::new(
                Regex::new(r".*").unwrap(),
                "welcome.back",
                WelcomeBack,
            ))
            .finish();
    }

    #[test]
    fn fn_stateful_callback() {
        fn welcome_back<C, S>(chat: Arc<C>, message: &IncomingMessage, store: S) -> Success
        where
            C: ChatService + 'static,
            S: Store,
        {
            let message = message.clone();
            let id = message.user().id().to_owned();

            let future = async move {
                match await!(store.get(&id)) {
                    Ok(Some(id)) => {
                        chat.send_message(message.reply(format!(
                            "Hello again, {}!",
                            message.user().name().unwrap_or(&id)
                        )));

                        Ok(())
                    }
                    Ok(None) => match await!(store.set(id, "1")) {
                        Ok(_) => Ok(()),
                        Err(_) => Err(Error),
                    },
                    Err(_) => Err(Error),
                }
            };

            Box::pin(future)
        }

        Robot::build(NullChat, Memory::new())
            .route(Route::new(
                Regex::new(r".*").unwrap(),
                "welcome.back",
                welcome_back,
            ))
            .finish();
    }
}
