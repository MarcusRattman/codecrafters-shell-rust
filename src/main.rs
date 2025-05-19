#[allow(unused_imports)]
use std::io::{self, Error, Write};
use std::{env, path::Path, process::exit};

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
    match command {
        "exit" => exit(args.parse().unwrap_or(-1)),
        "echo" => println!("{}", args),
        "type" => type_command(args),
        _ => (),
    }
}

fn type_command(command: &str) {
    let builtin = ["exit", "echo", "type"];

    let binaries = get_binaries().unwrap();

    if builtin.contains(&command) {
        println!("{} is a shell builtin", command);
    } else if let Some(binary) = binaries.iter().find(|binary| binary.name.eq(command)) {
        println!("{} is {}", command, binary.path);
    } else {
        println!("{}: not found", command);
    }
}

struct Binary {
    path: String,
    name: String,
}

fn get_binaries() -> io::Result<Vec<Binary>> {
    let path_var = env::var("PATH").unwrap_or_default();

    let binaries = path_var
        .split(':')
        .filter_map(|dir| {
            let dir_path = Path::new(dir);
            if dir_path.exists() && dir_path.is_dir() {
                dir_path.read_dir().ok()
            } else {
                None
            }
        })
        .flatten()
        .filter_map(|entry| {
            let path = entry.unwrap().path();
            if path.is_file() {
                let path_str = path.to_str()?.to_string();
                let name = path.file_name()?.to_str()?.to_string();
                Some(Binary {
                    path: path_str,
                    name,
                })
            } else {
                None
            }
        })
        .collect();

    Ok(binaries)
}
