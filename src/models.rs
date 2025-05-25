use std::{
    fs::File,
    io::{Error, Write},
    path::Path,
};

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

    pub fn print(&self) -> Option<&String> {
        match (self.stdout.is_some(), self.stderr.is_some()) {
            (true, _) => self.stdout.as_ref(),
            (_, true) => self.stderr.as_ref(),
            _ => None,
        }
    }

    pub fn write_to_file(
        &self,
        path: &str,
        content: &str,
        writemode: &WriteMode,
    ) -> Result<(), Error> {
        let file = match writemode {
            WriteMode::CreateNew => File::create_new(path),
            WriteMode::AppendExisting => {
                let p = Path::new(path);

                if !p.exists() {
                    File::create_new(path)?;
                }

                File::options().append(true).open(path)
            }
        };

        let content = format!("{}\n", content);

        match file {
            Ok(mut f) => f.write_all(content.as_bytes()),
            Err(e) => Err(e),
        }
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

impl std::fmt::Display for CommandParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CommandParseError::ComposableError(ioe) => write!(f, "{}", ioe),
            CommandParseError::CommandNotFound(s) => write!(f, "{}", s),
            _ => write!(f, ""),
        }
    }
}

#[derive(Debug)]
pub enum IOError {
    NoSuchDir(String),
    StdError(Error),
}

pub enum IOStreamType {
    StdErr,
    StdOut,
}

pub enum WriteMode {
    AppendExisting,
    CreateNew,
}

impl std::fmt::Display for IOError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IOError::NoSuchDir(dir) => write!(f, "{}", dir),
            IOError::StdError(e) => write!(f, "{}", e),
        }
    }
}

pub const BUILTINS: &[&str] = &["exit", "echo", "type", "pwd", "cd"];
pub const SPECIAL_CHARACTERS: [char; 3] = ['\\', '$', '\"'];
