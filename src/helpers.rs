use crate::commands::*;
use crate::models::{CommandParseError, SPECIAL_CHARACTERS};
use std::fs::File;
use std::io::{self, Write};
use std::process::exit;

// Updated parse_command to handle output redirection
pub fn parse_command(input: &str) -> Result<String, CommandParseError> {
    // First, parse the input into tokens
    let mut tokens = parse_input(input);

    if tokens.is_empty() {
        return Ok(String::new());
    }

    // Check for output redirection '>' operator
    let mut output_file: Option<String> = None;
    let mut command_tokens = Vec::new();
    let mut tokens_iter = tokens.into_iter();

    while let Some(token) = tokens_iter.next() {
        if token == ">" {
            // Expect the next token to be the filename
            if let Some(filename) = tokens_iter.next() {
                output_file = Some(filename);
            } else {
                // No filename provided after '>'
                return Err(CommandParseError(
                    "No filename specified for output redirection".to_string(),
                ));
            }
        } else {
            command_tokens.push(token);
        }
    }

    if command_tokens.is_empty() {
        return Ok(String::new());
    }

    // Separate command and args
    let command = command_tokens.remove(0);
    let args = command_tokens;

    // Prepare output redirection if needed
    if let Some(filename) = output_file {
        // Capture the output of command execution
        let result = match command.as_str() {
            "exit" => {
                let code: i32 = args.get(0).and_then(|s| s.parse().ok()).unwrap_or(-1);
                // Exit the process
                exit(code);
            }
            "echo" => Ok(echo_command(args)),
            "type" => type_command(args),
            "pwd" => pwd_command(),
            "cd" => cd_command(args),
            _ => run_binary(&command, args),
        };

        // Write the result to the specified file
        match result {
            Ok(output_str) => {
                let mut file =
                    File::create(&filename).map_err(|e| CommandParseError(e.to_string()))?;
                file.write_all(output_str.as_bytes())
                    .map_err(|e| CommandParseError(e.to_string()))?;
                Ok(String::new())
            }
            Err(e) => Err(e),
        }
    } else {
        // No redirection, just run command normally
        match command.as_str() {
            "exit" => {
                let code: i32 = args.get(0).and_then(|s| s.parse().ok()).unwrap_or(-1);
                exit(code);
            }
            "echo" => Ok(echo_command(args)),
            "type" => type_command(args),
            "pwd" => pwd_command(),
            "cd" => cd_command(args),
            _ => run_binary(&command, args),
        }
    }
}

// Your existing parse_input function remains unchanged
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
