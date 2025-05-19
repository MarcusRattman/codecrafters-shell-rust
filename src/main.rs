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

fn get_binaries() -> Result<Vec<Binary>, Error> {
    let path_var = env::var("PATH");

    match path_var {
        Ok(path) => {
            let mut binaries = Vec::<Binary>::new();

            let paths = path
                .split(":")
                .map(|test| Path::new(test))
                .filter(|path| path.exists() && path.is_dir())
                .collect::<Vec<&Path>>();

            for path in paths {
                for entry in path.read_dir()? {
                    let entry = entry?;
                    if entry.path().is_dir() {
                        continue;
                    }

                    let binary_path = entry.path().to_str().unwrap().to_string();
                    let binary_name = entry.file_name().to_str().unwrap().to_string();

                    let bin = Binary {
                        path: binary_path,
                        name: binary_name,
                    };

                    binaries.push(bin);
                }
            }

            return Ok(binaries);
        }

        Err(_) => Ok(vec![]),
    }
}
