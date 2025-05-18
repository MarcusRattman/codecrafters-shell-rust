#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::exit;

#[derive(Debug)]
struct CommandParseError;

fn main() {
    loop {
        let mut input = String::new();
        print!("$ ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();

        let result = parse_command(&input);

        if let Err(_) = result {
            println!("{}: command not found", input.trim());
        }
    }
}

fn parse_command(input_string: &str) -> Result<(), CommandParseError> {
    let input = input_string.trim();
    let space_index = input.find(" ");

    if let None = space_index {
        return Err(CommandParseError);
    }

    let space_index = space_index.unwrap();

    let command = input[0..space_index].trim();
    let args = input_string[space_index + 1..].trim();

    match_command(command, args);

    return Ok(());
}

fn match_command(command: &str, args: &str) {
    let command_list = ["exit", "echo", "type"];
    match command {
        "exit" => exit(args.parse().unwrap_or(-1)),
        "echo" => println!("{}", args),
        "type" => {
            if command_list.contains(&args) {
                println!("{} is a shell builtin", args);
            } else {
                println!("{}: command not found", args);
            }
        }
        _ => (),
    }
}
