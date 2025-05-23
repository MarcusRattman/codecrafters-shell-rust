use std::io::Error;

#[derive(Debug)]
pub struct IOStream {
    pub stdout: Option<String>,
    pub stderr: Option<String>,
}

impl IOStream {
    pub fn new(stdout: String, stderr: String) -> Self {
        Self {
            stdout: if stdout.is_empty() {
                None
            } else {
                Some(stdout)
            },

            stderr: if stderr.is_empty() {
                None
            } else {
                Some(stderr)
            },
        }
    }

    pub fn print(&self) -> Option<String> {
        if self.stdout.is_some() {
            return self.stdout.clone();
        }

        self.stderr.clone()
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
    StreamError(IOStream),
}

pub const BUILTINS: &[&str] = &["exit", "echo", "type", "pwd", "cd"];
pub const SPECIAL_CHARACTERS: [char; 3] = ['\\', '$', '\"'];
