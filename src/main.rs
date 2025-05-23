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
        io::stdin().read_line(&mut input).unwrap();

        let result = parse_command(&input);

        match result {
            Ok(val) => {
                if val.len() > 0 {
                    println!("{}", val);
                }
            }
            Err(e) => match e {
                CommandParseError::CommandNotFound(e) => println!("{}", e),
                CommandParseError::ComposableError(e) => match e {
                    IOError::NoSuchDir(e) => println!("{}", e),
                    IOError::StdError(e) => println!("{}", e),
                },
                //CommandParseError::BinExecError(e) => println!("{}", e),
                CommandParseError::WrongArgsNum => (),
            },
        }
    }
}
