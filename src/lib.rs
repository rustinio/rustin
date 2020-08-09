//! Crate `rustin` is an extensible chat bot framework.

#![deny(missing_docs)]

pub mod callback;
pub mod chat_service;
pub mod config;
pub mod message;
pub mod prelude;
pub mod result;
pub mod robot;
pub mod room;
pub mod route;
pub mod store;
pub mod user;

#[cfg(test)]
mod tests {
    use std::{pin::Pin, sync::Arc};

    use futures::{
        future::{ok, Future},
        stream::empty,
    };

    use super::{
        chat_service::Incoming,
        message::OutgoingMessage,
        prelude::*,
        store::Memory,
        user::User,
    };

    #[derive(Clone, Debug)]
    struct NullChat;

    impl ChatService for NullChat {
        fn send_message(&self, _message: OutgoingMessage) -> Success {
            Box::pin(ok(()))
        }

        fn incoming(&self, _alias: Option<String>) -> Incoming {
            Box::pin(empty())
        }

        fn user(&self) -> Pin<Box<dyn Future<Output = Result<User, Error>>>> {
            Box::pin(ok(User::new("null", None, None)))
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
                    match store.get(&id).await {
                        Ok(Some(id)) => {
                            chat.send_message(message.reply(format!(
                                "Hello again, {}!",
                                message.user().display_name().unwrap_or(&id)
                            )));

                            Ok(())
                        }
                        Ok(None) => match store.set(id, "1").await {
                            Ok(_) => Ok(()),
                            Err(error) => Err(Error::custom(error.to_string())),
                        },
                        Err(error) => Err(Error::custom(error.to_string())),
                    }
                };

                Box::pin(future)
            }
        }

        Robot::build(Config::default(), NullChat, Memory::new())
            .route(Route::new(r".*", true, "welcome.back", WelcomeBack).unwrap())
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
                match store.get(&id).await {
                    Ok(Some(id)) => {
                        chat.send_message(message.reply(format!(
                            "Hello again, {}!",
                            message.user().display_name().unwrap_or(&id)
                        )));

                        Ok(())
                    }
                    Ok(None) => match store.set(id, "1").await {
                        Ok(_) => Ok(()),
                        Err(error) => Err(Error::custom(error.to_string())),
                    },
                    Err(error) => Err(Error::custom(error.to_string())),
                }
            };

            Box::pin(future)
        }

        Robot::build(Config::default(), NullChat, Memory::new())
            .route(Route::new(r".*", true, "welcome.back", welcome_back).unwrap())
            .finish();
    }
}
