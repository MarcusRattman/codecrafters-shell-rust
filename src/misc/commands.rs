use super::models::{Binary, CommandParseError, IOError, IOStream, BUILTINS};
use std::{env, io::Error, path::Path, process::Command};

pub fn echo_command(args: Vec<String>) -> IOStream {
    let result = format!("{}", args.join(" "));
    IOStream::new(&result, "")
}

pub fn cd_command(args: Vec<String>) -> Result<(), IOError> {
    let home = env::var("HOME").unwrap_or_else(|_| "/".to_string());
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
        return Err(IOError::NoSuchDir(error_msg));
    }

    Ok(())
}

pub fn pwd_command() -> Result<IOStream, IOError> {
    let dir = env::current_dir();

    if let Ok(dir) = dir {
        let dir = dir.to_str().unwrap();
        let result = IOStream::new(dir, "");

        return Ok(result);
    }

    Err(IOError::NoSuchDir("Incorrect directory".to_string()))
}

pub fn type_command(args: Vec<String>) -> Result<IOStream, CommandParseError> {
    let binaries = get_binaries().unwrap();
    let args = args.first();

    if args.is_none() {
        return Err(CommandParseError::WrongArgsNum);
    }

    let args = args.unwrap().as_str();

    if BUILTINS.contains(&args) {
        let stdout = format!("{} is a shell builtin", args);
        let result = IOStream::new(&stdout, "");
        return Ok(result);
    }

    if let Some(binary) = binaries.iter().find(|binary| binary.name.eq(args)) {
        let stdout = format!("{} is {}", args, binary.path);
        let result = IOStream::new(&stdout, "");
        return Ok(result);
    }

    let error_msg = format!("{}: not found", args);
    Err(CommandParseError::CommandNotFound(error_msg))
}

pub fn run_binary(command: String, args: Vec<String>) -> Result<IOStream, IOError> {
    let binaries = get_binaries().unwrap();

    if binaries.iter().find(|bin| bin.name.eq(&command)).is_some() {
        let exec = Command::new(&command).args(args).output();

        // non-empty stderr doesn't mean exec will be Err
        // so we're splitting both outputs and forming an IOStream object

        if let Ok(output) = exec {
            let stdout = String::from_utf8(output.stdout).unwrap();
            let stderr = String::from_utf8(output.stderr).unwrap();
            let result = IOStream::new(stdout.trim(), stderr.trim());

            return Ok(result);
        }

        let err = IOError::StdError(exec.unwrap_err());
        Err(err)
    } else {
        let error_msg = format!("{}: not found", command);
        Err(IOError::NoSuchDir(error_msg))
    }
}

fn get_binaries() -> Result<Vec<Binary>, Error> {
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
                Some(Binary::new(path_str, name))
            } else {
                None
            }
        })
        .collect();

    Ok(binaries)
}
