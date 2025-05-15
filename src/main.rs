#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        let mut input = String::new();
        print!("$ ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();

        if input.trim().eq("exit 0") {
            return;
        }

        println!("{}: command not found", input.trim());
    }
}
