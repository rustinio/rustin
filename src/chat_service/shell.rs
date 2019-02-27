use std::future::Future;
use std::io::{self, BufRead, Write};
use std::pin::Pin;
use std::thread;
use std::time::Duration;

use futures::{channel::mpsc::channel, executor::block_on, future::ok};

use super::{ChatService, Incoming};
use crate::{
    message::{IncomingMessage, OutgoingMessage, Source},
    result::{Error, Success},
    user::User,
};

/// An adapter that runs in your shell.
#[derive(Clone, Debug)]
pub struct Shell {
    user: User,
}

impl Shell {
    /// Creates a new `Shell` with the given name for the robot.
    pub fn new(name: &str) -> Self {
        Shell {
            user: User::new("1", Some(name), None),
        }
    }
}

impl Default for Shell {
    fn default() -> Self {
        Shell {
            user: User::new("1", Some("Rustin"), None),
        }
    }
}

impl ChatService for Shell {
    fn send_message(&self, message: OutgoingMessage) -> Success {
        println!("{}", message);

        Box::pin(ok(()))
    }

    fn incoming(&self) -> Incoming {
        let (mut tx, rx) = channel(0);
        let robot = block_on(self.user()).expect("accessing robot user");
        let prompt = format!("{} > ", robot.username().expect("accessing username"));

        thread::spawn(move || {
            let input = io::stdin();
            let mut output = io::stdout();
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

                        let user = User::new("1", Some("Shell User"), None);
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

        Box::pin(rx)
    }

    fn user(&self) -> Pin<Box<dyn Future<Output = Result<User, Error>>>> {
        Box::pin(ok(self.user.clone()))
    }
}
