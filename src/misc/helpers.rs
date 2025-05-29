use super::commands::{cd_command, echo_command, pwd_command, run_binary, type_command};
use super::models::{CommandParseError, IOStream, IOStreamType, WriteMode};
use crossterm::terminal::disable_raw_mode;

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
        IOStreamType::StdOut => IOStream::from_options(None, iostream.get_stderr()),
        IOStreamType::StdErr => IOStream::from_options(iostream.get_stdout(), None),
    };

    Ok(result)
}

pub fn exec_command(mut command: Vec<String>) -> Result<IOStream, CommandParseError> {
    let cmd = command.remove(0);
    let args = command;

    match cmd.as_str() {
        "exit" => {
            let code: i32 = args
                .first()
                .unwrap_or(&String::from("-1"))
                .parse()
                .unwrap_or(-1);
            disable_raw_mode().unwrap();
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
                Ok(_) => Ok(IOStream::default()),
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
