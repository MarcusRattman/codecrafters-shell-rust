use crate::commands::*;
use crate::models::{CommandParseError, IOError, SPECIAL_CHARACTERS};
use std::fs;
use std::process::exit;

pub fn parse_command(input: &str) -> Result<String, CommandParseError> {
    let parsed = parse_input(input);

    if parsed.is_empty() {
        return Ok(String::new());
    }

    let mut filename: Option<String> = None;
    let mut parsed_iter = parsed.into_iter();
    let mut left: Vec<String> = vec![];

    while let Some(arg) = parsed_iter.next() {
        match arg.as_str() {
            ">" | "1>" => {
                if let Some(s) = parsed_iter.next() {
                    filename = Some(s);
                    break;
                }
            }
            _ => left.push(arg.to_string()),
        };
    }

    let result = exec_command(left);

    if let Some(fname) = filename {
        let result = result.unwrap();
        let written = write_to_file(fname, result);
        match written {
            Ok(_) => Ok(String::new()),
            Err(e) => Err(CommandParseError::ComposableError(e)),
        }
    } else {
        result
    }
}

fn write_to_file(filename: String, content: String) -> Result<(), IOError> {
    let result = fs::write(filename, content);
    match result {
        Ok(_) => Ok(()),
        Err(_) => Err(IOError::FileWriteError),
    }
}

fn exec_command(mut to_match: Vec<String>) -> Result<String, CommandParseError> {
    let command = to_match.remove(0);
    let args = to_match;

    match command.as_str() {
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
                Ok(s) => Ok(s),
                Err(e) => Err(CommandParseError::ComposableError(e)),
            }
        }
        _ => run_binary(command, args),
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
