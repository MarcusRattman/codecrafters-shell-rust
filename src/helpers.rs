use crate::{commands::*, models::CommandParseError};
use std::process::exit;

pub fn parse_command(input: &str) -> Result<String, CommandParseError> {
    let mut parts = input.trim().splitn(2, ' ');
    let command = parts.next().unwrap_or("").trim();
    let args = parts.next().unwrap_or("").trim();
    let args = &parse_args(args);

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

fn parse_args(args: &str) -> String {
    let args = args.trim();
    let mut result = String::new();
    let mut enclosed = false;

    if !args.contains("'") {
        return args.split_whitespace().collect();
    }

    args.chars().for_each(|c| {
        if c.eq(&'\'') {
            enclosed = !enclosed;
        }

        if c.ne(&'\'') && enclosed {
            result.push(c);
        }
    });

    result
}
