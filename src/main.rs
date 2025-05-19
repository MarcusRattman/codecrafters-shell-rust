use std::env;
use std::io::{self, Write};
use std::path::Path;
use std::process::{exit, Command};

#[derive(Debug)]
struct CommandParseError(String);

fn main() {
    loop {
        let mut input = String::new();
        print!("$ ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();

        let result = parse_command(&input);
        if let Err(e) = result {
            println!("{}", e.0);
        }
    }
}

fn parse_command(input: &str) -> Result<(), CommandParseError> {
    let mut parts = input.trim().splitn(2, ' ');
    let command = parts.next().unwrap_or("");
    let args = parts.next().unwrap_or("");

    match command {
        "exit" => {
            let code: i32 = args.parse().unwrap_or(-1);
            exit(code);
        }
        "echo" => Ok(println!("{}", args)),
        "type" => type_command(args),
        "pwd" => Ok(println!("{}", pwd_command().unwrap_or_default())),
        _ => run_binary(command, args),
    }
}

fn pwd_command() -> Result<String, CommandParseError> {
    let dir = env::current_dir();

    if let Ok(dir) = dir {
        return Ok(dir.to_str().unwrap().to_string());
    }

    Err(CommandParseError("Incorrect directory".to_string()))
}

fn type_command(command: &str) -> Result<(), CommandParseError> {
    let builtin = ["exit", "echo", "type"];

    let binaries = get_binaries().unwrap();

    if builtin.contains(&command) {
        return Ok(println!("{} is a shell builtin", command));
    }

    if let Some(binary) = binaries.iter().find(|binary| binary.name.eq(command)) {
        return Ok(println!("{} is {}", command, binary.path));
    }

    let error_msg = format!("{}: not found", command);
    Err(CommandParseError(error_msg))
}

struct Binary {
    path: String,
    name: String,
}

fn run_binary(command: &str, args: &str) -> Result<(), CommandParseError> {
    let binaries = get_binaries().unwrap();

    if binaries.iter().find(|bin| bin.name.eq(command)).is_some() {
        let test = Command::new(command).arg(args).output().unwrap();
        io::stdout().write_all(&test.stdout).unwrap();
        Ok(())
    } else {
        let error_msg = format!("{}: command not found", command);
        Err(CommandParseError(error_msg))
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
