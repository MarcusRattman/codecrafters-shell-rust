mod commands;
mod helpers;
mod models;

use helpers::parse_command;
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
                    print!("{}", val);
                }
            }
            Err(e) => println!("{}", e.0),
        }
    }
}
