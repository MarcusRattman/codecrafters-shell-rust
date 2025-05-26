mod misc;
use misc::parsers::parse_input;
use std::io::{self, Write};

fn main() {
    loop {
        let mut input = String::new();
        print!("$ ");
        io::stdout().flush().unwrap();
        io::stderr().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();

        let result = parse_input(&input);

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
