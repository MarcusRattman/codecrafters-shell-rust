use super::helpers::{exec_command, write_to_file};
use super::models::{
    CommandParseError, IOError, IOStream, IOStreamType, WriteMode, SPECIAL_CHARACTERS,
};

pub fn parse_input(input: &str) -> Result<IOStream, CommandParseError> {
    let parsed = parse_chars(input);
    if parsed.is_empty() {
        return Ok(IOStream::default());
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
                }
            }
            "2>" => {
                if let Some(s) = parsed_iter.next() {
                    filename = Some(s);
                    stream_to_write = IOStreamType::StdErr;
                    writemode = WriteMode::CreateNew;
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
        let result = result?;
        let result = write_to_file(result, &stream_to_write, &fname, writemode);

        if let Err(e) = result {
            return Err(CommandParseError::ComposableError(IOError::StdError(e)));
        }

        return Ok(result.unwrap());
    }
    result
}

fn parse_chars(args: &str) -> Vec<String> {
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
