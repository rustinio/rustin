use std::io::{self, BufRead, Error as IoError};
use std::thread;

use futures::future::{err, ok};
use futures::sync::mpsc::channel;
use futures::{Future, Sink, Stream};

use correspondent::Source;
use error::Error;
use message::{IncomingMessage, OutgoingMessage};
use room::Room;
use super::Adapter;
use user::User;

#[derive(Debug)]
pub struct Shell;

impl Shell {
    pub fn new() -> Self {
        Shell
    }
}

impl Adapter for Shell {
    fn join(&self, _room: &Room) -> Box<Future<Item = (), Error = Error>> {
        Box::new(err(Error))
    }

    fn part(&self, _room: &Room) -> Box<Future<Item = (), Error = Error>> {
        Box::new(err(Error))
    }

    fn send_message(&self, message: OutgoingMessage) -> Box<Future<Item = (), Error = Error>> {
        println!("{}", message.body());

        Box::new(ok(()))
    }

    fn send_messages(&self, messages: &[OutgoingMessage])
    -> Box<Future<Item = (), Error = Error>> {

        for message in messages {
            println!("{}", message.body());
        }

        Box::new(ok(()))
    }

    fn incoming(&self) -> Box<Stream<Item = IncomingMessage, Error = Error>> {
        let (mut tx, rx) = channel(0);

        thread::spawn(move || {
            let input = io::stdin();

            for line in input.lock().lines() {
                match line {
                    Ok(body) => {
                        let user = User::new("1", Some("Shell User"));
                        let source = Source::User(user);
                        let message = IncomingMessage::new(source, body);

                        match tx.send(Ok(message)).wait() {
                            Ok(new_tx) => tx = new_tx,
                            Err(_) => break,
                        }
                    }
                    Err(_) => break,
                }
            }
        });

        Box::new(rx.then(|result| {
            result.expect("futures::sync::mpsc::Receiver cannot generate an error")
        }))
    }
}
