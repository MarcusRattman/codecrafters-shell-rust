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
        "echo" => Ok(format!("{}", args.join(" "))),
        "type" => type_command(args),
        "pwd" => pwd_command(),
        "cd" => cd_command(args),
        _ => run_binary(command, args),
    }
}

fn parse_args(args: &str) -> Vec<String> {
    let args = args.trim();
    let mut result = Vec::<String>::new();

    let mut in_quotes = false;
    let mut current_arg = String::new();

    if !args.contains("'") {
        args.split_whitespace()
            .for_each(|el| result.push(el.to_string()));

        return result;
    }

    args.chars().for_each(|c| {
        if c.eq(&'\'') {
            in_quotes = !in_quotes;
        }

        if c.ne(&'\'') && in_quotes {
            current_arg.push(c);
        }

        if !in_quotes && !current_arg.is_empty() && c.is_whitespace() {
            result.push(current_arg.clone());
            current_arg = String::new();
        }
    });

    result
}

#[test]
fn cat_is_ok() {
    let command = "cat \'./testing/cock.txt\'";
    let result = parse_command(command);
    assert!(result.is_ok());
}

#[test]
fn cat_correct() {
    let command = "cat \'./testing/cock.txt\'";
    let binding = parse_command(command).unwrap();
    let result = binding.trim();
    assert!(result.eq("cock"));
}

#[test]
fn cat_incorrect() {
    let command = "cat ./testing/shit.txt";
    let result = parse_command(command).unwrap();
    assert!(result.is_empty());
}

#[test]
fn cat_four_spaces() {
    let command = "cat \'./testing/cock    23.txt\'";
    let result = parse_command(command).unwrap();
    assert!(result.eq("cock"));
}

#[test]
fn cat_multiple_files() {
    let command = "cat \'./testing/test.txt\' \'./testing/cock.txt\'";
    let result = parse_command(command).unwrap();
    println!("Result {}", result);
    assert!(result.eq("cockcock"));
}
