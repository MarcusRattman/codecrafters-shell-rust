#[allow(unused_imports)]
use std::io::{self, Error, Write};
use std::{env, path::Path, process::exit, process::Command};

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

fn parse_command(input: &str) -> Result<(), CommandParseError> {
    let input = input.trim();
    let mut parts = input.splitn(2, ' ');
    let command = parts.next().ok_or(CommandParseError)?;
    let args = parts.next().unwrap_or("");

    match_command(command, args);
    Ok(())
}

fn match_command(command: &str, args: &str) {
    match command {
        "exit" => exit(args.parse().unwrap_or(-1)),
        "echo" => println!("{}", args),
        "type" => type_command(args),
        _ => run_binary(command, args),
    }
}

fn type_command(command: &str) {
    let builtin = ["exit", "echo", "type"];

    let binaries = get_binaries().unwrap();

    if builtin.contains(&command) {
        println!("{} is a shell builtin", command);
        return;
    }

    if let Some(binary) = binaries.iter().find(|binary| binary.name.eq(command)) {
        println!("{} is {}", command, binary.path);
        return;
    }

    println!("{}: not found", command);
}

struct Binary {
    path: String,
    name: String,
}

fn run_binary(command: &str, args: &str) {
    let path_var = env::var("PATH");

    if let Ok(path) = path_var {
        let test = Command::new(command).arg(args).output().unwrap();
        println!("{:?}", test.stdout)
    }
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
