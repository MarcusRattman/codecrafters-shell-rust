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
        return args.split_whitespace().collect::<Vec<&str>>().join(" ");
    }

    args.chars().for_each(|c| {
        if c.eq(&'\'') {
            enclosed = !enclosed;
        }

        if c.ne(&'\'') {
            result.push(c);
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
    println!("Result: {}", result);
    assert!(result.eq("cock"));
}
