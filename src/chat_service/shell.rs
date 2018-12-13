use std::io::{self, BufRead, Write};
use std::thread;
use std::time::Duration;

use futures::channel::mpsc::channel;
use futures::future::{err, ok};
use futures::{Future, SinkExt, Stream};

use super::ChatService;
use crate::error::Error;
use crate::message::{IncomingMessage, OutgoingMessage, Source};
use crate::room::Room;
use crate::user::User;

/// An adapter that runs in your shell.
#[derive(Clone, Debug)]
pub struct Shell;

impl ChatService for Shell {
    existential type Success: Future<Output = Result<(), Error>>;
    existential type Incoming: Stream<Item = Result<IncomingMessage, Error>>;

    fn join(&self, _room: &Room) -> Self::Success {
        err(Error)
    }

    fn part(&self, _room: &Room) -> Self::Success {
        err(Error)
    }

    fn send_message(&self, message: OutgoingMessage) -> Self::Success {
        println!("{}", message);

        ok(())
    }

    fn incoming(&self) -> Self::Incoming {
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

                        match tx.try_send(Ok(message)) {
                            Ok(_) => {
                                // Hack to keep the prompt from appearing before callbacks have
                                // finished responding.
                                thread::sleep(duration);

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

        rx
    }
}
