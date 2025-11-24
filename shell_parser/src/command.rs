use std::fmt;

/// Description of a command that the parser can validate against.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandSpec {
    pub name: String,
    pub min_args: usize,
    pub max_args: Option<usize>,
}

impl CommandSpec {
    /// Create a new spec with a name and no argument constraints.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            min_args: 0,
            max_args: None,
        }
    }

    /// Require at least `count` arguments.
    pub fn with_min_args(mut self, count: usize) -> Self {
        self.min_args = count;
        self
    }

    /// Restrict to at most `count` arguments.
    pub fn with_max_args(mut self, count: usize) -> Self {
        self.max_args = Some(count);
        self
    }
}

/// Parsed invocation of a command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandInvocation {
    pub name: String,
    pub args: Vec<String>,
    /// Byte offset of the command name in the original input.
    pub position: usize,
}

impl fmt::Display for CommandInvocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {:?}", self.name, self.args)
    }
}

/// Parsed command alongside the separator that followed it (if any).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedCommand {
    pub invocation: CommandInvocation,
    pub separator: Option<crate::separator::Separator>,
}
