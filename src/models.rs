use std::io::Error;

pub struct Binary {
    pub path: String,
    pub name: String,
}
#[derive(Debug)]
pub enum CommandParseError {
    ComposableError(IOError),
    CommandNotFound(String),
    WrongArgsNum,
}
#[derive(Debug)]
pub enum IOError {
    NoSuchDir(String),
    StdError(Error),
}

pub const BUILTINS: &[&str] = &["exit", "echo", "type", "pwd", "cd"];
pub const SPECIAL_CHARACTERS: [char; 3] = ['\\', '$', '\"'];
