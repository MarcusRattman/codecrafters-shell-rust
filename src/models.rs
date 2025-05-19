pub struct Binary {
    pub path: String,
    pub name: String,
}
#[derive(Debug)]
pub struct CommandParseError(pub String);

pub const BUILTINS: &[&str] = &["exit", "echo", "type", "pwd", "cd"];
