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
mod route;
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
    route::Route,
    store::Store,
    user::User,
};

#[cfg(test)]
mod tests {
    use regex::Regex;

    use futures::{
        future::ok,
        stream::empty,
    };

    use super::{
        chat_service::{ChatService, Incoming, Success},
        message::{IncomingMessage, OutgoingMessage},
        robot::handle::Handle,
        store::{Memory, Store},
        user::User,
        Action,
        Callback,
        FutureActionStream,
        Robot,
        Route,
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
    fn manual_stateless_callback() {
        struct Echo;

        impl<S> Callback<S> for Echo
        where
            S: Store
        {
            fn call(&self, handle: Handle<S>) -> FutureActionStream {
                let body = handle.message_body();
                let reply = handle.reply(body);

                reply.into()
            }
        }

        Robot::build(NullChat, Memory::new())
            .route(Route::new(Regex::new(r".*").unwrap(), "echo", Echo))
            .finish();
    }

    #[test]
    fn fn_stateless_callback() {
        fn echo<S>(handle: Handle<S>) -> FutureActionStream
        where
            S: Store
        {
            let body = handle.message_body();
            let reply = handle.reply(body);

            reply.into()
        }

        Robot::build(NullChat, Memory::new())
            .route(Route::new(Regex::new(r".*").unwrap(), "echo", echo))
            .finish();
    }

    #[test]
    fn manual_stateful_callback() {
        struct WelcomeBack;

        impl<S> Callback<S> for WelcomeBack
        where
            S: Store,
        {
            fn call(&self, handle: Handle<S>) -> FutureActionStream {
                let id = handle.user().id().to_owned();

                let future = async move {
                    match await!(handle.get(&id)) {
                        Ok(Some(id)) => {
                            let reply = handle.reply(format!(
                                "Hello again, {}!",
                                handle.user().name().unwrap_or(&id)
                            ));
                            let stream = reply.into();

                            Ok(stream)
                        }
                        Ok(None) => {
                            if let Err(_) = await!(handle.set(id, "1")) {
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
            .route(Route::new(Regex::new(r".*").unwrap(), "welcome.back", WelcomeBack))
            .finish();
    }

    #[test]
    fn fn_stateful_callback() {
        fn welcome_back<S>(handle: Handle<S>) -> FutureActionStream
        where
            S: Store,
        {
            let id = handle.user().id().to_owned();

            let future = async move {
                match await!(handle.get(&id)) {
                    Ok(Some(id)) => {
                        let reply = handle.reply(format!(
                            "Hello again, {}!",
                            handle.user().name().unwrap_or(&id)
                        ));
                        let stream = reply.into();

                        Ok(stream)
                    }
                    Ok(None) => {
                        if let Err(_) = await!(handle.set(id, "1")) {
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
            .route(Route::new(Regex::new(r".*").unwrap(), "welcome.back", welcome_back))
            .finish();
    }
}
