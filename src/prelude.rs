//! Types to be glob imported when writing a callback.

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
