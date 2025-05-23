pub struct Binary {
    pub path: String,
    pub name: String,
}
#[derive(Debug)]
pub enum CommandParseError {
    ComposableError(IOError),
    CommandNotFound(String),
    BinExecError,
    WrongArgsNum,
}
#[derive(Debug)]
pub enum IOError {
    NoSuchDir(String),
    FileWriteError,
}

pub const BUILTINS: &[&str] = &["exit", "echo", "type", "pwd", "cd"];
pub const SPECIAL_CHARACTERS: [char; 3] = ['\\', '$', '\"'];
