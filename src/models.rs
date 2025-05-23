use std::io::Error;

pub struct IOStream {
    pub stdout: String,
    pub stderr: String,
}

impl IOStream {
    pub fn new(stdout: String, stderr: String) -> Self {
        Self { stdout, stderr }
    }
}

pub struct Binary {
    pub path: String,
    pub name: String,
}

impl Binary {
    pub fn new(path: String, name: String) -> Self {
        Self { path, name }
    }
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
