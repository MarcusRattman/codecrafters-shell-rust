use crate::misc::commands::{cd_command, echo_command, pwd_command, run_binary, type_command};
use crate::misc::models::{CommandParseError, IOStream, IOStreamType, WriteMode};

use std::io::Error;
use std::process::exit;

pub fn write_to_file(
    iostream: IOStream,
    stream_to_write: &IOStreamType,
    filename: &str,
    writemode: WriteMode,
) -> Result<IOStream, Error> {
    iostream.write_to_file(&filename, &stream_to_write, &writemode)?;
    let result = match stream_to_write {
        IOStreamType::StdOut => IOStream {
            stdout: None,
            stderr: iostream.stderr,
        },
        IOStreamType::StdErr => IOStream {
            stdout: iostream.stdout,
            stderr: None,
        },
    };

    Ok(result)
}

pub fn exec_command(mut command: Vec<String>) -> Result<IOStream, CommandParseError> {
    let cmd = command.remove(0);
    let args = command;

    match cmd.as_str() {
        "exit" => {
            let code: i32 = args[0].parse().unwrap_or(-1);
            exit(code);
        }
        "echo" => Ok(echo_command(args)),
        "type" => type_command(args),
        "pwd" => {
            let pwd = pwd_command();
            match pwd {
                Ok(s) => Ok(s),
                Err(e) => Err(CommandParseError::ComposableError(e)),
            }
        }
        "cd" => {
            let cd = cd_command(args);
            match cd {
                Ok(_) => Ok(IOStream {
                    stdout: None,
                    stderr: None,
                }),
                Err(e) => Err(CommandParseError::ComposableError(e)),
            }
        }
        _ => {
            let exec = run_binary(cmd, args);
            match exec {
                Ok(s) => Ok(s),
                Err(e) => Err(CommandParseError::ComposableError(e)),
            }
        }
    }
}
