use crate::models::{Binary, CommandParseError, BUILTINS};
use std::{env, io, path::Path, process::Command};

pub fn cd_command(args: Vec<String>) -> Result<String, CommandParseError> {
    let home = env::var("HOME").unwrap();
    let args = args.join("");

    let path = if args.trim() == "~" {
        Path::new(&home).to_path_buf()
    } else if args.starts_with("~") {
        Path::new(&home).join(&args[1..])
    } else {
        Path::new(&args).to_path_buf()
    };

    let cd = env::set_current_dir(&path);

    if cd.is_err() {
        let error_msg = format!("cd: {}: No such file or directory", path.display());
        return Err(CommandParseError(error_msg));
    }

    Ok(String::new())
}

pub fn pwd_command() -> Result<String, CommandParseError> {
    let dir = env::current_dir();

    if let Ok(dir) = dir {
        return Ok(dir.to_str().unwrap().to_string());
    }

    Err(CommandParseError("Incorrect directory".to_string()))
}

pub fn type_command(args: Vec<String>) -> Result<String, CommandParseError> {
    let binaries = get_binaries().unwrap();
    let args = args.first();

    if args.is_none() {
        return Err(CommandParseError("No arguments supplied".to_string()));
    }

    let args = args.unwrap().as_str();

    if BUILTINS.contains(&args) {
        return Ok(format!("{} is a shell builtin", args));
    }

    if let Some(binary) = binaries.iter().find(|binary| binary.name.eq(args)) {
        return Ok(format!("{} is {}", args, binary.path));
    }

    let error_msg = format!("{}: not found", args);
    Err(CommandParseError(error_msg))
}

pub fn run_binary(command: &str, args: Vec<String>) -> Result<String, CommandParseError> {
    let binaries = get_binaries().unwrap();
    let error_msg: String;

    if binaries.iter().find(|bin| bin.name.eq(command)).is_some() {
        let exec = Command::new(command).args(args).output();

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

fn get_binaries() -> Result<Vec<Binary>, io::Error> {
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
