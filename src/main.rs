use std::env;
use std::io::{self, Write};
use std::path::Path;
use std::process::{exit, Command};

#[derive(Debug)]
struct CommandParseError(String);

struct Binary {
    path: String,
    name: String,
}

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
            Err(e) => println!("{}", e.0),
        }
    }
}

const BUILTINS: &[&str] = &["exit", "echo", "type", "pwd", "cd"];

fn parse_command(input: &str) -> Result<String, CommandParseError> {
    let mut parts = input.trim().splitn(2, ' ');
    let command = parts.next().unwrap_or("");
    let args = parts.next().unwrap_or("");

    match command {
        "exit" => {
            let code: i32 = args.parse().unwrap_or(-1);
            exit(code);
        }
        "echo" => Ok(format!("{}", args)),
        "type" => type_command(args),
        "pwd" => pwd_command(),
        "cd" => cd_command(args),
        _ => run_binary(command, args),
    }
}

fn cd_command(args: &str) -> Result<String, CommandParseError> {
    let home = env::var("HOME").unwrap();

    let path = if args.trim() == "~" {
        Path::new(&home).to_path_buf()
    } else if args.starts_with("~") {
        Path::new(&home).join(&args[1..])
    } else {
        Path::new(args).to_path_buf()
    };

    env::set_current_dir(&path)
        .map_err(|e| CommandParseError(format!("cd: {}: {}", path.display(), e)))?;

    Ok(String::new())
}

fn pwd_command() -> Result<String, CommandParseError> {
    let dir = env::current_dir();

    if let Ok(dir) = dir {
        return Ok(dir.to_str().unwrap().to_string());
    }

    Err(CommandParseError("Incorrect directory".to_string()))
}

fn type_command(command: &str) -> Result<String, CommandParseError> {
    let binaries = get_binaries().unwrap();

    if BUILTINS.contains(&command) {
        return Ok(format!("{} is a shell builtin", command));
    }

    if let Some(binary) = binaries.iter().find(|binary| binary.name.eq(command)) {
        return Ok(format!("{} is {}", command, binary.path));
    }

    let error_msg = format!("{}: not found", command);
    Err(CommandParseError(error_msg))
}

fn run_binary(command: &str, args: &str) -> Result<String, CommandParseError> {
    let binaries = get_binaries().unwrap();
    let error_msg: String;

    if binaries.iter().find(|bin| bin.name.eq(command)).is_some() {
        let exec = Command::new(command).arg(args).output();

        if let Ok(output) = exec {
            let result = String::from_utf8(output.stdout).unwrap().trim().to_string();
            return Ok(result);
        }

        error_msg = "Error while running the binary".to_string();
        Err(CommandParseError(error_msg))
    } else {
        error_msg = format!("{}: command not found", command);
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
