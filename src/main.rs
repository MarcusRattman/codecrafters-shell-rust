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
    let input = input.trim();

    // Split into command and args
    let mut parts = input.splitn(2, ' ');
    let command = parts.next().unwrap_or("").trim();
    let args_str = parts.next().unwrap_or("").trim();

    // Function to parse args respecting single quotes
    fn tokenize_args(s: &str) -> Vec<String> {
        let mut args = Vec::new();
        let mut current = String::new();
        let mut in_quotes = false;
        let mut chars = s.chars().peekable();

        while let Some(c) = chars.next() {
            match c {
                '\'' => {
                    in_quotes = !in_quotes;
                }
                ' ' if !in_quotes => {
                    if !current.is_empty() {
                        args.push(current);
                        current = String::new();
                    }
                }
                _ => {
                    current.push(c);
                }
            }
        }
        if !current.is_empty() {
            args.push(current);
        }

        // Remove surrounding quotes from args if present
        args.into_iter()
            .map(|arg| {
                let arg = arg.trim();
                if arg.starts_with('\'') && arg.ends_with('\'') && arg.len() >= 2 {
                    arg[1..arg.len() - 1].to_string()
                } else {
                    arg.to_string()
                }
            })
            .collect()
    }

    let args = tokenize_args(args_str);

    match command {
        "exit" => {
            let code: i32 = args.first().and_then(|s| s.parse().ok()).unwrap_or(-1);
            std::process::exit(code);
        }
        "echo" => Ok(args.join(" ")),
        "type" => {
            // Assuming type_command takes &str (the entire args string)
            // or modify as needed
            let args_str = args.join(" ");
            type_command(&args_str)
        }
        "pwd" => pwd_command(),
        "cd" => {
            let path = args.join(" ");
            cd_command(&path)
        }
        _ => {
            // Pass command and args as needed
            // For example, passing command and full args string
            run_binary(command, &args.join(" "))
        }
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

    let cd = env::set_current_dir(&path);

    if cd.is_err() {
        let error_msg = format!("cd: {}: No such file or directory", path.display());
        return Err(CommandParseError(error_msg));
    }

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
