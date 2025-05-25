use crate::commands::*;
use crate::models::{
    CommandParseError, IOError, IOStream, IOStreamType, WriteMode, SPECIAL_CHARACTERS,
};
use std::io::Error;
use std::process::exit;

pub fn parse_command(input: &str) -> Result<IOStream, CommandParseError> {
    let parsed = parse_input(input);
    if parsed.is_empty() {
        return Ok(IOStream::new(String::new(), String::new()));
    }

    let mut filename: Option<String> = None;
    let mut parsed_iter = parsed.into_iter();
    let mut left: Vec<String> = vec![];
    let mut stream_to_write = IOStreamType::StdOut;
    let mut writemode = WriteMode::CreateNew;

    while let Some(arg) = parsed_iter.next() {
        match arg.as_str() {
            ">" | "1>" => {
                if let Some(s) = parsed_iter.next() {
                    filename = Some(s);
                    stream_to_write = IOStreamType::StdOut;
                    writemode = WriteMode::CreateNew;
                    break;
                }
            }
            "2>" => {
                if let Some(s) = parsed_iter.next() {
                    filename = Some(s);
                    stream_to_write = IOStreamType::StdErr;
                    writemode = WriteMode::CreateNew;
                    break;
                }
            }
            "1>>" | ">>" => {
                if let Some(s) = parsed_iter.next() {
                    filename = Some(s);
                    stream_to_write = IOStreamType::StdOut;
                    writemode = WriteMode::AppendExisting;
                }
            }
            "2>>" => {
                if let Some(s) = parsed_iter.next() {
                    filename = Some(s);
                    stream_to_write = IOStreamType::StdErr;
                    writemode = WriteMode::AppendExisting;
                }
            }
            _ => left.push(arg.to_string()),
        };
    }

    let result = exec_command(left);

    if let Some(fname) = filename {
        if let Err(e) = result {
            return Err(e);
        }
        let result = result.unwrap();

        let written = write_to_file(fname, &result, &stream_to_write, &writemode);
        // Shitshow
        if let Err(e) = written {
            return Err(CommandParseError::ComposableError(IOError::StdError(e)));
        }

        let result = match stream_to_write {
            IOStreamType::StdOut => IOStream {
                stdout: None,
                stderr: result.stderr,
            },
            IOStreamType::StdErr => IOStream {
                stdout: result.stdout,
                stderr: None,
            },
        };

        return Ok(result);
    }
    result
}

fn write_to_file(
    filename: String,
    stream: &IOStream,
    stream_type: &IOStreamType,
    writemode: &WriteMode,
) -> Result<(), Error> {
    let stdout = stream.stdout.clone();
    let stderr = stream.stderr.clone();

    let text = match stream_type {
        IOStreamType::StdOut => {
            if stdout.is_some() {
                stdout.unwrap()
            } else {
                String::new()
            }
        }
        IOStreamType::StdErr => {
            if stderr.is_some() {
                stderr.unwrap()
            } else {
                String::new()
            }
        }
    };

    stream.write_to_file(&filename, &text, writemode)?;

    Ok(())
}

fn exec_command(mut command: Vec<String>) -> Result<IOStream, CommandParseError> {
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

fn parse_input(args: &str) -> Vec<String> {
    let args = args.trim();
    let mut result = Vec::<String>::new();

    let mut in_single_quotes = false;
    let mut in_double_quotes = false;
    let mut current_arg = String::new();

    let mut chars = args.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '\\' => {
                if let Some(&next_char) = chars.peek() {
                    if !in_single_quotes
                        && (SPECIAL_CHARACTERS.contains(&next_char) || !in_double_quotes)
                    {
                        chars.next();
                        current_arg.push(next_char);
                    } else {
                        current_arg.push(c);
                    }
                }
            }
            '\'' => {
                if !in_double_quotes {
                    in_single_quotes = !in_single_quotes;
                } else {
                    current_arg.push(c);
                }
            }
            '\"' => {
                if !in_single_quotes {
                    in_double_quotes = !in_double_quotes;
                } else {
                    current_arg.push(c);
                }
            }
            c if c.is_whitespace() => {
                if in_single_quotes || in_double_quotes {
                    current_arg.push(c);
                } else if !current_arg.is_empty() {
                    result.push(current_arg.clone());
                    current_arg.clear();
                }
            }
            _ => {
                current_arg.push(c);
            }
        }
    }

    if !current_arg.is_empty() {
        result.push(current_arg);
    }

    result
}
