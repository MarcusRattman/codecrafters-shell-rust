use std::{
    fs::File,
    io::{Error, Write},
    path::Path,
};

#[derive(Debug)]
pub struct IOStream {
    stdout: Option<String>,
    stderr: Option<String>,
}

impl IOStream {
    pub fn new(stdout: &str, stderr: &str) -> Self {
        Self {
            stdout: if stdout.is_empty() {
                None
            } else {
                Some(stdout.to_string())
            },

            stderr: if stderr.is_empty() {
                None
            } else {
                Some(stderr.to_string())
            },
        }
    }

    pub fn from_options(stdout: Option<String>, stderr: Option<String>) -> Self {
        Self { stdout, stderr }
    }

    pub fn new_empty() -> Self {
        Self {
            stdout: None,
            stderr: None,
        }
    }

    pub fn get_stdout(&self) -> Option<String> {
        self.stdout.clone()
    }

    pub fn get_stderr(&self) -> Option<String> {
        self.stderr.clone()
    }

    pub fn print(&self) -> Option<&String> {
        self.stdout.as_ref().or(self.stderr.as_ref())
    }

    pub fn write_to_file(
        &self,
        path: &str,
        stream_type: &IOStreamType,
        writemode: &WriteMode,
    ) -> Result<(), Error> {
        let mut created = false;

        let content = match stream_type {
            IOStreamType::StdOut => self.stdout.as_deref().unwrap_or_default(),
            IOStreamType::StdErr => self.stderr.as_deref().unwrap_or_default(),
        };

        let mut file = match writemode {
            WriteMode::CreateNew => {
                let f = File::create_new(path)?;
                created = true;
                f
            }
            WriteMode::AppendExisting => {
                let p = Path::new(path);

                if !p.exists() {
                    File::create_new(path)?;
                    created = true;
                }

                File::options().append(true).open(path)?
            }
        };

        let content = if created {
            content.to_string()
        } else {
            format!("\n{}", content)
        };

        file.write_all(content.as_bytes())
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
pub const SPECIAL_CHARACTERS: &[char] = &['\\', '$', '\"'];
