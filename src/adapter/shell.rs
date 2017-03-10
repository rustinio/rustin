use std::io::{self, BufRead, Write};
use std::thread;
use std::time::Duration;

use futures::future::{err, ok};
use futures::sync::mpsc::channel;
use futures::{Future, Sink, Stream};
use correspondent::Source;
use error::Error;
use message::{IncomingMessage, OutgoingMessage};
use robot::Config;
use room::Room;
use super::Adapter;
use user::User;

/// An adapter that runs in your shell.
#[derive(Clone, Debug)]
pub struct Shell {
    config: Config,
}

impl Shell {
    pub fn new(config: Config) -> Self {
        Shell {
            config: config,
        }
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
            let mut output = io::stdout();
            let prompt = "Rustin > ";
            let duration = Duration::from_millis(10);
            let mut dirty_exit = true;

            println!("Type \"exit\" or \"quit\" to end the session.");
            print!("{}", prompt);
            output.flush().unwrap();

            for line in input.lock().lines() {
                match line {
                    Ok(body) => {
                        if body.len() == 0 {
                            print!("{}", prompt);
                            output.flush().unwrap();

                            continue;
                        } else if body == "exit" || body == "quit" {
                            dirty_exit = false;

                            break;
                        }

                        let user = User::new("1", Some("Shell User"));
                        let source = Source::User(user);
                        let message = IncomingMessage::new(source, body);

                        match tx.send(Ok(message)).wait() {
                            Ok(new_tx) => {
                                // Hack to keep the prompt from appearing before handlers have
                                // finished responding.
                                thread::sleep(duration);

                                tx = new_tx;

                                print!("{}", prompt);
                                output.flush().unwrap();
                            }
                            Err(_) => break,
                        }
                    }
                    Err(_) => break,
                }
            }

            if dirty_exit {
                println!();
            }
        });

        Box::new(rx.then(|result| {
            result.expect("futures::sync::mpsc::Receiver cannot generate an error")
        }))
    }
}
