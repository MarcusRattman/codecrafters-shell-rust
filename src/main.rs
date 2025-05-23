mod commands;
mod helpers;
mod models;

use helpers::parse_command;
use models::{CommandParseError, IOError};
use std::io::{self, Write};

fn main() {
    loop {
        let mut input = String::new();
        print!("$ ");
        io::stdout().flush().unwrap();
        io::stderr().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();

        let result = parse_command(&input);

        match result {
            Ok(stream) => {
                if let Some(s) = stream.print() {
                    println!("{}", s);
                }
            }
            Err(e) => match e {
                //CommandParseError::CommandNotFound(e) => println!("{}", e),
                CommandParseError::ComposableError(e) => match e {
                    IOError::StreamError(stream) => println!("{}", stream.stderr.unwrap()),
                    e => println!("{}", e),
                    //IOError::NoSuchDir(e) => println!("{}", e),
                    //IOError::StdError(e) => println!("{}", e),
                },
                e => println!("{}", e),
                //CommandParseError::WrongArgsNum => (),
            },
        }
    }
}
