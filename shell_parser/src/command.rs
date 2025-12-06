use std::fmt;

/// Description of a command that the parser can validate against.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandSpec {
    pub name: String,
    pub about: String,
    pub min_args: usize,
    pub max_args: Option<usize>,
    pub aliases: Vec<String>,
}

impl CommandSpec {
    /// Create a new spec with a name and no argument constraints.
    pub fn new(name: impl Into<String>, about: impl Into<String>) -> Self {
        Self {
            about: about.into(),
            name: name.into(),
            min_args: 0,
            max_args: None,
            aliases: Vec::new(),
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

    /// Register a single alias that should map to this command.
    pub fn with_alias(mut self, alias: impl Into<String>) -> Self {
        self.aliases.push(alias.into());
        self
    }

    /// Register multiple aliases that should map to this command.
    pub fn with_aliases<I, S>(mut self, aliases: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.aliases
            .extend(aliases.into_iter().map(|alias| alias.into()));
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
