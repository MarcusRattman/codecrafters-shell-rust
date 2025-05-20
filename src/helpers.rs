use crate::{commands::*, models::CommandParseError};
use std::process::exit;

pub fn parse_command(input: &str) -> Result<String, CommandParseError> {
    let mut parts = input.trim().splitn(2, ' ');
    let command = parts.next().unwrap_or("").trim();
    let args = parts.next().unwrap_or("").trim();
    let args = parse_args(args);

    match command {
        "exit" => {
            let code: i32 = args[0].parse().unwrap_or(-1);
            exit(code);
        }
        "echo" => Ok(command_echo(args)),
        "type" => type_command(args),
        "pwd" => pwd_command(),
        "cd" => cd_command(args),
        _ => run_binary(command, args),
    }
}

fn parse_args(args: &str) -> Vec<String> {
    let args = args.trim();
    let mut result = Vec::<String>::new();

    let mut in_single_quotes = false;
    let mut in_double_quotes = false;
    let mut current_arg = String::new();

    let mut chars = args.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '\\' => {
                if !in_single_quotes || !in_double_quotes {
                    current_arg.push(c);
                    continue;
                }

                if let Some(&next_char) = chars.peek() {
                    chars.next();
                    current_arg.push(next_char);
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
                } else {
                    if !current_arg.is_empty() {
                        result.push(current_arg.clone());
                        current_arg.clear();
                    }
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
