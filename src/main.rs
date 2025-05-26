mod misc;
use crate::misc::parsers::parse_command;
//use parsers::parse_command;
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
            Err(e) => println!("{}", e),
        }
    }
}
